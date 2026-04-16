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

pub mod dealer;

mod engine;
mod types;

pub use engine::Game;
pub use types::{GameAction, GameActionResult, GamePhase, GameResult, GameState, PlayerState};
