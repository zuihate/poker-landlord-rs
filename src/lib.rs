//! 斗地主游戏引擎
//!
//! 这是一个完整的地主游戏实现库，包含以下主要模块：
//!
//! # 主要模块
//!
//! - [`game`] - 游戏主逻辑，包括出牌阶段和抢地主阶段
//! - [`player`] - 玩家定义和操作
//! - [`rules`] - 牌型分类和游戏规则判断
//! - [`card`] - 卡牌定义和操作
//! - [`error`] - 统一的错误处理类型
//!
//! # 快速开始
//!
//! ```ignored
//! use poker_landlord_rs::game::Game;
//!
//!
//! let mut game = Game::new();
//! game.bidding_phase();  // 抢地主
//! game.play_phase();     // 出牌
//! ```
//!
//! # 游戏规则
//!
//! 地主游戏有以下基本规则：
//! - 3个玩家，其中1个是地主
//! - 地主获得3张底牌
//! - 地主先出牌
//! - 大于等于5张牌的顺牌序列称为"顺"
//! - 炸弹可以压制除王炸外的所有牌
//! - 王炸是最强的牌

pub mod card;
pub mod error;
pub mod game;
pub mod player;
pub mod rules;
pub mod util;
