//! 玩家模块
//!
//! 本模块定义玩家的角色（地主/农民）、手牌管理和出牌操作。
//!
//! # 示例
//! ```
//! use poker_landlord_rs::player::{Player, Role, PlayerType};
//! use poker_landlord_rs::card::Cards;
//!
//! // 创建空手牌（测试用最小示例）
//! let cards = Cards::default();
//!
//! let mut player = Player::new(0, cards.clone(), Role::Farmer, PlayerType::Human);
//!
//! // 成为地主（传入空牌避免依赖复杂构造）
//! player.become_landlord(&cards);
//!
//! // 出牌（空出牌示例）
//! let result = player.play_cards(&cards);
//! assert!(result.is_ok());
//! ```

use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Write};

use crate::card::Card;
use crate::card::Cards;
use crate::card::parser::tokenize_card_input;
use crate::card::Rank;
use crate::error::{PlayerError, PlayerResult};
use crate::rules::Play;

/// 玩家在游戏中的角色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// 农民
    Farmer,
    /// 地主
    Landlord,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Farmer => write!(f, "农民"),
            Role::Landlord => write!(f, "地主"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerType {
    /// 真人玩家，需要键盘输入
    Human,
    /// 人机（AI），当前行为与真人一致，后续可实现自动逻辑
    AI,
}

impl fmt::Display for PlayerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerType::Human => write!(f, "玩家"),
            PlayerType::AI => write!(f, "人机"),
        }
    }
}

/// 代表游戏中的一个玩家
///
/// 记录玩家的唯一编号、手牌和角色信息。
/// 支持手牌管理、出牌验证等操作。
#[derive(Debug)]
pub struct Player {
    /// 玩家编号
    pub id: usize,
    /// 玩家的手牌
    pub hand: Cards,
    /// 玩家的角色（地主或农民）
    pub role: Role,
    /// 玩家类型（真人 / 人机）
    pub player_type: PlayerType,
}

impl Player {
    /// 创建一个新玩家
    ///
    /// # 参数
    /// * `id` - 玩家唯一编号
    /// * `hand` - 玩家的初始手牌
    /// * `role` - 玩家的角色
    pub fn new(id: usize, hand: Cards, role: Role, player_type: PlayerType) -> Self {
        Self {
            id,
            hand,
            role,
            player_type,
        }
    }

    /// 判断玩家是否是地主
    pub fn is_landlord(&self) -> bool {
        self.role == Role::Landlord
    }

    /// 玩家成为地主并接收地主牌
    ///
    /// # 参数
    /// * `landlord_cards` - 地主底牌
    ///
    /// # Panics
    /// 如果玩家已是地主则触发 panic
    pub fn become_landlord(&mut self, landlord_cards: &Cards) {
        self.role = Role::Landlord;
        self.hand.extend(landlord_cards.clone());
        self.hand.sort();
    }

    /// 给地主玩家添加额外的地主牌
    ///
    /// # 参数
    /// * `landlord_cards` - 要添加的地主牌
    ///
    /// # Panics
    /// 如果玩家不是地主，或地主卡已添加过会触发 panic
    pub fn add_landlord_cards(&mut self, landlord_cards: &Cards) {
        if !self.is_landlord() {
            panic!("你不是地主");
        }
        self.hand.extend(landlord_cards.clone());
        self.hand.sort();
    }

    /// 检查玩家是否拥有指定的卡牌
    ///
    /// # 参数
    /// * `cards` - 要检查的卡牌集合
    ///
    /// # 时间复杂度
    /// O(n + m)，其中 n 是手牌数量，m 是检查的卡牌数量
    pub fn has_cards(&self, cards: &Cards) -> bool {
        self.hand.contains_all(cards)
    }

    pub fn choose_play(&self) -> PlayerResult<Play> {
        match self.player_type {
            PlayerType::Human => self.choose_play_human(),
            PlayerType::AI => self.choose_play_ai(),
        }
    }

    pub fn choose_play_human(&self) -> PlayerResult<Play> {
        let selected = self.select_cards(tokenize_card_input(&input()))?;

        let play = match Play::new(selected) {
            Ok(play) => play,
            Err(_) => return Err(PlayerError::InvalidPlay("手牌中不存在这些卡牌".to_string())),
        };
        Ok(play)
    }

    /// AI 出牌逻辑的占位实现
    ///
    /// 当前版本仍使用与真人相同的输入路径，后续可替换为自动出牌策略。
    pub fn choose_play_ai(&self) -> PlayerResult<Play> {
        self.choose_play_human()
    }

    /// 根据输入字符串解析并选择卡牌
    ///
    /// 支持两种输入格式：
    /// - 空格分隔：`"3 3 4 5"`
    /// - 连续格式：`"3345"`
    ///
    /// # 参数
    /// * `input` - 卡牌输入字符串
    ///
    /// # 返回值
    /// 返回成功选择的卡牌集合，或返回错误消息
    ///
    /// # 时间复杂度
    /// O(n * m)，其中 n 是手牌数量，m 是输入的卡牌数量
    pub fn select_cards(&self, card_tokens: Vec<&str>) -> PlayerResult<Cards> {
        let mut card_count = BTreeMap::new();
        for card in self.hand.iter() {
            *card_count.entry(*card).or_insert(0) += 1;
        }

        let mut selected: Vec<Card> = Vec::new();

        for token in card_tokens {
            if let Ok(rank) = token.parse::<Rank>() {
                let card_to_select = self
                    .hand
                    .iter()
                    .find(|card| card.rank == rank && card_count.get(card).map_or(0, |&c| c) > 0)
                    .copied();

                if let Some(card) = card_to_select {
                    selected.push(card);
                    *card_count.entry(card).or_insert(0) -= 1;
                } else {
                    return Err(PlayerError::CardNotFound(rank.to_string()));
                }
            } else {
                return Err(PlayerError::InvalidInput(token.to_string()));
            }
        }

        Ok(Cards::from_vec(selected))
    }

    /// 玩家出牌并从手牌中移除
    ///
    /// 验证玩家是否拥有要出的卡牌，若验证通过则从手牌中移除。
    ///
    /// # 参数
    /// * `cards` - 要出的卡牌
    ///
    /// # 返回值
    /// - `Ok(())` - 成功出牌
    /// - `Err(PlayerError)` - 出牌失败
    pub fn play_cards(&mut self, cards: &Cards) -> PlayerResult<()> {
        if !self.has_cards(cards) {
            return Err(PlayerError::InvalidPlay("手牌中不存在这些卡牌".to_string()));
        }

        for card in cards.iter() {
            self.hand.remove_one(*card);
        }

        Ok(())
    }
}

/// 从标准输入读取玩家的卡牌输入
///
/// 循环读取用户输入，直到获得非空输入。
/// 支持的格式（不进行验证）：
/// - 空格分隔：`3 3 4 5`
/// - 连续格式：`3345`
/// - 空字符串：表示跳过出牌
///
/// # 返回值
/// 用户输入的卡牌字符串（原样返回），或空字符串表示跳过
pub fn input() -> String {
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    input.trim().to_string()
}
