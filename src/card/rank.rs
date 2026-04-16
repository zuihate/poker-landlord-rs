//! 扑克牌的点数（等级）定义及相关操作
//!
//! 地主中的点数顺序为：3 < 4 < 5 < ... < A < 2 < 小王 < 大王
//! 用数值表示便于排序和比较

/// 代表扑克牌的点数
///
/// 使用 #[repr(u8)] 使得点数能直接用数值表示，便于排序和比较。
/// 点数的排序规则遵循地主游戏规则。
///
/// # 点数顺序
/// - 普通牌：3(3) - 10(10) - J(11) - Q(12) - K(13) - A(14) - 2(15)
/// - 王牌：小王(16) - 大王(17)
#[allow(clippy::new_without_default)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    /// 3 - 最小的普通牌
    Three = 3,
    /// 4
    Four = 4,
    /// 5
    Five = 5,
    /// 6
    Six = 6,
    /// 7
    Seven = 7,
    /// 8
    Eight = 8,
    /// 9
    Nine = 9,
    /// 10
    Ten = 10,
    /// J - 花牌
    Jack = 11,
    /// Q - 花牌
    Queen = 12,
    /// K - 花牌
    King = 13,
    /// A - 花牌
    Ace = 14,
    /// 2 - 最大的普通牌，在地主中特别强势
    Two = 15,
    /// 小王 - 可以压制普通牌
    JokerSmall = 16,
    /// 大王 - 最强的牌，只能被大王压制
    JokerBig = 17,
}

impl Rank {
    /// 所有点数的完整列表
    pub const ALL: [Rank; 15] = [
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
        Rank::Two,
        Rank::JokerSmall,
        Rank::JokerBig,
    ];

    /// 获取点数的数值表示
    ///
    /// 返回与 enum 判别式相同的值，便于数值比较
    pub fn value(&self) -> u8 {
        *self as u8
    }

    /// 判断该点数是否为王牌（小王或大王）
    ///
    /// 王牌与普通牌的区别：
    /// - 不能与普通牌组成牌型
    /// - 只能用王炸压制
    pub fn is_joker(&self) -> bool {
        matches!(self, Rank::JokerSmall | Rank::JokerBig)
    }
}

use std::fmt;

/// Display trait 实现 - 用于打印点数
///
/// 输出首选格式：
/// - 普通牌：数字 "3" - "9"、"10" 和字母 "J", "Q", "K", "A"
/// - 普通牌：中文 "2"
/// - 王牌：中文 "小王" 和 "大王"
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::JokerSmall => "小王",
            Rank::JokerBig => "大王",
        };
        write!(f, "{}", s)
    }
}

/// FromStr trait 实现 - 从字符串解析点数
///
/// 支持多种格式（不区分大小写）：
/// - 数字：`"3"` 到 `"9"`
/// - 十：`"10"`, `"0"`, `"t"`
/// - 花牌：`"j"`, `"q"`, `"k"`, `"a"`
/// - 花牌英文：`"jack"`, `"queen"`, `"king"`, `"ace"`
/// - 二：`"2"`
/// - 小王：`"s"`, `"small"`, `"jokersmall"`, `"sj"`, `"小王"`
/// - 大王：`"b"`, `"big"`, `"jokerbig"`, `"bj"`, `"大王"`
///
/// # 返回值
/// - `Ok(Rank)`：解析成功
/// - `Err(String)`：解析失败（包含原始输入）
///
/// # 示例
/// ```
/// use std::str::FromStr;
/// use poker_landlord_rs::card::rank::Rank;
///
/// assert_eq!(Rank::from_str("3").unwrap(), Rank::Three);
/// assert_eq!(Rank::from_str("K").unwrap(), Rank::King);
/// assert_eq!(Rank::from_str("小王").unwrap(), Rank::JokerSmall);
/// assert!(Rank::from_str("invalid").is_err());
/// ```
impl std::str::FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_trimmed = s.trim();
        let s = s_trimmed.to_ascii_lowercase();

        let rank = match s.as_str() {
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            "9" => Rank::Nine,
            "10" | "0" | "t" => Rank::Ten,
            "j" | "jack" => Rank::Jack,
            "q" | "queen" => Rank::Queen,
            "k" | "king" => Rank::King,
            "a" | "ace" => Rank::Ace,
            "2" => Rank::Two,
            "s" | "small" | "jokersmall" | "sj" | "小王" => Rank::JokerSmall,
            "b" | "big" | "jokerbig" | "bj" | "大王" => Rank::JokerBig,
            _ => return Err(format!("Invalid rank: {}", s_trimmed)),
        };

        Ok(rank)
    }
}
