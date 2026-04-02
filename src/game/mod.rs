//! 游戏主模块
//!
//! 本模块包含游戏的核心逻辑，包括：
//! - [`phase`] - 出牌阶段的逻辑
//! - [`bidding`] - 抢地主阶段的逻辑
//! - [`dealer`] - 发牌和游戏初始化
//!
//! # 游戏流程
//!
//! 1. 创建游戏：`Game::new()`
//! 2. 抢地主：`game.bidding_phase()`
//! 3. 出牌：`game.play_phase()`

pub mod bidding;
pub mod dealer;
pub mod phase;

use crate::card::cards::Cards;
use crate::game::dealer::*;
use crate::player::*;
use crate::rules::Play;

/// 代表一局斗地主游戏
///
/// 该结构体管理游戏的整个状态，包括玩家、手牌、地主底牌和游戏进度。
///
/// # 字段
///
/// - `players` - 三名玩家（固定3人）
/// - `landlord_cards` - 地主的底牌（3张）
/// - `current_player` - 当前轮到的玩家索引（0-2）
/// - `last_player` - 上次有效出牌的玩家索引
/// - `last_played_cards` - 上次出牌的牌型及卡牌
#[allow(clippy::new_without_default)]
#[derive(Debug)]
pub struct Game {
    /// 三名玩家的信息和手牌
    pub players: [Player; 3],
    /// 地主底牌（待发配）
    pub landlord_cards: Cards,
    /// 当前轮到的玩家索引（0-2）
    pub current_player: usize,
    /// 上次有效出牌的玩家索引
    pub last_player: usize,
    /// 上次有效出牌的牌型和卡牌
    pub last_played_cards: Play,
}

impl Game {
    /// 从指定的发牌结果创建游戏
    ///
    /// # 参数
    /// * `deal` - 发牌结果，包含三名玩家的手牌和地主底牌
    ///
    /// # 返回值
    /// 新的游戏实例
    pub fn from_deal(deal: InitialDeal) -> Self {
        let players = [
            Player::new(
                0,
                deal.player_hands[0].clone(),
                Role::Farmer,
                PlayerType::Human,
            ),
            Player::new(
                1,
                deal.player_hands[1].clone(),
                Role::Farmer,
                PlayerType::Human,
            ),
            Player::new(
                2,
                deal.player_hands[2].clone(),
                Role::Farmer,
                PlayerType::Human,
            ),
        ];

        let last_played_cards = Play::new(Cards::new()).unwrap();

        Self {
            players,
            landlord_cards: deal.landlord_cards,
            current_player: 0,
            last_player: 0,
            last_played_cards,
        }
    }

    /// 创建一个新游戏
    ///
    /// 自动进行发牌并初始化游戏状态。
    /// 52张标准牌分配给三名玩家，剩余3张作为地主底牌。
    ///
    /// # 返回值
    /// 新的游戏实例
    pub fn new() -> Self {
        let deal = InitialDeal::new();
        Self::from_deal(deal)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
