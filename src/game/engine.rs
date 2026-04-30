use crate::card::{Cards, Rank, Suit};
use crate::error::GameError;
use crate::game::dealer::InitialDeal;
use crate::game::types::{GameAction, GameActionResult, GamePhase, GameResult};
use crate::player::{Player, PlayerType, Role};
use crate::rules::Play;

/// 斗地主游戏状态机的核心实现
///
/// `Game` 是纯逻辑的游戏引擎，负责管理游戏的所有内部状态和流程转换。
/// 它不包含任何终端交互或网络通信代码，适合在服务端或本地客户端中复用。n///
/// # 游戏流程
///
/// 1. **抢地主阶段** (`Bidding`) - 三个玩家依次决定是否抢地主
/// 2. **出牌阶段** (`Playing`) - 地主先出牌，其他玩家依次跟牌或过牌
/// 3. **游戏结束** (`Finished`) - 当某个玩家的手牌为空时结束
///
/// # 使用示例
///
/// ```ignore
/// use poker_landlord_rs::game::Game;
/// use poker_landlord_rs::game::GameAction;
///
/// let mut game = Game::new();
///
/// // 抢地主阶段
/// while !game.is_finished() {
///     if matches!(game.phase(), GamePhase::Bidding { .. }) {
///         let action = GameAction::Bid { player_id: game.current_player(), bid: true };
///         let result = game.apply_action(action)?;
///     } else {
///         break; // 进入出牌阶段
///     }
/// }
/// ```
///
/// # 内部状态
///
/// - `players` - 三个玩家的对象数组，包含手牌和角色信息
/// - `landlord_cards` - 底牌（地主获胜后将获得的3张牌）
/// - `current_player` - 当前轮到的玩家编号（0、1、2）
/// - `last_player` - 上一个出牌的玩家编号
/// - `last_played_cards` - 上一次出的牌，用于检查当前出牌是否能压过
/// - `phase` - 游戏当前阶段（抢地主/出牌/结束）
pub struct Game {
    /// 三个玩家
    players: [Player; 3],
    /// 底牌（3张），未处理的扑克
    landlord_cards: Cards,
    /// 当前轮到的玩家索引（0-2）
    current_player: usize,
    /// 上一个出牌的玩家索引（用于记录谁出的最后一手牌）
    last_player: usize,
    /// 上一手出过的牌（用于判断当前出牌是否能压过）
    last_played_cards: Play,
    /// 当前游戏阶段
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

    /// 计算下一个玩家的索引
    ///
    /// # 返回值
    ///
    /// 下一个玩家的索引（0、1、2 之间循环）
    fn next_player(&self) -> usize {
        (self.current_player + 1) % 3
    }

    /// 处理抢地主动作
    ///
    /// 该方法处理玩家在抢地主阶段的决策：
    /// - 如果 `bid=true`，玩家选择抢地主
    /// - 如果 `bid=false`，玩家选择过牌
    ///
    /// 当连续有2个玩家过牌时，抢地主阶段结束，进入出牌阶段。
    /// 若没有任何玩家抢地主，则起始玩家自动成为地主。
    ///
    /// # 参数
    ///
    /// - `player_id` - 执行抢地主动作的玩家编号
    /// - `bid` - `true` 表示抢地主，`false` 表示过牌
    ///
    /// # 返回值
    ///
    /// 返回该动作的执行结果：
    /// - `BiddingContinues` - 抢地主阶段继续，轮到下一个玩家
    /// - `BiddingEnded` - 确定地主，进入出牌阶段
    ///
    /// # 错误
    ///
    /// 如果 `player_id` 不等于 `current_player`，或当前不在抢地主阶段，会返回错误
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

    /// 处理出牌动作
    ///
    /// 该方法验证玩家的出牌是否合法，并更新游戏状态。
    /// 验证包括：
    /// - 确认轮到该玩家出牌
    /// - 确认出的牌能压过上一手牌（或是首次出牌）
    /// - 从玩家手牌中移除已出的牌
    ///
    /// 如果玩家出牌后手牌为空，游戏结束，该玩家胜利。
    ///
    /// # 参数
    ///
    /// - `player_id` - 执行出牌动作的玩家编号
    /// - `play` - 玩家要出的牌（包含卡牌和牌型分类）
    ///
    /// # 返回值
    ///
    /// 返回该动作的执行结果：
    /// - `PlayAccepted` - 出牌成功，轮到下一个玩家
    /// - `GameOver` - 出牌后玩家手牌为空，游戏结束
    ///
    /// # 错误
    ///
    /// 如果以下条件之一成立，会返回错误：
    /// - `player_id` 不等于 `current_player`（不是该玩家回合）
    /// - 出的牌无法压过上一手牌
    /// - 玩家手中不含此牌
    /// - 当前不在出牌阶段
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
