//! 卡牌集合的定义及相关操作
//!
//! 用于表示和操作一组卡牌，例如玩家的手牌、出牌等。
//! 内部使用 Vec 存储，外部包装以实现 Deref 和多种迭代器特性。

use std::collections::HashMap;

use super::{Card, Rank};

#[macro_export]
macro_rules! cards {
    ($card:expr; $n:expr) => {
        $crate::card::Cards::from_vec(vec![$card; $n])
    };
}

/// 代表一组扑克牌的集合
///
/// 使用 Vec<Card> 作为内部存储，支持排序、查询、迭代等操作。
/// 新增的便利方法使其适合游戏中的各种场景。
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// // 创建卡牌集合
/// let mut cards = Cards::new();
/// cards.push(Card::new(Rank::Three, Suit::Spades));
/// cards.push(Card::new(Rank::Three, Suit::Hearts));
///
/// // 排序
/// cards.sort();
///
/// // 查询
/// assert_eq!(cards.len(), 2);
/// assert_eq!(cards.count_rank(Rank::Three), 2);
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cards(Vec<Card>);

impl Cards {
    /// 创建一个空的卡牌集合
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// 创建一个指定容量的卡牌集合
    ///
    /// 可以预先分配底层 Vec 的空间，避免重复扩容。
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// 从 Vec<Card> 创建卡牌集合
    pub fn from_vec(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    /// 从卡牌切片创建卡牌集合
    pub fn from_slice(cards: &[Card]) -> Self {
        Self(cards.to_vec())
    }

    /// 内部方法：生成卡牌计数 HashMap
    ///
    /// 用于需要频繁计数的操作
    #[inline]
    fn make_counts(&self) -> HashMap<Card, u8> {
        let mut counts = HashMap::with_capacity(self.len());
        for card in &self.0 {
            *counts.entry(*card).or_insert(0) += 1;
        }
        counts
    }

    /// 内部方法：生成点数计数 HashMap
    ///
    /// 用于 count_rank 等操作
    #[inline]
    fn make_rank_counts(&self) -> HashMap<Rank, u8> {
        let mut counts = HashMap::with_capacity(15);
        for card in &self.0 {
            *counts.entry(card.rank).or_insert(0) += 1;
        }
        counts
    }

    /// 对集合中的卡牌进行原位排序
    ///
    /// 排序规则：先按点数排序，相同点数再按花色排序
    pub fn sort(&mut self) {
        self.0.sort_unstable();
    }

    /// 返回一个已排序的新卡牌集合（按点数然后花色排序）
    ///
    /// 消费当前集合，返回排序后的新集合
    pub fn sorted(mut self) -> Self {
        self.sort();
        self
    }

    /// 判断集合是否包含另一个集合中的所有卡牌
    ///
    /// 用于检查玩家是否能出指定的牌
    ///
    /// # 时间复杂度
    /// O(n + m)，其中 n 是当前集合大小，m 是检查集合大小
    pub fn contains_all(&self, other: &Cards) -> bool {
        if other.is_empty() {
            return true;
        }
        if self.len() < other.len() {
            return false;
        }

        // 使用 HashMap 替代 BTreeMap，提升性能
        let mut self_count = self.make_counts();

        for card in &other.0 {
            if let Some(cnt) = self_count.get_mut(&card) {
                if *cnt > 0 {
                    *cnt -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// 统计集合中指定点数的卡牌数量
    ///
    /// 用于判断是否有炸弹、顺子等牌型
    ///
    /// # 示例
    /// ```
    /// use poker_landlord_rs::card::Cards;
    /// use poker_landlord_rs::card::Card;
    /// use poker_landlord_rs::card::rank::Rank;
    /// use poker_landlord_rs::card::suit::Suit;
    ///
    /// let cards = Cards::from_vec(vec![
    ///     Card::new(Rank::Three, Suit::Spades),
    ///     Card::new(Rank::Three, Suit::Hearts),
    /// ]);
    /// assert_eq!(cards.count_rank(Rank::Three), 2);
    /// ```
    pub fn count_rank(&self, rank: Rank) -> u8 {
        // 使用预计算的 rank_counts
        self.make_rank_counts().get(&rank).copied().unwrap_or(0)
    }

    /// 批量获取所有点数的计数
    ///
    /// 用于需要多次调用 count_rank 的场景，避免重复计算
    ///
    /// # 示例
    /// ```
    /// use poker_landlord_rs::card::Cards;
    /// use poker_landlord_rs::card::Card;
    /// use poker_landlord_rs::card::rank::Rank;
    /// use poker_landlord_rs::card::suit::Suit;
    ///
    /// let cards = Cards::from_vec(vec![
    ///     Card::new(Rank::Three, Suit::Spades),
    ///     Card::new(Rank::Three, Suit::Hearts),
    ///     Card::new(Rank::Five, Suit::Diamonds),
    /// ]);
    /// let counts = cards.rank_counts();
    /// assert_eq!(counts[&Rank::Three], 2);
    /// assert_eq!(counts[&Rank::Five], 1);
    /// ```
    pub fn rank_counts(&self) -> HashMap<Rank, u8> {
        self.make_rank_counts()
    }

    /// 从当前集合中移除另一个集合中的所有卡牌，返回剩余的卡牌
    ///
    /// 常用于玩家出牌后更新手牌
    ///
    /// # 参数
    /// * `played` - 要移除的卡牌集合
    ///
    /// # 返回值
    /// - 如果当前集合包含所有要移除的卡牌，返回 Some(remaining_cards)
    /// - 否则返回 None（表示卡牌不足）
    ///
    /// # 时间复杂度
    /// O(n + m) - 线性时间复杂度
    ///
    /// # 示例
    /// ```rust
    /// use poker_landlord_rs::card::Cards;
    ///
    /// let hand = Cards::from_vec(vec![/* ... */]);
    /// let played = Cards::from_vec(vec![/* ... */]);
    /// if let Some(remaining) = hand.subtract(&played) {
    ///     // 出牌成功，remaining 是剩余手牌
    /// }
    /// ```
    pub fn subtract(&self, played: &Cards) -> Option<Cards> {
        if played.is_empty() {
            return Some(self.clone());
        }
        if self.len() < played.len() {
            return None;
        }

        // 一次性计算，避免重复遍历
        let mut self_counts = self.make_counts();
        let mut to_remove_counts = HashMap::with_capacity(played.len());

        // 统计要移除的卡牌
        for card in &played.0 {
            *to_remove_counts.entry(*card).or_insert(0) += 1;
        }

        // 检查并移除
        for (card, remove_count) in to_remove_counts {
            if let Some(current) = self_counts.get_mut(&card) {
                if *current < remove_count {
                    return None; // 卡牌不足
                }
                *current -= remove_count;
            } else {
                return None; // 卡牌不存在
            }
        }

        // 构建剩余卡牌
        let mut remaining = Self::with_capacity(self.len());
        for (card, count) in self_counts {
            for _ in 0..count {
                remaining.0.push(card);
            }
        }
        Some(remaining)
    }

    /// 移除单张指定的卡牌
    ///
    /// 只删除第一个匹配的卡牌（按集合中的顺序）
    ///
    /// # 返回值
    /// 如果找到并删除了卡牌返回 Some(card)，否则返回 None
    pub fn remove_one(&mut self, card: Card) -> Option<Card> {
        self.0
            .iter()
            .position(|c| c == &card)
            .map(|pos| self.0.remove(pos))
    }

    /// 判断集合是否包含指定的单张卡牌
    ///
    /// # 示例
    /// ```
    /// use poker_landlord_rs::card::Cards;
    /// use poker_landlord_rs::card::Card;
    /// use poker_landlord_rs::card::Rank;
    /// use poker_landlord_rs::card::Suit;
    ///
    /// let cards = Cards::from_vec(vec![
    ///     Card::new(Rank::Three, Suit::Spades),
    /// ]);
    /// assert!(cards.contains(&Card::new(Rank::Three, Suit::Spades)));
    /// ```
    pub fn contains(&self, card: &Card) -> bool {
        self.0.iter().any(|c| c == card)
    }
}

/// Deref trait - 允许直接访问内部的 Vec<Card>
///
/// 这允许 Cards 像 Vec 一样使用索引访问等功能。
/// 自动解引用使得外部调用时可以直接使用 Vec 的方法。
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let cards = Cards::from_vec(vec![
///     Card::new(Rank::Three, Suit::Spades),
/// ]);
///
/// // 自动解引用为 Vec<Card>，可以调用 Vec 的方法
/// let len = cards.len();
/// let is_empty = cards.is_empty();
/// ```
impl std::ops::Deref for Cards {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// DerefMut trait - 允许可变访问内部的 Vec<Card>
///
/// 这允许 Cards 像 Vec 一样进行可变修改。
/// 支持 push, pop, clear 等修改操作。
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let mut cards = Cards::new();
/// cards.push(Card::new(Rank::Three, Suit::Spades));
/// ```
impl std::ops::DerefMut for Cards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use std::fmt;

/// Display trait - 用于打印卡牌集合
///
/// 格式：空格分隔的卡牌字符串（例：♠3 ♥3 ♦5 ♣K）
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let cards = Cards::from_vec(vec![
///     Card::new(Rank::Three, Suit::Spades),
///     Card::new(Rank::Three, Suit::Hearts),
/// ]);
/// println!("{}", cards);  // 输出: ♠3 ♥3
/// ```
impl fmt::Display for Cards {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, card) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", card)?;
        }
        Ok(())
    }
}

/// IntoIterator trait (owned) - 允许 for card in cards { ... }
///
/// 消费 Cards 获取所有权的迭代器。
/// 迭代完成后 Cards 将不可再使用。
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let cards = Cards::from_vec(vec![
///     Card::new(Rank::Three, Suit::Spades),
/// ]);
///
/// // 消费所有权
/// for card in cards {
///     println!("{:?}", card);
/// }
/// ```
impl IntoIterator for Cards {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// IntoIterator trait (immutable reference) - 允许 for card in &cards { ... }
///
/// 借用迭代，不获取所有权。
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let cards = Cards::from_vec(vec![
///     Card::new(Rank::Three, Suit::Spades),
/// ]);
///
/// // 借用引用
/// for card in &cards {
///     println!("{:?}", card);
/// }
/// ```
impl<'a> IntoIterator for &'a Cards {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// IntoIterator trait (mutable reference) - 允许 for card in &mut cards { ... }
///
/// 可变借用迭代，可以修改元素。
impl<'a> IntoIterator for &'a mut Cards {
    type Item = &'a mut Card;
    type IntoIter = std::slice::IterMut<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

/// FromIterator trait - 允许使用 collect() 从迭代器创建 Cards
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let cards: Cards = vec![
///     Card::new(Rank::Three, Suit::Spades),
///     Card::new(Rank::Three, Suit::Hearts),
/// ].into_iter().collect();
/// ```
impl std::iter::FromIterator<Card> for Cards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

/// Index trait - 允许使用索引访问 cards[index]
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let cards = Cards::from_vec(vec![
///     Card::new(Rank::Three, Suit::Spades),
///     Card::new(Rank::Three, Suit::Hearts),
/// ]);
///
/// let first = cards[0];  // ♠3
/// ```
impl std::ops::Index<usize> for Cards {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// IndexMut trait - 允许使用索引修改 cards[index] = card
///
/// # 示例
/// ```
/// use poker_landlord_rs::card::Cards;
/// use poker_landlord_rs::card::Card;
/// use poker_landlord_rs::card::Rank;
/// use poker_landlord_rs::card::Suit;
///
/// let mut cards = Cards::from_vec(vec![
///     Card::new(Rank::Three, Suit::Spades),
/// ]);
///
/// cards[0] = Card::new(Rank::Five, Suit::Spades);
/// ```
impl std::ops::IndexMut<usize> for Cards {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    // ===== 辅助函数 =====

    fn spades(rank: Rank) -> Card {
        Card::new(rank, Suit::Spades)
    }

    fn hearts(rank: Rank) -> Card {
        Card::new(rank, Suit::Hearts)
    }

    fn diamonds(rank: Rank) -> Card {
        Card::new(rank, Suit::Diamonds)
    }

    fn clubs(rank: Rank) -> Card {
        Card::new(rank, Suit::Clubs)
    }

    // ===== 构造函数测试 =====

    #[test]
    fn test_new() {
        let cards = Cards::new();
        assert!(cards.is_empty());
        assert_eq!(cards.len(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let cards = Cards::with_capacity(54);
        assert_eq!(cards.len(), 0);
        assert!(cards.capacity() >= 54);
    }

    #[test]
    fn test_from_vec() {
        let cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        assert_eq!(cards.len(), 2);
    }

    #[test]
    fn test_from_slice() {
        let original = vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ];
        let cards = Cards::from_slice(&original);
        assert_eq!(cards.len(), 2);
        // 确保是深拷贝
        assert_eq!(cards[0], original[0]);
    }

    // ===== contains_all 测试 =====

    #[test]
    fn test_contains_all_basic() {
        let all = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
            spades(Rank::Five),
            clubs(Rank::King),
        ]);

        let subset = Cards::from_vec(vec![
            spades(Rank::Three),
            spades(Rank::Five),
        ]);
        assert!(all.contains_all(&subset));
    }

    #[test]
    fn test_contains_all_not_contained() {
        let all = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        let not_contained = Cards::from_vec(vec![spades(Rank::Ace)]);
        assert!(!all.contains_all(&not_contained));
    }

    #[test]
    fn test_contains_all_empty() {
        let cards = Cards::from_vec(vec![spades(Rank::Three)]);
        assert!(cards.contains_all(&Cards::new()));
    }

    #[test]
    fn test_contains_all_self() {
        let cards = Cards::from_vec(vec![spades(Rank::Three)]);
        assert!(cards.contains_all(&cards));
    }

    #[test]
    fn test_contains_all_insufficient_count() {
        // 有两张♠3，但检查三张♠3
        let all = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        let subset = Cards::from_vec(vec![
            spades(Rank::Three),
            spades(Rank::Three),
            spades(Rank::Three),
        ]);
        assert!(!all.contains_all(&subset));
    }

    #[test]
    fn test_contains_all_larger_than_self() {
        let small = Cards::from_vec(vec![spades(Rank::Three)]);
        let large = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        assert!(!small.contains_all(&large));
    }

    // ===== count_rank 测试 =====

    #[test]
    fn test_count_rank_basic() {
        let cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
            spades(Rank::Five),
        ]);
        assert_eq!(cards.count_rank(Rank::Three), 2);
        assert_eq!(cards.count_rank(Rank::Five), 1);
        assert_eq!(cards.count_rank(Rank::King), 0);
    }

    #[test]
    fn test_count_rank_empty() {
        let cards = Cards::new();
        assert_eq!(cards.count_rank(Rank::Three), 0);
    }

    // ===== subtract 测试 =====

    #[test]
    fn test_subtract_basic() {
        let hand = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
            spades(Rank::Five),
            clubs(Rank::King),
        ]);
        let played = Cards::from_vec(vec![
            spades(Rank::Three),
            spades(Rank::Five),
        ]);

        let remaining = hand.subtract(&played);
        assert!(remaining.is_some());
        let remaining = remaining.unwrap();
        assert_eq!(remaining.len(), 2);
        assert!(remaining.contains(&hearts(Rank::Three)));
        assert!(remaining.contains(&clubs(Rank::King)));
    }

    #[test]
    fn test_subtract_all() {
        let hand = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        let played = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);

        let remaining = hand.subtract(&played);
        assert!(remaining.is_some());
        assert!(remaining.unwrap().is_empty());
    }

    #[test]
    fn test_subtract_not_enough() {
        let hand = Cards::from_vec(vec![spades(Rank::Three)]);
        let played = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);

        let remaining = hand.subtract(&played);
        assert!(remaining.is_none());
    }

    #[test]
    fn test_subtract_empty() {
        let hand = Cards::from_vec(vec![spades(Rank::Three)]);
        let remaining = hand.subtract(&Cards::new());
        assert!(remaining.is_some());
        assert_eq!(remaining.unwrap().len(), 1);
    }

    #[test]
    fn test_subtract_preserves_remaining() {
        // 测试剩余卡牌是否正确（包含重复卡牌的情况）
        let hand = Cards::from_vec(vec![
            spades(Rank::Three),
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        let played = Cards::from_vec(vec![spades(Rank::Three)]);

        let remaining = hand.subtract(&played).unwrap();
        assert_eq!(remaining.len(), 2);
        assert_eq!(remaining.count_rank(Rank::Three), 2);
    }

    // ===== remove_one 测试 =====

    #[test]
    fn test_remove_one_basic() {
        let mut cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
            spades(Rank::Five),
        ]);

        let removed = cards.remove_one(spades(Rank::Three));
        assert!(removed.is_some());
        assert_eq!(removed.unwrap(), spades(Rank::Three));
        assert_eq!(cards.len(), 2);
    }

    #[test]
    fn test_remove_one_not_found() {
        let mut cards = Cards::from_vec(vec![spades(Rank::Three)]);

        let removed = cards.remove_one(spades(Rank::Five));
        assert!(removed.is_none());
        assert_eq!(cards.len(), 1);
    }

    #[test]
    fn test_remove_one_first_only() {
        // 只删除第一个匹配的
        let mut cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
            spades(Rank::Three),
        ]);

        cards.remove_one(spades(Rank::Three));
        assert_eq!(cards.len(), 2);
        // 应该还剩一张♠3
        assert!(cards.iter().filter(|c| c.rank == Rank::Three).count() == 2);
    }

    // ===== sort 测试 =====

    #[test]
    fn test_sort() {
        let mut cards = Cards::from_vec(vec![
            clubs(Rank::King),
            spades(Rank::Three),
            diamonds(Rank::Five),
            hearts(Rank::Three),
        ]);

        cards.sort();

        // 排序后：Three < Five < King
        assert_eq!(cards[0].rank, Rank::Three);
        assert_eq!(cards[1].rank, Rank::Three);
        assert_eq!(cards[2].rank, Rank::Five);
        assert_eq!(cards[3].rank, Rank::King);
    }

    #[test]
    fn test_sort_same_rank_different_suit() {
        // 相同点数应该按花色排序
        // Suit 的排序顺序：Diamonds(1) < Clubs(2) < Hearts(3) < Spades(4)
        let mut cards = Cards::from_vec(vec![
            hearts(Rank::Three),
            spades(Rank::Three),
            clubs(Rank::Three),
            diamonds(Rank::Three),
        ]);

        cards.sort();

        // 花色顺序：Diamonds < Clubs < Hearts < Spades
        assert_eq!(cards[0].suit, Some(Suit::Diamonds));
        assert_eq!(cards[1].suit, Some(Suit::Clubs));
        assert_eq!(cards[2].suit, Some(Suit::Hearts));
        assert_eq!(cards[3].suit, Some(Suit::Spades));
    }

    #[test]
    fn test_sorted() {
        let cards = Cards::from_vec(vec![
            clubs(Rank::King),
            spades(Rank::Three),
        ]);

        let sorted = cards.sorted();
        assert_eq!(sorted[0].rank, Rank::Three);
        assert_eq!(sorted[1].rank, Rank::King);
    }

    // ===== 迭代器测试 =====

    #[test]
    fn test_into_iter_owned() {
        let cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        let count = cards.into_iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_into_iter_ref() {
        let cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        let count = cards.iter().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_into_iter_mut() {
        let mut cards = Cards::from_vec(vec![spades(Rank::Three)]);
        for card in &mut cards {
            let _ = card;
        }
    }

    #[test]
    fn test_for_loop_iteration() {
        let cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
            spades(Rank::Five),
        ]);

        let mut count = 0;
        for _ in &cards {
            count += 1;
        }
        assert_eq!(count, 3);
    }

    // ===== Index 访问测试 =====

    #[test]
    fn test_index_access() {
        let cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        assert_eq!(cards[0].rank, Rank::Three);
        assert_eq!(cards[1].rank, Rank::Three);
    }

    #[test]
    fn test_index_mut_access() {
        let mut cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        cards[0] = spades(Rank::Five);
        assert_eq!(cards[0].rank, Rank::Five);
    }

    // ===== FromIterator 测试 =====

    #[test]
    fn test_from_iterator() {
        let cards: Cards = vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ].into_iter().collect();

        assert_eq!(cards.len(), 2);
    }

    // ===== Display 测试 =====

    #[test]
    fn test_display() {
        let cards = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
        ]);
        let s = format!("{}", cards);
        assert!(!s.is_empty());
    }

    // ===== 边界情况测试 =====

    #[test]
    fn test_empty_contains_all() {
        let empty = Cards::new();
        let cards = Cards::from_vec(vec![spades(Rank::Three)]);
        // 空集合不包含任何非空集合（元素数量不足）
        assert!(!empty.contains_all(&cards));
    }

    #[test]
    fn test_empty_subtract() {
        let empty = Cards::new();
        let result = empty.subtract(&Cards::from_vec(vec![spades(Rank::Three)]));
        assert!(result.is_none());
    }

    #[test]
    fn test_single_card_operations() {
        let cards = Cards::from_vec(vec![spades(Rank::Three)]);

        assert_eq!(cards.len(), 1);
        assert!(!cards.is_empty());
        assert_eq!(cards.count_rank(Rank::Three), 1);
    }

    // ===== 潜在 Bug 探索 =====

    #[test]
    fn test_bug_exploration_duplicates_in_contains_all() {
        // 测试 contains_all 对重复卡牌的处理
        let all = Cards::from_vec(vec![
            spades(Rank::Three),
            spades(Rank::Three), // 两张♠3
        ]);
        let subset = Cards::from_vec(vec![
            spades(Rank::Three),
            spades(Rank::Three),
            spades(Rank::Three), // 三张♠3
        ]);

        // 应该返回 false，因为只有两张♠3
        assert!(!all.contains_all(&subset));
    }

    #[test]
    fn test_bug_exploration_subtract_preserves_order() {
        // 测试 subtract 后剩余卡牌的数量和内容
        // 注意：使用 HashMap 后不保证顺序，只保证内容正确
        let hand = Cards::from_vec(vec![
            spades(Rank::Three),
            hearts(Rank::Three),
            spades(Rank::Five),
        ]);
        let played = Cards::from_vec(vec![spades(Rank::Three)]);

        let remaining = hand.subtract(&played).unwrap();
        assert_eq!(remaining.len(), 2);
        // 不检查顺序，只检查内容
        assert!(remaining.contains(&hearts(Rank::Three)));
        assert!(remaining.contains(&spades(Rank::Five)));
    }

    #[test]
    fn test_bug_exploration_joker_handling() {
        // 测试王牌的处理
        let cards = Cards::from_vec(vec![
            Card::joker(false), // 大王
            Card::joker(true),  // 小王
            spades(Rank::Three),
        ]);

        assert_eq!(cards.len(), 3);
        assert_eq!(cards.count_rank(Rank::JokerBig), 1);
        assert_eq!(cards.count_rank(Rank::JokerSmall), 1);
    }

    #[test]
    fn test_bug_exploration_full_deck() {
        // 测试一副完整的牌（54张）
        // 一副牌 = 52张普通牌 (13点数 × 4花色) + 2张王牌
        let mut deck = Cards::with_capacity(54);

        // 添加所有普通牌
        for rank in Rank::ALL.iter() {
            if rank.is_joker() {
                continue;
            }
            for suit in Suit::ALL.iter() {
                deck.push(Card::new(*rank, *suit));
            }
        }

        // 添加大小王
        deck.push(Card::joker(false));
        deck.push(Card::joker(true));

        // 52 + 2 = 54
        assert_eq!(deck.len(), 54);
    }
}
