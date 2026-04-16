//! 斗地主游戏状态机。
//!
//! 该模块提供纯逻辑游戏引擎，适合用于局域网联机场景。
//!
//! 其职责仅包括：
//! - 管理游戏阶段（抢地主、出牌、游戏结束）
//! - 接收外部动作并更新游戏状态
//! - 提供可序列化的状态快照
//!
//! 不包含终端输入/输出和网络协议实现，适合在服务端或客户端代码中复用。


use crate::card::cards::Cards;
use crate::card::Rank;
use crate::card::Suit;
use std::fmt;
use crate::error::{PlayError, PlayerError};
use crate::game::dealer::InitialDeal;
use crate::player::{Player, PlayerType, Role};
use crate::rules::Play;

/// 游戏阶段。
///
/// 用于表示当前正在进行的游戏流程。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    /// 抢地主阶段，记录起始玩家、过牌计数和最后抢地主人。
    Bidding {
        start_player: usize,
        pass_streak: usize,
        last_bidder: Option<usize>,
    },
    /// 出牌阶段
    Playing,
    /// 游戏结束
    Finished { winner: usize },
}

/// 游戏状态机错误类型。
///
/// 该错误类型用于表示 `_game` 模块内部的状态机失败原因。
#[derive(Debug, Clone)]
pub enum GameError {
    /// 玩家编号无效。
    InvalidPlayerId(usize),
    /// 操作用于错误的游戏阶段。
    WrongPhase {
        expected: &'static str,
        actual: GamePhase,
    },
    /// 动作不被允许，例如非当前玩家执行出牌。
    ActionNotAllowed(String),
    /// 出牌验证失败。
    PlayError(PlayError),
    /// 玩家操作相关错误。
    PlayerError(PlayerError),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidPlayerId(id) => write!(f, "无效玩家编号: {}", id),
            GameError::WrongPhase { expected, actual } => write!(
                f,
                "错误阶段: 期望 {}，当前 {:?}",
                expected,
                actual
            ),
            GameError::ActionNotAllowed(reason) => write!(f, "动作不被允许: {}", reason),
            GameError::PlayError(err) => write!(f, "出牌错误: {}", err),
            GameError::PlayerError(err) => write!(f, "玩家错误: {}", err),
        }
    }
}

impl std::error::Error for GameError {}

impl From<PlayError> for GameError {
    fn from(err: PlayError) -> Self {
        GameError::PlayError(err)
    }
}

impl From<PlayerError> for GameError {
    fn from(err: PlayerError) -> Self {
        GameError::PlayerError(err)
    }
}

pub type GameResult<T> = std::result::Result<T, GameError>;

/// 游戏动作，由外层调用者发起。
///
/// 该枚举表示不同的玩家操作，供状态机执行。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameAction {
    /// 抢地主动作，`bid` 为 `true` 表示抢地主，`false` 表示过牌。
    Bid { player_id: usize, bid: bool },
    /// 出牌动作，直接传入合法的 `Play`。
    Play { player_id: usize, play: Play },
}

/// 状态机动作执行结果。
///
/// 表示一次动作执行后的状态变更结果，用于通知调用者下一步行为。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameActionResult {
    /// 抢地主阶段继续，轮到下一个玩家。
    BiddingContinues { next_player: usize },
    /// 抢地主阶段结束并确定地主。
    BiddingEnded { landlord: usize },
    /// 出牌成功，轮到下一个玩家。
    PlayAccepted { next_player: usize },
    /// 游戏结束，返回胜利玩家。
    GameOver { winner: usize },
}

/// 玩家当前状态的可序列化视图。
///
/// 该结构体用于在客户端或网络层中展示玩家信息，
/// 包含手牌、身份和玩家类型。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerState {
    pub id: usize,
    pub role: Role,
    pub hand: Cards,
    pub player_type: PlayerType,
}

/// 当前游戏状态快照，用于客户端显示或网络传输。
///
/// 该结构体表示整个游戏的当前状态，可以安全地序列化并发送给远端。
#[derive(Debug, Clone)]
pub struct GameState {
    pub phase: GamePhase,
    pub current_player: usize,
    pub last_player: usize,
    pub last_played_cards: Play,
    pub landlord_cards: Cards,
    pub players: [PlayerState; 3],
}

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
            Player::new(0, deal.player_hands[0].clone(), Role::Farmer, player_types[0]),
            Player::new(1, deal.player_hands[1].clone(), Role::Farmer, player_types[1]),
            Player::new(2, deal.player_hands[2].clone(), Role::Farmer, player_types[2]),
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
    pub fn game_state(&self) -> GameState {
        let players = self
            .players
            .iter()
            .map(|p| PlayerState {
                id: p.id,
                role: p.role,
                hand: p.hand.clone(),
                player_type: p.player_type,
            })
            .collect::<Vec<_>>();

        GameState {
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
                return Ok(GameActionResult::BiddingEnded { landlord: landlord_index });
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

            self.players[player_id].play_cards(&play.cards).map_err(GameError::PlayerError)?;
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
