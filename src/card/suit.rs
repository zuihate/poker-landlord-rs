//! 扑克牌的花色定义及相关操作
//!
//! 包含四种标准花色：黑桃、红桃、梅花、方块
//! 王牌没有花色（用 Option::None 表示）

use crate::error::CardError;

/// 代表扑克牌的四种花色
///
/// 使用 #[repr(u8)] 以便于排序和存储
#[allow(clippy::new_without_default)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suit {
    /// 方块（Diamond）- 红色，编码值 1
    Diamonds = 1,
    /// 梅花（Club）- 黑色，编码值 2
    Clubs = 2,
    /// 红桃（Heart）- 红色，编码值 3
    Hearts = 3,
    /// 黑桃（Spade）- 黑色，编码值 4
    Spades = 4,
}

use std::fmt;

/// Display trait 实现 - 用于打印花色
///
/// 输出花色符号：♠ ♥ ♣ ♦
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit_str = match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Clubs => "♣",
            Suit::Diamonds => "♦",
        };
        write!(f, "{}", suit_str)
    }
}

/// FromStr trait 实现 - 从字符串解析花色
///
/// 支持多种格式（不区分大小写）：
/// - 符号：`"♠"`, `"♥"`, `"♣"`, `"♦"`
/// - 英文字母：`"s"`, `"h"`, `"c"`, `"d"`
/// - 英文全名：`"spade"`, `"spades"`, `"heart"`, `"hearts"` 等
/// - 中文：`"黑桃"`, `"红桃"`, `"梅花"`, `"方块"`
///
/// # 示例
/// ```rust
/// use poker_landlord_rs::card::suit::Suit;
///
/// let suit: Suit = "♠".parse().unwrap();
/// let suit: Suit = "hearts".parse().unwrap();
/// ```
///
/// # 返回值
/// 成功返回 Ok(Suit)，失败返回 CardError::InvalidSuit
impl std::str::FromStr for Suit {
    type Err = CardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw = s.trim();

        if raw.is_empty() {
            return Err(CardError::InvalidSuit(raw.to_string()));
        }

        // 🚀 直接匹配（无分配路径）
        match raw {
            // 符号（最常见，优先匹配）
            "♠" => return Ok(Suit::Spades),
            "♥" => return Ok(Suit::Hearts),
            "♣" => return Ok(Suit::Clubs),
            "♦" => return Ok(Suit::Diamonds),

            // 单字符（大小写一起处理）
            _ if raw.len() == 1 => {
                let c = raw.as_bytes()[0];
                return match c {
                    b's' | b'S' => Ok(Suit::Spades),
                    b'h' | b'H' => Ok(Suit::Hearts),
                    b'c' | b'C' => Ok(Suit::Clubs),
                    b'd' | b'D' => Ok(Suit::Diamonds),
                    _ => Err(CardError::InvalidSuit(raw.to_string())),
                };
            }

            _ => {}
        }

        // 仅在必要时才 lowercase（极少数路径）
        let lower = raw.to_ascii_lowercase();

        let suit = match lower.as_str() {
            "spade" | "spades" | "黑桃" => Suit::Spades,
            "heart" | "hearts" | "红桃" => Suit::Hearts,
            "club" | "clubs" | "梅花" => Suit::Clubs,
            "diamond" | "diamonds" | "方块" => Suit::Diamonds,
            _ => return Err(CardError::InvalidSuit(raw.to_string())),
        };

        Ok(suit)
    }
}
