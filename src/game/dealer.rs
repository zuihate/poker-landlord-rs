use crate::card::{Card, Cards, Rank, Suit};

/// 代表一副完整的牌堆
///
/// 包含54张扑克牌（13个点数 × 4种花色 + 2张Joker）。
/// 用于游戏初始化时进行洗牌和发牌操作。n///
/// # 牌堆构成
///
/// - 普通牌：52张（13个点数 × 4种花色）
/// - Joker牌：2张（大小王各1张）
/// - 总计：54张
///
/// # 使用示例
///
/// ```ignore
/// use poker_landlord_rs::game::dealer::Deck;
///
/// let mut deck = Deck::new();
/// deck.shuffle();  // 洗牌
/// // 之后可以从 deck.cards 中取牌进行发牌
/// ```
pub struct Deck {
    /// 54张牌的集合
    pub cards: Cards,
}

use rand::seq::SliceRandom;
use rand::thread_rng;

impl Deck {
    /// 创建一个新的牌堆，包含54张牌（52张普通牌和2张 Joker）
    ///
    /// 新创建的牌堆按点数和花色顺序排列，尚未洗牌。
    /// 调用 `shuffle()` 方法可以打乱牌堆的顺序。
    ///
    /// # 牌堆初始排列
    ///
    /// 1. 按点数从3到A依次排列
    /// 2. 每个点数按黑桃、红心、方块、梅花顺序排列
    /// 3. 最后添加小王、大王各1张
    ///
    /// # 返回值
    ///
    /// 返回一个新的 `Deck` 实例
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let deck = Deck::new();
    /// assert_eq!(deck.cards.len(), 54);
    /// ```
    pub fn new() -> Self {
        // 创建一个标准的54张牌的牌堆
        let mut cards = Cards::with_capacity(54);

        // 牌的花色和点数
        let suits = Suit::ALL;
        let ranks = Rank::ALL;

        // 生成牌堆
        for &rank in &ranks {
            if rank.is_joker() {
                continue;
            }
            for &suit in &suits {
                cards.push(Card {
                    rank,
                    suit: Some(suit),
                });
            }
        }
        // 添加两张 Joker
        cards.push(Card {
            rank: Rank::JokerBig,
            suit: None,
        });
        cards.push(Card {
            rank: Rank::JokerSmall,
            suit: None,
        });

        Self { cards }
    }

    /// 洗牌函数，使用随机数生成器打乱牌堆的顺序
    ///
    /// 使用 `rand::seq::SliceRandom` 的 `shuffle()` 方法进行洗牌，
    /// 采用 Fisher-Yates 算法确保随机性。
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let mut deck = Deck::new();
    /// let first_card_before = deck.cards[0].clone();
    /// deck.shuffle();
    /// // 通常情况下，洗牌后第一张牌会改变
    /// ```
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

/// 游戏初始发牌结果
///
/// 包含给三个玩家各自的17张底牌以及3张地主牌。
/// 通常由 `InitialDeal::new()` 生成，包含了整个游戏开始前的所有必要信息。n///
/// # 发牌规则
///
/// - 每个玩家获得17张牌
/// - 地主牌（底牌）为3张
/// - 总计 17×3 + 3 = 54张牌
///
/// # 字段
///
/// - `player_hands` - 数组，每个元素是一个玩家的17张底牌
/// - `landlord_cards` - 地主可以获得的3张底牌
///
/// # 使用示例
///
/// ```ignore
/// use poker_landlord_rs::game::dealer::InitialDeal;
///
/// let deal = InitialDeal::new();
/// println!("玩家1的牌: {}", deal.player_hands[0]);
/// println!("地主牌: {}", deal.landlord_cards);
/// ```
pub struct InitialDeal {
    /// 3个玩家的初始手牌，每个玩家17张
    pub player_hands: [Cards; 3],
    /// 地主底牌，共3张
    pub landlord_cards: Cards,
}

impl InitialDeal {
    /// 生成一次初始发牌
    ///
    /// 该方法会：
    /// 1. 创建一副54张牌的牌堆
    /// 2. 对牌堆进行洗牌
    /// 3. 将牌分配给3个玩家（每个17张）和地主底牌（3张）
    /// 4. 对每个玩家的手牌进行排序
    ///
    /// # 返回值
    ///
    /// 返回一个 `InitialDeal` 实例，包含所有玩家的初始手牌和地主底牌
    ///
    /// # 示例
    ///
    /// ```ignore
    /// let deal = InitialDeal::new();
    /// assert_eq!(deal.player_hands.len(), 3);
    /// assert_eq!(deal.player_hands[0].len(), 17);
    /// assert_eq!(deal.landlord_cards.len(), 3);
    /// ```
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let player_hands: [Cards; 3] = [
            Cards::from_slice(&deck.cards.as_slice()[0..17]),
            Cards::from_slice(&deck.cards.as_slice()[17..34]),
            Cards::from_slice(&deck.cards.as_slice()[34..51]),
        ];
        let landlord_cards = Cards::from_slice(&deck.cards.as_slice()[51..54]);
        let mut deal = Self {
            player_hands,
            landlord_cards,
        };

        for hand in &mut deal.player_hands {
            hand.sort();
        }
        deal.landlord_cards.sort();

        deal
    }
}

impl Default for InitialDeal {
    fn default() -> Self {
        Self::new()
    }
}

use std::fmt;

/// 为 `InitialDeal` 实现 `Display` trait
///
/// 用于将发牌结果以易读的格式输出到终端。
/// 依次显示三个玩家的初始手牌和地主底牌。
impl fmt::Display for InitialDeal {
    /// 将发牌结果格式化为可读的字符串
    ///
    /// 输出格式为：
    /// ```text
    /// 玩家1的牌: [牌列表]
    /// 玩家2的牌: [牌列表]
    /// 玩家3的牌: [牌列表]
    /// 地主牌: [牌列表]
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "玩家1的牌: {}", self.player_hands[0])?;
        writeln!(f, "玩家2的牌: {}", self.player_hands[1])?;
        writeln!(f, "玩家3的牌: {}", self.player_hands[2])?;
        writeln!(f, "地主牌: {}", self.landlord_cards)
    }
}
