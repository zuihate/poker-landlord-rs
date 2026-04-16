//! 斗地主游戏的错误类型定义
//!
//! 提供统一的错误处理机制，包括：
//! - 牌型分类错误 (`PlayError`)
//! - 玩家操作错误 (`PlayerError`)
//! - 卡牌解析错误 (`CardError`)
//!
//! 以及它们之间的转换和传播方式

use crate::game::GamePhase;
use std::fmt;

/// 游戏的主错误类型
///
/// 枚举所有可能在游戏中发生的错误类型，支持错误链传播。
#[derive(Debug, Clone)]
pub enum GameError {
    /// 牌型分类或验证错误
    Play(PlayError),
    /// 玩家操作错误
    Player(PlayerError),
    /// 卡牌解析或操作错误
    Card(CardError),
    /// 无效的玩家标识符
    InvalidPlayerId(usize),
    /// 操作为错误阶段
    WrongPhase {
        expected: &'static str,
        actual: GamePhase,
    },
    /// 操作被规则或阶段拒绝
    ActionNotAllowed(String),
    /// 其他通用错误
    Other(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::Play(err) => write!(f, "牌型错误: {}", err),
            GameError::Player(err) => write!(f, "玩家错误: {}", err),
            GameError::Card(err) => write!(f, "卡牌错误: {}", err),
            GameError::InvalidPlayerId(id) => write!(f, "无效玩家编号: {}", id),
            GameError::WrongPhase { expected, actual } => {
                write!(f, "错误阶段: 期望 {}，当前 {:?}", expected, actual)
            }
            GameError::ActionNotAllowed(reason) => write!(f, "动作不被允许: {}", reason),
            GameError::Other(msg) => write!(f, "错误: {}", msg),
        }
    }
}

impl std::error::Error for GameError {}

impl From<PlayError> for GameError {
    fn from(err: PlayError) -> Self {
        GameError::Play(err)
    }
}

impl From<PlayerError> for GameError {
    fn from(err: PlayerError) -> Self {
        GameError::Player(err)
    }
}

impl From<CardError> for GameError {
    fn from(err: CardError) -> Self {
        GameError::Card(err)
    }
}

impl From<String> for GameError {
    fn from(err: String) -> Self {
        GameError::Other(err)
    }
}

impl From<&str> for GameError {
    fn from(err: &str) -> Self {
        GameError::Other(err.to_string())
    }
}

// ============================================================================

/// 牌型验证错误
///
/// 表示卡牌集合无法分类为有效的地主游戏牌型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayError {
    /// 卡牌集合不代表任何有效的牌型
    Invalid,
    /// 卡牌为空（未出牌）
    Empty,
    /// 牌型不匹配
    MismatchedType,
}

impl fmt::Display for PlayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayError::Invalid => write!(f, "无效的出牌：卡牌组合不构成任何有效牌型"),
            PlayError::Empty => write!(f, "空牌集合"),
            PlayError::MismatchedType => write!(f, "牌型不匹配"),
        }
    }
}

impl std::error::Error for PlayError {}

// ============================================================================

/// 玩家操作错误
///
/// 表示玩家在游戏中执行操作时的各种错误情况。
#[derive(Debug, Clone)]
pub enum PlayerError {
    /// 玩家没有指定的卡牌
    CardNotFound(String),
    /// 玩家出牌时选择的卡牌不合法
    InvalidPlay(String),
    /// 玩家已是地主，不能再成为地主
    AlreadyLandlord,
    /// 玩家不是地主（当需要地主才能执行的操作时）
    NotLandlord,
    /// 玩家输入无效
    InvalidInput(String),
    /// 玩家手牌为空
    NoCards,
    /// 其他玩家操作错误
    Other(String),
}

impl fmt::Display for PlayerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerError::CardNotFound(card) => write!(f, "手牌中没有: {}", card),
            PlayerError::InvalidPlay(reason) => write!(f, "无效的出牌: {}", reason),
            PlayerError::AlreadyLandlord => write!(f, "玩家已是地主"),
            PlayerError::NotLandlord => write!(f, "玩家不是地主"),
            PlayerError::InvalidInput(input) => write!(f, "无效的输入: {}", input),
            PlayerError::NoCards => write!(f, "玩家手牌为空"),
            PlayerError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for PlayerError {}

impl From<String> for PlayerError {
    fn from(err: String) -> Self {
        PlayerError::Other(err)
    }
}

impl From<&str> for PlayerError {
    fn from(err: &str) -> Self {
        PlayerError::Other(err.to_string())
    }
}

// ============================================================================

/// 卡牌解析和操作错误
///
/// 表示与卡牌相关的错误，包括解析、验证和操作。
#[derive(Debug, Clone)]
pub enum CardError {
    /// 无效的卡牌点数
    InvalidRank(String),
    /// 无效的卡牌花色
    InvalidSuit(String),
    /// 无效的卡牌字符串表示
    InvalidCardString(String),
    /// 卡牌集合为空
    EmptyCards,
    /// 卡牌集合无效
    InvalidCards(String),
    /// 其他卡牌相关错误
    Other(String),
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardError::InvalidRank(rank) => write!(f, "无效的点数: {}", rank),
            CardError::InvalidSuit(suit) => write!(f, "无效的花色: {}", suit),
            CardError::InvalidCardString(s) => write!(f, "无效的卡牌字符串: {}", s),
            CardError::EmptyCards => write!(f, "卡牌集合为空"),
            CardError::InvalidCards(reason) => write!(f, "无效的卡牌集合: {}", reason),
            CardError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CardError {}

impl From<String> for CardError {
    fn from(err: String) -> Self {
        CardError::Other(err)
    }
}

impl From<&str> for CardError {
    fn from(err: &str) -> Self {
        CardError::Other(err.to_string())
    }
}

// ============================================================================

/// 便捷类型别名
///
/// 用于简化返回值类型的声明
pub type Result<T> = std::result::Result<T, GameError>;
pub type PlayResult<T> = std::result::Result<T, PlayError>;
pub type PlayerResult<T> = std::result::Result<T, PlayerError>;
pub type CardResult<T> = std::result::Result<T, CardError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_error_display() {
        let err = PlayError::Invalid;
        assert_eq!(err.to_string(), "无效的出牌：卡牌组合不构成任何有效牌型");
    }

    #[test]
    fn test_player_error_display() {
        let err = PlayerError::CardNotFound("红桃3".to_string());
        assert_eq!(err.to_string(), "手牌中没有: 红桃3");
    }

    #[test]
    fn test_game_error_from_play_error() {
        let play_err = PlayError::Invalid;
        let game_err: GameError = play_err.into();

        match game_err {
            GameError::Play(_) => (),
            _ => panic!("Expected GameError::Play"),
        }
    }

    #[test]
    fn test_game_error_from_player_error() {
        let player_err = PlayerError::NotLandlord;
        let game_err: GameError = player_err.into();

        match game_err {
            GameError::Player(_) => (),
            _ => panic!("Expected GameError::Player"),
        }
    }

    #[test]
    fn test_game_error_display() {
        let game_err: GameError = PlayError::Invalid.into();
        let display_str = game_err.to_string();
        assert!(display_str.contains("牌型错误"));
    }
}
