use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

use crate::card::Card;
use crate::card::cards::Cards;
use crate::card::rank::Rank;
use crate::card::suit::Suit;

pub struct Deck {
    cards: Cards,
}
impl Deck {
    /// 创建一个新的牌堆，包含54张牌（52张普通牌和2张 Joker）
    pub fn new() -> Self {
        // 创建一个标准的54张牌的牌堆
        let mut card_vec = Vec::with_capacity(54);
        // 牌的花色和点数
        let suits = [Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds];
        let ranks = [
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
        ];

        // 生成牌堆
        for &rank in &ranks {
            for &suit in &suits {
                card_vec.push(Card {
                    rank,
                    suit: Some(suit),
                });
            }
        }
        // 添加两张 Joker
        card_vec.push(Card {
            rank: Rank::JokerBig,
            suit: None,
        });
        card_vec.push(Card {
            rank: Rank::JokerSmall,
            suit: None,
        });

        let cards = Cards::from_vec(card_vec);
        Self { cards }
    }

    /// 洗牌函数，使用随机数生成器打乱牌堆的顺序
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

pub struct InitialDeal {
    pub player_hands: [Cards; 3],
    pub landlord_cards: Cards,
}

impl InitialDeal {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        // 将牌堆转换为 Vec 以便分配
        let all_cards: Vec<Card> = deck.cards.iter().cloned().collect();

        let player_hands: [Cards; 3] = [
            Cards::from_slice(&all_cards[0..17]),
            Cards::from_slice(&all_cards[17..34]),
            Cards::from_slice(&all_cards[34..51]),
        ];
        let landlord_cards = Cards::from_slice(&all_cards[51..54]);

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

impl fmt::Display for InitialDeal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "玩家1的牌: {}", self.player_hands[0])?;
        writeln!(f, "玩家2的牌: {}", self.player_hands[1])?;
        writeln!(f, "玩家3的牌: {}", self.player_hands[2])?;
        writeln!(f, "地主牌: {}", self.landlord_cards)
    }
}
