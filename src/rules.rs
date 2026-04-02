//! 牌型分类和规则引擎
//!
//! 本模块负责识别和验证各种扑克牌型，判断牌的大小关系。
//!
//! # 主要功能
//!
//! - 牌型分类 - 将卡牌集合分类为有效的地主游戏牌型
//! - 牌型验证 - 检查卡牌集合是否构成合法的牌型
//! - 大小比较 - 判断一手牌是否能压制另一手牌
//!
//! # 支持的牌型
//!
//! ## 基础牌型（无特殊结构）
//! - `Single` - 单牌（1张）
//! - `Pair` - 对子（2张相同点数的牌）
//! - `Triple` - 三不带（3张相同点数的牌）
//! - `Bomb` - 炸弹（4张相同点数的牌）
//! - `JokerBomb` - 王炸（大小王各1张）
//!
//! ## 组合牌型（多个点数组合）
//! - `ThreeWithOne` - 三带一（3张+1张）
//! - `ThreeWithTwo` - 三带二（3张+2张）
//! - `FourWithTwo` - 四带二（4张+2张）
//!
//! ## 顺牌类（需要连续的点数）
//! - `Straight` - 顺子（5张以上的单牌连续）
//! - `PairSequence` - 连对（3对以上的连续对子）
//! - `Plane` - 飞机（2个以上的连续三张）
//! - `PlaneWithSingle` - 飞机带单牌
//! - `PlaneWithPair` - 飞机带对子

use std::collections::BTreeMap;

use crate::card::cards::Cards;
use crate::card::rank::Rank;
use crate::error::PlayError;

/// 顺子的最小长度（5张卡牌）
const MIN_STRAIGHT_LENGTH: usize = 5;

/// 连对的最小长度（6张卡牌，至少3对）
const MIN_PAIR_SEQUENCE_LENGTH: usize = 6;

/// 飞机的最小长度（6张卡牌，至少2个三张）
const MIN_PLANE_LENGTH: usize = 6;

/// 飞机所需的最少三张组合数
const MIN_PLANE_TRIPLES: usize = 2;

/// 连对所需的最少对数
const MIN_PAIRS_IN_SEQUENCE: usize = 3;

/// 扑克牌的牌型分类
///
/// 代表地主游戏中的各种牌型，包括基础牌型和组合牌型。
/// 每个牌型都有特定的结构要求和出牌规则。
///
/// # 牌型分类
///
/// ## 基础牌型
/// - `Single` - 单牌
/// - `Pair` - 对子
/// - `Triple` - 三不带
/// - `Bomb` - 炸弹（最强基础牌型）
/// - `JokerBomb` - 王炸（最强牌型）
///
/// ## 组合牌型
/// - `ThreeWithOne` - 三带一
/// - `ThreeWithTwo` - 三带二
/// - `FourWithTwo` - 四带二
///
/// ## 顺牌类
/// - `Straight` - 顺子
/// - `PairSequence` - 连对
/// - `Plane` - 飞机
/// - `PlaneWithSingle` - 飞机带单牌
/// - `PlaneWithPair` - 飞机带对子
///
/// # 出牌规则
///
/// 不同的牌型有不同的压制关系：
/// - 同类牌型中，较大的牌只能被相同或更大的牌压制
/// - 炸弹可以压制除了王炸以外的所有牌型
/// - 王炸是最强的牌型，无法被压制
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayCategory {
    /// 空手 - 表示没有出牌
    ///
    /// 无任何牌。通常用于表示玩家选择不出牌或轮到玩家但没有合法出牌
    Pass,
    /// 单牌 - 基础牌型
    ///
    /// 1张任意卡牌。可以被更大点数的单牌、对子、三不带等压制。
    Single,

    /// 对子 - 基础牌型
    ///
    /// 2张相同点数的卡牌。可以被更大的对子、三不带等压制。
    Pair,

    /// 三不带 - 基础牌型
    ///
    /// 3张相同点数的卡牌。可以被更大的三不带、炸弹等压制。
    Triple,

    /// 炸弹 - 基础牌型
    ///
    /// 4张相同点数的卡牌。这是最强的基础牌型，可以压制除王炸外的所有牌型。
    /// 但在地主中，同样是炸弹的情况下，较大点数的炸弹压过较小点数的。
    Bomb,

    /// 王炸 - 最强牌型
    ///
    /// 大小王各1张。这是游戏中最强的牌型，无法被任何其他牌压制。
    /// 只有在大家都无法跟牌时才会出现。
    JokerBomb,

    /// 三带一 - 组合牌型
    ///
    /// 3张相同点数 + 1张其他点数的卡牌（总共4张）。
    /// 出牌时，必须用相同点数的三张进行压制，附加的单牌由出牌方自由选择。
    ThreeWithOne,

    /// 三带二 - 组合牌型
    ///
    /// 3张相同点数 + 2张相同点数的其他卡牌（总共5张）。
    /// 比三带一更强，出牌时必须用相同点数的三张压制，附加的对子由出牌方自由选择。
    ThreeWithTwo,

    /// 四带二 - 组合牌型
    ///
    /// 4张相同点数（炸弹） + 2张相同点数的其他卡牌（总共6张）。
    /// 这是最强的组合牌型，出牌时必须用相同点数的四张压制，附加的对子由出牌方自由选择。
    FourWithTwo,

    /// 顺子 - 顺牌类
    ///
    /// 至少5张卡牌，每个点数各1张，点数必须连续。
    /// - 最小顺子：3-4-5-6-7（五顺）
    /// - 最大顺子：10-J-Q-K-A（五顺）
    /// - 2和王牌不能参与顺子
    /// - 顺子只能被更大的顺子压制（需要相同张数）
    Straight,

    /// 连对 - 顺牌类
    ///
    /// 至少3对卡牌，每个点数各2张，点数必须连续。
    /// - 最少为3对（6张）
    /// - 2和王牌不能参与连对
    /// - 连对只能被相同张数的更大连对压制
    PairSequence,

    /// 飞机 - 顺牌类
    ///
    /// 至少2个三张组合，各三张的点数必须连续。
    /// 也称为"三顺"或"三连"，是最复杂的基础牌型之一。
    /// - 最小飞机：两个三张（6张）
    /// - 2和王牌不能参与飞机的三张组合
    /// - 飞机只能被相同数量三张的更大飞机压制（或带牌的飞机）
    Plane,

    /// 飞机带单牌 - 顺牌类变体
    ///
    /// 飞机 + 相同数量的单牌。
    /// 例如：两个三张（飞机）+ 2张单牌 = 8张总共。
    /// 单牌的点数不能是飞机中已有的点数。
    PlaneWithSingle,

    /// 飞机带对子 - 顺牌类变体
    ///
    /// 飞机 + 相同数量的对子。
    /// 例如：两个三张（飞机）+ 2个对子 = 10张总共。
    /// 对子的点数不能是飞机中已有的点数。
    PlaneWithPair,
}

/// 一次有效的出牌
///
/// 包含分类后的牌型和原始卡牌集合。创建时自动验证卡牌集合是否代表有效的牌型。
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::cards::Cards;
/// use poker_landlord_rs::card::rank::Rank;
/// use poker_landlord_rs::card::suit::Suit;
/// use poker_landlord_rs::rules::{Play, PlayCategory};
///
/// let mut cards = Cards::new();
/// cards.push(Card::new(Rank::Three, Suit::Spades));
///
/// let play = Play::new(cards).expect("valid play");
/// assert_eq!(play.category, PlayCategory::Single);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Play {
    /// 识别出的牌型分类
    pub category: PlayCategory,
    /// 原始卡牌集合
    pub cards: Cards,
}

impl Play {
    /// 创建一个新的牌型，自动验证和分类
    ///
    /// 检查卡牌集合是否代表有效的地主游戏牌型，如果有效，则创建 Play 对象。
    ///
    /// # 参数
    /// * `cards` - 要分类的卡牌集合
    ///
    /// # 返回值
    /// - `Ok(Play)` 如果卡牌集合是有效的牌型
    /// - `Err(PlayError::Invalid)` 如果卡牌集合无法分类为任何有效牌型
    ///
    /// # 示例
    /// ```
    /// use poker_landlord_rs::card::Card;
    /// use poker_landlord_rs::card::cards::Cards;
    /// use poker_landlord_rs::card::rank::Rank;
    /// use poker_landlord_rs::card::suit::Suit;
    /// use poker_landlord_rs::rules::Play;
    ///
    /// let mut cards = Cards::new();
    /// cards.push(Card::new(Rank::Three, Suit::Spades));
    ///
    /// match Play::new(cards) {
    ///     Ok(play) => println!("Valid play: {:?}", play.category),
    ///     Err(_) => println!("Invalid play"),
    /// }
    /// ```
    pub fn new(cards: Cards) -> Result<Self, PlayError> {
        let category = classify_play(&cards).ok_or(PlayError::Invalid)?;
        Ok(Play { category, cards })
    }

    /// 获取牌型的首要点数（用于比较大小）
    ///
    /// 根据不同的牌型返回用于比较的主要点数：
    /// - 单牌、对子、三张及其组合：返回该点数
    /// - 顺牌类：返回最大点数
    /// - 王炸：返回大王
    /// - 空手：返回 None
    ///
    /// # 返回值
    /// 用于比较大小的主要点数，如果是空手则返回 None
    fn get_main_rank(&self) -> Option<Rank> {
        if self.cards.is_empty() {
            return None;
        }

        match self.category {
            PlayCategory::Pass => None,

            // 单牌、对子、三张、炸弹、三带一/二、四带二：取出现次数最多的点数
            PlayCategory::Single
            | PlayCategory::Pair
            | PlayCategory::Triple
            | PlayCategory::Bomb
            | PlayCategory::ThreeWithOne
            | PlayCategory::ThreeWithTwo
            | PlayCategory::FourWithTwo => {
                // 取第一个牌的点数（这些牌型的核心点数在最前面或次数最多）
                self.cards.iter().next().map(|card| card.rank)
            }
            // 王炸，返回大王
            PlayCategory::JokerBomb => Some(Rank::JokerBig),

            // 顺牌类：取最大点数
            PlayCategory::Straight
            | PlayCategory::PairSequence
            | PlayCategory::Plane
            | PlayCategory::PlaneWithSingle
            | PlayCategory::PlaneWithPair => self.cards.iter().map(|card| card.rank).max(),
        }
    }

    /// 判断当前牌是否能压过对方的牌
    ///
    /// 根据斗地主规则判断：
    /// - 牌型必须相同或满足特殊压制规则（如炸弹压非炸弹）
    /// - 对于同类型的牌，比较首要点数的大小
    ///
    /// # 参数
    /// * `other` - 对方出的牌
    ///
    /// # 返回值
    /// `true` 如果当前牌能压过对方的牌，`false` 如果不能或无法比较
    pub fn can_beat(&self, other: &Play) -> bool {
        // 任何有效的牌都能压过空手（Pass）
        if other.category == PlayCategory::Pass {
            return !self.cards.is_empty();
        }

        // 王炸是最强的，只有王炸能压过王炸
        if self.category == PlayCategory::JokerBomb {
            return other.category != PlayCategory::JokerBomb;
        }

        // 王炸无法被任何东西压制（除了自己）
        if other.category == PlayCategory::JokerBomb {
            return false;
        }

        // 炸弹可以压过非炸弹牌（但被王炸压制）
        if self.category == PlayCategory::Bomb && other.category != PlayCategory::Bomb {
            return true;
            // 两个都是炸弹，按点数比较
        }

        // 不同牌型不能互相压制
        if self.category != other.category {
            return false;
        }

        // 同类型的牌比较首要点数
        match (self.get_main_rank(), other.get_main_rank()) {
            (Some(self_rank), Some(other_rank)) => self_rank > other_rank,
            _ => false,
        }
    }
}

/// 判断卡牌集合属于哪一种牌型
///
/// 使用策略模式依次尝试识别各种牌型，按优先级顺序检查：
/// 1. 顺牌类型（顺子、连对、飞机） - 最复杂的判断规则，需要优先检查
/// 2. 基础模式（单牌到四带二）- 简单的数量和结构检查
///
/// # 性能优化
///
/// - 构建 rank_counts 只一次，所有检查函数共享
/// - 根据卡牌长度快速路由，避免不必要的检查
/// - 大多数情况是基础牌型，最优化分支预测
///
/// # 参数
/// * `cards` - 要分类的卡牌集合
///
/// # 返回值
/// - 匹配的 `PlayCategory`，或 `None`（如果不匹配任何牌型）
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::cards::Cards;
/// use poker_landlord_rs::card::rank::Rank;
/// use poker_landlord_rs::card::suit::Suit;
/// use poker_landlord_rs::rules::{classify_play, PlayCategory};
///
/// let mut cards = Cards::new();
/// cards.push(Card::new(Rank::Three, Suit::Spades));
/// assert_eq!(classify_play(&cards), Some(PlayCategory::Single));
/// ```
pub fn classify_play(cards: &Cards) -> Option<PlayCategory> {
    let len = cards.len();

    // 快速路径：空卡牌集合
    if len == 0 {
        return Some(PlayCategory::Pass);
    }

    // 构建一次 rank_counts，所有检查函数共享
    let rank_counts = build_rank_counts(cards);

    // 根据卡牌长度智能路由
    match len {
        // 基础牌型专属范围（1-4张）
        1..=4 => try_basic_pattern_with_counts(cards, len, &rank_counts),

        // 5张：可能是三带二或顺子
        5 => try_basic_pattern_with_counts(cards, len, &rank_counts)
            .or_else(|| try_straight_with_counts(cards, &rank_counts)),

        // 6张：最复杂的范围，可能是四带二、顺子、连对或飞机
        6 => try_basic_pattern_with_counts(cards, len, &rank_counts)
            .or_else(|| try_straight_with_counts(cards, &rank_counts))
            .or_else(|| try_pair_sequence_with_counts(cards, &rank_counts))
            .or_else(|| try_plane_with_counts(cards, &rank_counts)),

        // 7+张：优先顺牌类，最后检查基础牌型（虽然不太可能）
        _ => try_straight_with_counts(cards, &rank_counts)
            .or_else(|| try_pair_sequence_with_counts(cards, &rank_counts))
            .or_else(|| try_plane_with_counts(cards, &rank_counts))
            .or_else(|| try_basic_pattern_with_counts(cards, len, &rank_counts)),
    }
}

/// 验证卡牌集合是否代表一个合法的牌型
///
/// 检查给定的卡牌集合是否能分类为任何有效的牌型。
///
/// # 参数
/// * `cards` - 要验证的卡牌集合
///
/// # 返回值
/// `true` 如果卡牌集合是合法的牌型，否则 `false`
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::cards::Cards;
/// use poker_landlord_rs::card::rank::Rank;
/// use poker_landlord_rs::card::suit::Suit;
/// use poker_landlord_rs::rules::is_valid_play;
///
/// let mut cards = Cards::new();
/// cards.push(Card::new(Rank::Three, Suit::Spades));
/// assert!(is_valid_play(&cards));
/// ```
pub fn is_valid_play(cards: &Cards) -> bool {
    classify_play(cards).is_some()
}

// 辅助函数

/// 构建卡牌的点数统计映射
///
/// 统计每个点数出现的次数，自动排序点数。
///
/// # 参数
/// * `cards` - 要统计的卡牌集合
///
/// # 返回值
/// 按点数排序的点数-数量映射
fn build_rank_counts(cards: &Cards) -> BTreeMap<Rank, usize> {
    let mut rank_counts = BTreeMap::new();
    for card in cards {
        *rank_counts.entry(card.rank).or_insert(0) += 1;
    }
    rank_counts
}

/// 检查某个点数是否为禁用点数
///
/// 禁用点数指不能参与某些牌型的点数（如顺子和连对中的2和王牌）。
///
/// # 参数
/// * `rank` - 要检查的点数
///
/// # 返回值
/// `true` 如果点数是禁用的（2、小王或大王），否则 `false`
fn is_forbidden_rank(rank: Rank) -> bool {
    matches!(rank, Rank::Two | Rank::JokerSmall | Rank::JokerBig)
}

/// 检查一个点数集合中是否包含禁用点数
///
/// # 参数
/// * `ranks` - 要检查的点数切片
///
/// # 返回值
/// `true` 如果包含任何禁用点数，否则 `false`
fn contains_forbidden_ranks(ranks: &[Rank]) -> bool {
    ranks.iter().any(|r| is_forbidden_rank(*r))
}

/// 检查一个点数迭代器中是否包含禁用点数
///
/// # 参数
/// * `ranks` - 要检查的点数迭代器
///
/// # 返回值
/// `true` 如果包含任何禁用点数，否则 `false`
fn contains_forbidden_ranks_iter<I: IntoIterator<Item = Rank>>(ranks: I) -> bool {
    ranks.into_iter().any(is_forbidden_rank)
}

/// 检查一组已排序的点数是否连续
///
/// 直接从迭代器检查，无需创建中间 Vec。
/// 假设输入的点数已按升序排列。
///
/// # 参数
/// * `ranks` - 已排序的点数迭代器
///
/// # 返回值
/// `true` 如果点数连续，否则 `false`
fn are_ranks_consecutive_from_iter<I: IntoIterator<Item = Rank>>(ranks: I) -> bool {
    let mut prev_value: Option<u8> = None;

    for rank in ranks {
        let curr_value = rank as u8;
        match prev_value {
            None => prev_value = Some(curr_value),
            Some(p) => {
                if curr_value != p + 1 {
                    return false;
                }
                prev_value = Some(curr_value);
            }
        }
    }

    true
}

// 牌型识别函数

/// 尝试识别顺子（使用预构建的 rank_counts）
fn try_straight_with_counts(
    cards: &Cards,
    rank_counts: &BTreeMap<Rank, usize>,
) -> Option<PlayCategory> {
    let len = cards.len();
    if len < MIN_STRAIGHT_LENGTH {
        return None;
    }

    // 顺子：每个点数恰好1张
    if rank_counts.len() != len {
        return None;
    }

    // 检查是否包含非法点数（直接从迭代器，无需创建 Vec）
    if contains_forbidden_ranks_iter(rank_counts.keys().copied()) {
        return None;
    }

    // 检查点数是否连续（直接从迭代器，无需创建中间 Vec）
    are_ranks_consecutive_from_iter(rank_counts.keys().copied()).then_some(PlayCategory::Straight)
}

/// 尝试识别连对（使用预构建的 rank_counts）
fn try_pair_sequence_with_counts(
    cards: &Cards,
    rank_counts: &BTreeMap<Rank, usize>,
) -> Option<PlayCategory> {
    let len = cards.len();

    // 长度必须至少6张，并且必须是偶数
    if len < MIN_PAIR_SEQUENCE_LENGTH || !len.is_multiple_of(2) {
        return None;
    }

    let pair_count = len / 2;

    // 至少要有3对
    if pair_count < MIN_PAIRS_IN_SEQUENCE {
        return None;
    }

    // 连对：点数数量必须等于对数
    if rank_counts.len() != pair_count {
        return None;
    }

    // 每个点数恰好2张
    if !rank_counts.values().all(|&count| count == 2) {
        return None;
    }

    // 检查是否包含非法点数（直接从迭代器，无需创建 Vec）
    if contains_forbidden_ranks_iter(rank_counts.keys().copied()) {
        return None;
    }

    // 检查点数是否连续（直接从迭代器，无需创建中间 Vec）
    are_ranks_consecutive_from_iter(rank_counts.keys().copied())
        .then_some(PlayCategory::PairSequence)
}

/// 尝试识别飞机牌型（使用预构建的 rank_counts）
fn try_plane_with_counts(
    cards: &Cards,
    rank_counts: &BTreeMap<Rank, usize>,
) -> Option<PlayCategory> {
    let len = cards.len();
    if len < MIN_PLANE_LENGTH {
        return None;
    }

    // 有效卡牌数量：3（三张数） + 1或2（带牌数）
    let valid_counts = rank_counts.values().all(|&c| (1..=3).contains(&c));

    if !valid_counts {
        return None;
    }

    // 单次遍历收集各类点数，避免多次过滤
    let mut triple_ranks = Vec::new();
    let mut single_ranks = Vec::new();
    let mut pair_ranks = Vec::new();

    for (rank, count) in rank_counts.iter() {
        match count {
            3 => triple_ranks.push(*rank),
            1 => single_ranks.push(*rank),
            2 => pair_ranks.push(*rank),
            _ => {}
        }
    }

    if triple_ranks.len() < MIN_PLANE_TRIPLES {
        return None;
    }

    // 检查三张组合是否包含禁用点数
    if contains_forbidden_ranks(&triple_ranks) {
        return None;
    }

    // 检查三张点数是否连续（BTreeMap已排序，直接从迭代器）
    if !are_ranks_consecutive_from_iter(triple_ranks.iter().copied()) {
        return None;
    }

    let base_triples_len = triple_ranks.len() * 3;

    // 识别具体飞机类型
    if len == base_triples_len {
        // 纯飞机
        Some(PlayCategory::Plane)
    } else if len == base_triples_len + single_ranks.len()
        && single_ranks.len() == triple_ranks.len()
    {
        // 飞机带单牌：单牌数量必须等于三张组合数
        // 检查单牌与三张组合没有重复
        for rank in &single_ranks {
            if triple_ranks.contains(rank) {
                return None;
            }
        }
        Some(PlayCategory::PlaneWithSingle)
    } else if len == base_triples_len + pair_ranks.len() * 2
        && pair_ranks.len() == triple_ranks.len()
    {
        // 飞机带对子：对子数量必须等于三张组合数
        // 检查对子与三张组合没有重复
        for rank in &pair_ranks {
            if triple_ranks.contains(rank) {
                return None;
            }
        }
        Some(PlayCategory::PlaneWithPair)
    } else {
        None
    }
}

/// 尝试识别基础牌型（使用预构建的 rank_counts）
fn try_basic_pattern_with_counts(
    _cards: &Cards,
    len: usize,
    rank_counts: &BTreeMap<Rank, usize>,
) -> Option<PlayCategory> {
    let max_count = rank_counts.values().copied().max().unwrap_or(0);

    match len {
        1 => Some(PlayCategory::Single),

        2 => match max_count {
            // 两张点数不同的卡牌，检查是否为王炸
            1 => (rank_counts.get(&Rank::JokerSmall) == Some(&1)
                && rank_counts.get(&Rank::JokerBig) == Some(&1))
            .then_some(PlayCategory::JokerBomb),

            2 => Some(PlayCategory::Pair),
            _ => None,
        },

        3 => (max_count == 3).then_some(PlayCategory::Triple),

        4 => match max_count {
            3 => Some(PlayCategory::ThreeWithOne),
            4 => Some(PlayCategory::Bomb),
            _ => None,
        },

        5 => {
            // 三带二：最大数量为3，且恰好2种点数
            (max_count == 3 && rank_counts.len() == 2).then_some(PlayCategory::ThreeWithTwo)
        }

        6 => {
            // 四带二：最大数量为4，且恰好2种点数
            (max_count == 4 && rank_counts.len() == 2).then_some(PlayCategory::FourWithTwo)
        }

        _ => None,
    }
}

// 单元测试

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;
    use crate::card::suit::Suit;

    // 辅助函数：创建对子
    fn pair(rank: Rank) -> Cards {
        let mut cards = Cards::new();
        cards.push(Card::new(rank, Suit::Spades));
        cards.push(Card::new(rank, Suit::Hearts));
        cards
    }

    // 辅助函数：创建炸弹
    fn bomb(rank: Rank) -> Cards {
        let mut cards = Cards::new();
        cards.push(Card::new(rank, Suit::Spades));
        cards.push(Card::new(rank, Suit::Hearts));
        cards.push(Card::new(rank, Suit::Diamonds));
        cards.push(Card::new(rank, Suit::Clubs));
        cards
    }

    #[test]
    fn test_pair_comparison() {
        let pair_3 = Play::new(pair(Rank::Three)).unwrap();
        let pair_5 = Play::new(pair(Rank::Five)).unwrap();

        // 对子5应该能压过对子3
        assert!(pair_5.can_beat(&pair_3));
        assert!(!pair_3.can_beat(&pair_5));
    }

    #[test]
    fn test_bomb_comparison() {
        let bomb_3 = Play::new(bomb(Rank::Three)).unwrap();
        let bomb_5 = Play::new(bomb(Rank::Five)).unwrap();

        // 炸弹5应该能压过炸弹3
        assert!(bomb_5.can_beat(&bomb_3));
    }

    #[test]
    fn test_pair_sequence_comparison() {
        let mut cards_3_4_5 = Cards::new();
        cards_3_4_5.push(Card::new(Rank::Three, Suit::Spades));
        cards_3_4_5.push(Card::new(Rank::Three, Suit::Hearts));
        cards_3_4_5.push(Card::new(Rank::Four, Suit::Spades));
        cards_3_4_5.push(Card::new(Rank::Four, Suit::Hearts));
        cards_3_4_5.push(Card::new(Rank::Five, Suit::Spades));
        cards_3_4_5.push(Card::new(Rank::Five, Suit::Hearts));

        let mut cards_4_5_6 = Cards::new();
        cards_4_5_6.push(Card::new(Rank::Four, Suit::Spades));
        cards_4_5_6.push(Card::new(Rank::Four, Suit::Hearts));
        cards_4_5_6.push(Card::new(Rank::Five, Suit::Spades));
        cards_4_5_6.push(Card::new(Rank::Five, Suit::Hearts));
        cards_4_5_6.push(Card::new(Rank::Six, Suit::Spades));
        cards_4_5_6.push(Card::new(Rank::Six, Suit::Hearts));

        let pair_seq1 = Play::new(cards_3_4_5).unwrap();
        let pair_seq2 = Play::new(cards_4_5_6).unwrap();

        // 连对4-5-6应该能压过连对3-4-5
        assert!(pair_seq2.can_beat(&pair_seq1));
    }
    #[test]
    fn test_pair_sequence_() {
        let mut cards_3_4_5 = Cards::new();
        cards_3_4_5.push(Card::new(Rank::Three, Suit::Spades));
        cards_3_4_5.push(Card::new(Rank::Three, Suit::Hearts));
        cards_3_4_5.push(Card::new(Rank::Four, Suit::Spades));
        cards_3_4_5.push(Card::new(Rank::Four, Suit::Hearts));
        cards_3_4_5.push(Card::new(Rank::Five, Suit::Spades));
        cards_3_4_5.push(Card::new(Rank::Five, Suit::Hearts));
        let cards_pass = Cards::new();
        let pair_seq = Play::new(cards_3_4_5).unwrap();
        let pass = Play::new(cards_pass).unwrap();
        // 任何牌都能压过空手
        assert!(pair_seq.can_beat(&pass));
    }
}
