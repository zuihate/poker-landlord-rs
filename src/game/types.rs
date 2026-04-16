use crate::card::cards::Cards;
use crate::error::GameError;
use crate::player::{PlayerType, Role};
use crate::rules::Play;

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

/// 游戏结果类型别名。
///
/// 统一使用全局 `GameError` 作为游戏逻辑失败返回值。
pub type GameResult<T> = std::result::Result<T, GameError>;
