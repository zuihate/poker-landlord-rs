//! 卡牌集合的定义及相关操作
//!
//! 用于表示和操作一组卡牌，例如玩家的手牌、出牌等。
//! 内部使用 Vec 存储，外部包装以实现 Deref 和多种迭代器特性。

use std::collections::BTreeMap;

use super::Card;
use super::Rank;

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

    /// 从 Vec<Card> 创建卡牌集合
    pub fn from_vec(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    /// 从卡牌切片创建卡牌集合
    pub fn from_slice(cards: &[Card]) -> Self {
        Self(cards.to_vec())
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

    /// 获取集合中的卡牌数量
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// 判断集合是否为空
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// 清空集合中的所有卡牌
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// 向集合中添加一张卡牌
    pub fn push(&mut self, card: Card) {
        self.0.push(card);
    }

    /// 向集合中添加多张卡牌
    pub fn extend(&mut self, other: Cards) {
        self.0.extend(other.0);
    }

    /// 判断集合中是否包含某张卡牌
    pub fn contains(&self, card: Card) -> bool {
        self.0.contains(&card)
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

        let mut self_count = BTreeMap::new();
        for card in &self.0 {
            *self_count.entry(card).or_insert(0) += 1;
        }

        for card in &other.0 {
            if let Some(cnt) = self_count.get_mut(card) {
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
        self.0.iter().filter(|c| c.rank == rank).count() as u8
    }

    /// 获取卡牌集合的不可变迭代器
    pub fn iter(&self) -> std::slice::Iter<'_, Card> {
        self.0.iter()
    }

    /// 获取卡牌集合的可变迭代器
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Card> {
        self.0.iter_mut()
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
    /// O(n) - 线性时间复杂度
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
        if !self.contains_all(played) {
            return None;
        }
        let mut remaining = self.clone();
        let mut to_remove = std::collections::BTreeMap::new();
        for card in &played.0 {
            *to_remove.entry(card).or_insert(0) += 1;
        }
        remaining.0.retain(|card| {
            if let Some(cnt) = to_remove.get_mut(card)
                && *cnt > 0
            {
                *cnt -= 1;
                return false;
            }
            true
        });
        Some(remaining)
    }

    /// 按点数分组统计卡牌
    ///
    /// 返回 (点数, 数量) 的向量，按点数从小到大排序
    ///
    /// 用于判断炸弹（4张同点数）、顺子、飞机等复杂牌型
    ///
    /// # 例子
    /// ```
    /// use poker_landlord_rs::card::Card;
    /// use poker_landlord_rs::card::Cards;
    /// use poker_landlord_rs::card::rank::Rank;
    /// use poker_landlord_rs::card::suit::Suit;
    ///
    /// let cards = Cards::from_vec(vec![
    ///     Card::new(Rank::Three, Suit::Spades),
    ///     Card::new(Rank::Three, Suit::Hearts),
    ///     Card::new(Rank::Five, Suit::Diamonds),
    /// ]);
    /// let groups = cards.group_by_rank();
    /// // groups = [(Three, 2), (Five, 1)]
    /// ```
    pub fn group_by_rank(&self) -> Vec<(Rank, u8)> {
        let mut map = std::collections::BTreeMap::new();
        for card in &self.0 {
            *map.entry(card.rank).or_insert(0) += 1;
        }
        map.into_iter().collect()
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

    /// 获取第一张卡牌的不可变引用
    ///
    /// # 返回值
    /// 如果集合非空返回 Some(&card)，否则返回 None
    pub fn first(&self) -> Option<&Card> {
        self.0.first()
    }

    /// 移除并返回第一张卡牌
    ///
    /// 常用于获取最后一张牌
    ///
    /// # 返回值
    /// 如果集合非空返回 Some(card)，否则返回 None
    pub fn pop_first(&mut self) -> Option<Card> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

/// Deref trait - 允许直接访问内部的 Vec<Card>
///
/// 这允许 Cards 像 Vec 一样使用索引访问等功能
impl std::ops::Deref for Cards {
    type Target = Vec<Card>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// DerefMut trait - 允许可变访问内部的 Vec<Card>
impl std::ops::DerefMut for Cards {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use std::fmt;

/// Display trait - 用于打印卡牌集合
///
/// 格式：空格分隔的卡牌字符串（例：♠3 ♥3 ♦5 ♣K）
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
impl IntoIterator for Cards {
    type Item = Card;
    type IntoIter = std::vec::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// IntoIterator trait (immutable reference) - 允许 for card in &cards { ... }
impl<'a> IntoIterator for &'a Cards {
    type Item = &'a Card;
    type IntoIter = std::slice::Iter<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Cards {
    type Item = &'a mut Card;
    type IntoIter = std::slice::IterMut<'a, Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl std::iter::FromIterator<Card> for Cards {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl std::ops::Index<usize> for Cards {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Cards {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Suit;

    fn create_test_cards() -> Cards {
        Cards::from_vec(vec![
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::King, Suit::Clubs),
        ])
    }

    #[test]
    fn test_contains_all() {
        let all = create_test_cards();
        let subset = Cards::from_vec(vec![
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Five, Suit::Diamonds),
        ]);
        assert!(all.contains_all(&subset));

        let not_contained = Cards::from_vec(vec![Card::new(Rank::Ace, Suit::Spades)]);
        assert!(!all.contains_all(&not_contained));
    }

    #[test]
    fn test_subtract() {
        let all = create_test_cards();
        let to_remove = Cards::from_vec(vec![
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Five, Suit::Diamonds),
        ]);

        let remaining = all.subtract(&to_remove).unwrap();
        assert_eq!(remaining.len(), 2);
        assert!(remaining.contains(Card::new(Rank::Three, Suit::Hearts)));
        assert!(remaining.contains(Card::new(Rank::King, Suit::Clubs)));
    }

    #[test]
    fn test_remove_one() {
        let mut cards = create_test_cards();
        let card_to_remove = Card::new(Rank::Five, Suit::Diamonds);

        assert!(cards.remove_one(card_to_remove).is_some());
        assert!(!cards.contains(card_to_remove));
        assert_eq!(cards.len(), 3);
    }

    #[test]
    fn test_pop_first() {
        let mut cards = create_test_cards();
        let first = cards.pop_first();
        assert!(first.is_some());
        assert_eq!(cards.len(), 3);
    }

    #[test]
    fn test_group_by_rank() {
        let cards = create_test_cards();
        let grouped = cards.group_by_rank();

        assert_eq!(grouped.len(), 3); // Three, Five, King
        assert!(grouped.iter().any(|(r, c)| *r == Rank::Three && *c == 2));
        assert!(grouped.iter().any(|(r, c)| *r == Rank::Five && *c == 1));
    }

    #[test]
    fn test_into_iter() {
        let cards = create_test_cards();
        let count = cards.iter().count();
        assert_eq!(count, 4);
    }

    #[test]
    fn test_index_access() {
        let cards = create_test_cards();
        assert_eq!(cards[0].rank, Rank::Three);
    }
}
