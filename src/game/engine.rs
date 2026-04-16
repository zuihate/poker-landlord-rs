use crate::card::{Cards, Rank, Suit};
use crate::error::GameError;
use crate::game::dealer::InitialDeal;
use crate::game::types::{GameAction, GameActionResult, GamePhase, GameResult};
use crate::player::{Player, PlayerType, Role};
use crate::rules::Play;

/// 新的游戏状态机实现。
///
/// 该类型负责管理斗地主游戏的所有内部状态，并对外暴露纯逻辑接口。
pub struct Game {
    players: [Player; 3],
    landlord_cards: Cards,
    current_player: usize,
    last_player: usize,
    last_played_cards: Play,
    phase: GamePhase,
}

impl Game {
    /// 创建新游戏，默认 3 个真人玩家。
    ///
    /// 游戏初始化后直接进入抢地主阶段。
    pub fn new() -> Self {
        Self::new_with_player_types([PlayerType::Human; 3])
    }

    /// 创建新游戏并指定每个玩家类型。
    ///
    /// `player_types` 可用于区分真人和 AI。
    pub fn new_with_player_types(player_types: [PlayerType; 3]) -> Self {
        let deal = InitialDeal::new();
        Self::from_deal_and_types(deal, player_types)
    }

    /// 从给定发牌结果构建游戏状态。
    ///
    /// 该方法允许外部测试或复用已有发牌逻辑。
    pub fn from_deal(deal: InitialDeal) -> Self {
        Self::from_deal_and_types(deal, [PlayerType::Human; 3])
    }

    fn from_deal_and_types(deal: InitialDeal, player_types: [PlayerType; 3]) -> Self {
        let players = [
            Player::new(
                0,
                deal.player_hands[0].clone(),
                Role::Farmer,
                player_types[0],
            ),
            Player::new(
                1,
                deal.player_hands[1].clone(),
                Role::Farmer,
                player_types[1],
            ),
            Player::new(
                2,
                deal.player_hands[2].clone(),
                Role::Farmer,
                player_types[2],
            ),
        ];

        let start_player = players
            .iter()
            .position(|p| {
                p.hand
                    .iter()
                    .any(|c| c.rank == Rank::Three && c.suit == Some(Suit::Diamonds))
            })
            .unwrap_or(0);

        let last_played_cards = Play::new(Cards::new()).unwrap();

        Self {
            players,
            landlord_cards: deal.landlord_cards,
            current_player: start_player,
            last_player: start_player,
            last_played_cards,
            phase: GamePhase::Bidding {
                start_player,
                pass_streak: 0,
                last_bidder: None,
            },
        }
    }

    /// 当前轮到的玩家索引
    pub fn current_player(&self) -> usize {
        self.current_player
    }

    /// 当前游戏阶段
    pub fn phase(&self) -> GamePhase {
        self.phase
    }

    /// 是否已经结束
    pub fn is_finished(&self) -> bool {
        matches!(self.phase, GamePhase::Finished { .. })
    }

    /// 若游戏结束，返回胜利玩家
    pub fn winner(&self) -> Option<usize> {
        match self.phase {
            GamePhase::Finished { winner } => Some(winner),
            _ => None,
        }
    }

    /// 返回当前游戏状态快照。
    ///
    /// 该快照可用于客户端渲染或网络同步。
    pub fn game_state(&self) -> crate::game::types::GameState {
        let players = self
            .players
            .iter()
            .map(|p| crate::game::types::PlayerState {
                id: p.id,
                role: p.role,
                hand: p.hand.clone(),
                player_type: p.player_type,
            })
            .collect::<Vec<_>>();

        crate::game::types::GameState {
            phase: self.phase,
            current_player: self.current_player,
            last_player: self.last_player,
            last_played_cards: self.last_played_cards.clone(),
            landlord_cards: self.landlord_cards.clone(),
            players: [players[0].clone(), players[1].clone(), players[2].clone()],
        }
    }

    /// 设置玩家类型。
    ///
    /// 该方法允许在游戏运行时切换玩家的类型，例如将某个玩家改为 AI。
    pub fn set_player_type(&mut self, player_id: usize, player_type: PlayerType) -> GameResult<()> {
        if player_id >= self.players.len() {
            return Err(GameError::InvalidPlayerId(player_id));
        }
        self.players[player_id].player_type = player_type;
        Ok(())
    }

    /// 执行动作并更新状态机。
    ///
    /// 外层调用者应根据返回值继续驱动后续流程。
    pub fn apply_action(&mut self, action: GameAction) -> GameResult<GameActionResult> {
        match action {
            GameAction::Bid { player_id, bid } => self.apply_bid(player_id, bid),
            GameAction::Play { player_id, play } => self.apply_play(player_id, play),
        }
    }

    fn next_player(&self) -> usize {
        (self.current_player + 1) % 3
    }

    fn apply_bid(&mut self, player_id: usize, bid: bool) -> GameResult<GameActionResult> {
        if let GamePhase::Bidding {
            start_player,
            pass_streak,
            last_bidder,
        } = &mut self.phase
        {
            if player_id != self.current_player {
                return Err(GameError::ActionNotAllowed(
                    "当前不是该玩家出牌回合".to_string(),
                ));
            }

            if bid {
                *last_bidder = Some(player_id);
                *pass_streak = 0;
            } else {
                *pass_streak += 1;
            }

            if *pass_streak >= 2 {
                let landlord_index = last_bidder.unwrap_or(*start_player);
                self.players[landlord_index].become_landlord(&self.landlord_cards);
                self.current_player = landlord_index;
                self.last_player = landlord_index;
                self.phase = GamePhase::Playing;
                return Ok(GameActionResult::BiddingEnded {
                    landlord: landlord_index,
                });
            }

            self.current_player = self.next_player();
            Ok(GameActionResult::BiddingContinues {
                next_player: self.current_player,
            })
        } else {
            Err(GameError::ActionNotAllowed(
                "当前阶段不是抢地主阶段".to_string(),
            ))
        }
    }

    fn apply_play(&mut self, player_id: usize, play: Play) -> GameResult<GameActionResult> {
        if let GamePhase::Playing = self.phase {
            if player_id != self.current_player {
                return Err(GameError::ActionNotAllowed(
                    "当前不是该玩家出牌回合".to_string(),
                ));
            }

            if !play.can_beat(&self.last_played_cards) {
                return Err(GameError::ActionNotAllowed(
                    "出牌不能压过上一手牌".to_string(),
                ));
            }

            self.players[player_id]
                .play_cards(&play.cards)
                .map_err(GameError::Player)?;
            self.last_played_cards = play;
            self.last_player = self.current_player;

            if self.players[player_id].hand.is_empty() {
                self.phase = GamePhase::Finished { winner: player_id };
                return Ok(GameActionResult::GameOver { winner: player_id });
            }

            self.current_player = self.next_player();
            Ok(GameActionResult::PlayAccepted {
                next_player: self.current_player,
            })
        } else {
            Err(GameError::ActionNotAllowed(
                "当前阶段不是出牌阶段".to_string(),
            ))
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
