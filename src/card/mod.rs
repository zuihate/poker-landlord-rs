//! 卡牌模块
//!
//! 提供扑克牌游戏所需的所有基础数据结构和操作。
//!
//! # 子模块
//!
//! - [`rank`] - 卡牌点数（3-A 及王牌）
//! - [`suit`] - 卡牌花色（黑桃、红心、方块、梅花）
//! - [`card`] - 单张卡牌及其操作
//! - [`cards`] - 卡牌集合（手牌、出牌等）
//! - [`parser`] - 卡牌字符串解析

pub mod cards;
pub mod parser;
pub mod rank;
pub mod suit;

use super::card::rank::Rank;
use super::card::suit::Suit;
use crate::error::CardError;

/// 代表一张扑克牌
///
/// 一张卡牌由点数和花色组成。王牌特殊处理：没有花色。
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::rank::Rank;
/// use poker_landlord_rs::card::suit::Suit;
///
/// // 创建普通牌
/// let card = Card::new(Rank::Three, Suit::Spades);  // ♠3
///
/// // 创建王牌
/// let big_joker = Card::joker(false);  // 大王
/// let small_joker = Card::joker(true); // 小王
/// ```
#[allow(clippy::new_without_default)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    /// 卡牌的点数
    pub rank: Rank,
    /// 卡牌的花色，王牌为 None
    pub suit: Option<Suit>,
}

impl Card {
    /// 创建一张普通牌
    ///
    /// # 参数
    /// * `rank` - 点数
    /// * `suit` - 花色
    ///
    /// # Panics
    /// 如果点数是王牌（小王或大王）会触发 panic，因为王牌不应该有花色
    pub fn new(rank: Rank, suit: Suit) -> Self {
        assert!(!rank.is_joker());
        Self {
            rank,
            suit: Some(suit),
        }
    }

    /// 创建一张王牌
    ///
    /// # 参数
    /// * `small` - 如果为 true 创建小王，否则创建大王
    pub fn joker(small: bool) -> Self {
        Self {
            rank: if small {
                Rank::JokerSmall
            } else {
                Rank::JokerBig
            },
            suit: None,
        }
    }

    /// 判断这张牌是否是王牌
    pub fn is_joker(&self) -> bool {
        self.suit.is_none()
    }

    /// 判断这张牌是否是普通牌（非王牌）
    pub fn is_normal(&self) -> bool {
        self.suit.is_some()
    }

    /// 获取花色，王牌返回 None
    pub fn get_suit(&self) -> Option<Suit> {
        self.suit
    }

    /// 判断这张牌是否拥有指定的花色
    pub fn has_suit(&self, suit: Suit) -> bool {
        matches!(self.suit, Some(s) if s == suit)
    }

    /// 判断这张牌是否与指定的点数和花色匹配
    pub fn matches(&self, rank: Rank, suit: Option<Suit>) -> bool {
        self.rank == rank && self.suit == suit
    }
}

/// Display trait 实现 - 用于打印卡牌
///
/// 格式：花色符号 + 点数（例：♠3、♥K、大王）
impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.suit {
            Some(suit) => write!(f, "{}{}", suit, self.rank),
            None => write!(f, "{}", self.rank),
        }
    }
}

/// FromStr trait 实现 - 从字符串解析卡牌
///
/// 支持多种格式：
/// - 王牌：`"小王"`, `"大王"`, `"sj"`, `"bj"`
/// - 普通牌：`"♠3"` (花色在前) 或 `"3♠"` (点数在前)
/// - 普通牌：`"S3"`, `"3S"` (字母代替符号)
impl std::str::FromStr for Card {
    type Err = CardError;

    /// 从字符串解析卡牌
    ///
    /// # 解析规则
    /// 1. 先尝试解析为王牌（如 "小王", "大王", "sj", "bj"）
    /// 2. 再尝试解析为普通牌（花色 + 点数，或 点数 + 花色）
    ///
    /// # 返回值
    /// 如果格式正确返回 Ok(Card)，否则返回 Err(CardError)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(CardError::InvalidCardString(s.to_string()));
        }

        // 尝试解析为王牌
        if let Ok(rank) = s.parse::<Rank>()
            && rank.is_joker()
        {
            return Ok(Card::joker(rank == Rank::JokerSmall));
        }

        // 对于普通牌，应该有花色符号或字母
        // 支持 "♠3", "3♠", "S3", "3S" 等格式
        let chars: Vec<char> = s.chars().collect();
        if chars.is_empty() {
            return Err(CardError::InvalidCardString(s.to_string()));
        }

        // 尝试从字符中分离花色和点数
        let suit_char = chars[0].to_string();
        let rank_part = chars[1..].iter().collect::<String>();

        let suit = suit_char.parse::<Suit>();
        let rank = rank_part.parse::<Rank>();

        if let (Ok(rank), Ok(suit)) = (rank, suit) {
            return Ok(Card::new(rank, suit));
        }

        // 反向尝试：点数在前，花色在后
        let rank_part = chars[0].to_string();
        let suit_char = chars[1..].iter().collect::<String>();

        let suit = suit_char.parse::<Suit>();
        let rank = rank_part.parse::<Rank>();

        if let (Ok(rank), Ok(suit)) = (rank, suit) {
            return Ok(Card::new(rank, suit));
        }

        Err(CardError::Other(format!("Unknown error: {}", s)))
    }
}
