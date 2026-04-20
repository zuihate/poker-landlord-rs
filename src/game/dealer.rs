use crate::card::{Card, Cards, Rank, Suit};

pub struct Deck {
    pub cards: Cards,
}

use rand::seq::SliceRandom;
use rand::thread_rng;

impl Deck {
    /// 创建一个新的牌堆，包含54张牌（52张普通牌和2张 Joker）
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

impl fmt::Display for InitialDeal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "玩家1的牌: {}", self.player_hands[0])?;
        writeln!(f, "玩家2的牌: {}", self.player_hands[1])?;
        writeln!(f, "玩家3的牌: {}", self.player_hands[2])?;
        writeln!(f, "地主牌: {}", self.landlord_cards)
    }
}
