use crate::card::cards::Cards;
use crate::error::GameError;
use crate::player::{PlayerType, Role};
use crate::rules::Play;

/// 玩家可以发起的游戏动作
///
/// 该枚举表示玩家在游戏过程中能够发起的所有合法操作。
/// 这些动作会被传递给 `Game::apply_action()` 进行处理。
///
/// # 动作类型
///
/// - `Bid` - 在抢地主阶段，玩家选择是否抢地主
/// - `Play` - 在出牌阶段，玩家出牌或选择不出
///
/// # 示例
///
/// ```ignore
/// use poker_landlord_rs::game::GameAction;
///
/// // 抢地主动作
/// let bid_action = GameAction::Bid { player_id: 0, bid: true };
///
/// // 过牌动作
/// let pass_action = GameAction::Bid { player_id: 1, bid: false };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameAction {
    /// 抢地主动作
    ///
    /// # 参数
    /// - `player_id` - 执行抢地主的玩家编号（0-2）
    /// - `bid` - `true` 表示抢地主，`false` 表示过牌
    Bid { player_id: usize, bid: bool },

    /// 出牌动作
    ///
    /// # 参数
    /// - `player_id` - 执行出牌的玩家编号（0-2）
    /// - `play` - 已验证的合法出牌（包含卡牌和牌型分类）
    Play { player_id: usize, play: Play },
}

/// 动作执行后的结果反馈
///
/// 该枚举用于通知外部调用者一个动作执行的结果以及后续的游戏状态变更。
/// 外层应根据返回的结果类型决定下一步的操作流程。
///
/// # 结果类型
///
/// - `BiddingContinues` - 抢地主阶段继续
/// - `BiddingEnded` - 抢地主阶段结束
/// - `PlayAccepted` - 出牌成功
/// - `GameOver` - 游戏结束
///
/// # 示例
///
/// ```ignore
/// match game.apply_action(action)? {
///     GameActionResult::BiddingContinues { next_player } => {
///         println!("轮到玩家{}抢地主", next_player);
///     },
///     GameActionResult::BiddingEnded { landlord } => {
///         println!("玩家{}成为地主", landlord);
///     },
///     GameActionResult::PlayAccepted { next_player } => {
///         println!("出牌成功，轮到玩家{}", next_player);
///     },
///     GameActionResult::GameOver { winner } => {
///         println!("玩家{}获胜！", winner);
///     },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameActionResult {
    /// 抢地主阶段继续
    ///
    /// 当前动作后继续进行抢地主，轮到下一个玩家决策。
    /// 这通常意味着玩家选择过牌，或者还没有足够的玩家过牌。
    BiddingContinues {
        /// 下一个轮到的玩家编号
        next_player: usize,
    },

    /// 抢地主阶段结束
    ///
    /// 确定了地主，游戏进入出牌阶段。
    /// 地主将获得底牌，并首先出牌。
    BiddingEnded {
        /// 成为地主的玩家编号
        landlord: usize,
    },

    /// 出牌成功
    ///
    /// 当前玩家的出牌被接受，轮到下一个玩家。
    PlayAccepted {
        /// 下一个轮到的玩家编号
        next_player: usize,
    },

    /// 游戏结束
    ///
    /// 某个玩家的手牌已清空，游戏结束，该玩家获胜。
    GameOver {
        /// 游戏胜利者的玩家编号
        winner: usize,
    },
}

/// 游戏的当前阶段
///
/// 表示整个游戏过程中的不同阶段，驱动游戏流程向前推进。
///
/// # 阶段说明
///
/// 游戏按顺序经历以下阶段：
/// 1. **Bidding** - 抢地主阶段
/// 2. **Playing** - 出牌阶段  
/// 3. **Finished** - 游戏结束
///
/// # 示例
///
/// ```ignore
/// match game.phase() {
///     GamePhase::Bidding { .. } => println!("正在进行抢地主"),
///     GamePhase::Playing => println!("正在进行出牌"),
///     GamePhase::Finished { winner } => println!("游戏结束，胜者是玩家{}", winner),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamePhase {
    /// 抢地主阶段
    ///
    /// 三个玩家依次决定是否抢地主。
    /// 当某个玩家抢地主后，其他两个玩家有机会抢地主。
    /// 如果连续两个玩家过牌，抢地主阶段结束。
    Bidding {
        /// 最开始轮到的玩家编号（通常是持有黑桃3的玩家）
        start_player: usize,
        /// 连续过牌的次数，达到2次时阶段结束
        pass_streak: usize,
        /// 最后一个选择抢地主的玩家编号（如果没人抢则为 None）
        last_bidder: Option<usize>,
    },

    /// 出牌阶段
    ///
    /// 地主先出牌，其他玩家依次跟牌或过牌。
    /// 当某个玩家的手牌为空时，游戏结束。
    Playing,

    /// 游戏结束
    ///
    /// 某个玩家已清空手牌，游戏结束。
    Finished {
        /// 游戏获胜者的玩家编号
        winner: usize,
    },
}

/// 玩家状态的可序列化快照
///
/// 该结构体用于向客户端或远端展示玩家的当前状态。
/// 包含了玩家的身份、角色、手牌数量等所有必要的游戏信息。
///
/// # 字段说明
///
/// - `id` - 玩家的唯一编号（0-2）
/// - `role` - 玩家的角色（地主或农民）
/// - `hand` - 玩家当前的手牌
/// - `player_type` - 玩家类型（真人或 AI）
///
/// # 用途
///
/// 该结构体通常作为 `GameState` 的一部分，
/// 用于网络传输、状态保存或客户端渲染。
///
/// # 示例
///
/// ```ignore
/// let state = game.game_state();
/// for player_state in &state.players {
///     println!("玩家{}: {} {}", player_state.id, player_state.role, player_state.player_type);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerState {
    /// 玩家编号
    pub id: usize,
    /// 玩家当前的角色
    pub role: Role,
    /// 玩家当前的手牌
    pub hand: Cards,
    /// 玩家类型
    pub player_type: PlayerType,
}

/// 游戏状态的完整快照
///
/// 该结构体表示游戏在某一时刻的完整状态。
/// 它可以被序列化、网络传输、保存等，用于状态同步和恢复。
///
/// # 字段说明
///
/// - `phase` - 当前游戏阶段
/// - `current_player` - 当前轮到的玩家编号
/// - `last_player` - 上一个出牌的玩家编号
/// - `last_played_cards` - 上一手出的牌
/// - `landlord_cards` - 地主的底牌（或已获得的底牌）
/// - `players` - 三个玩家的状态快照
///
/// # 用途
///
/// - **网络同步** - 将游戏状态发送给远端客户端
/// - **状态保存** - 保存游戏进度用于恢复
/// - **客户端渲染** - 客户端根据该状态更新显示
/// - **日志记录** - 记录游戏进程用于回放或分析
///
/// # 示例
///
/// ```ignore
/// let state = game.game_state();
/// println!("当前阶段: {:?}", state.phase);
/// println!("当前玩家: {}", state.current_player);
/// for player in &state.players {
///     println!("  玩家{}: {} 张牌", player.id, player.hand.len());
/// }
/// ```
#[derive(Debug, Clone)]
pub struct GameState {
    /// 当前游戏阶段
    pub phase: GamePhase,
    /// 当前轮到的玩家编号
    pub current_player: usize,
    /// 上一个出牌的玩家编号
    pub last_player: usize,
    /// 上一手已出的牌
    pub last_played_cards: Play,
    /// 地主的底牌
    pub landlord_cards: Cards,
    /// 三个玩家的状态
    pub players: [PlayerState; 3],
}

/// 游戏操作的结果类型
///
/// 这是一个类型别名，简化游戏操作中常见的错误处理模式。
/// 所有游戏操作失败时都返回 `GameError`。
///
/// # 示例
///
/// ```ignore
/// use poker_landlord_rs::game::GameResult;
///
/// fn do_something() -> GameResult<()> {
///     // ...
///     Ok(())
/// }
/// ```
pub type GameResult<T> = std::result::Result<T, GameError>;
