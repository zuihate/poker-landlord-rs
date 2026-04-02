use std::io::{self, Write};

use crate::card::rank::Rank;
use crate::card::suit::Suit;
use crate::player::PlayerType;

use super::Game;

/// 询问玩家是/否问题
///
/// 循环读取用户输入，直到获得有效的 y/n 响应。
///
/// # 返回值
/// 用户的是/否选择
fn ask_yes_no() -> bool {
    loop {
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("请输入 'y' 或 'n'");
                continue;
            }
        }
    }
}

impl Game {
    /// 抢地主阶段
    ///
    /// 从持有方块3的玩家开始，依次询问是否抢地主。
    /// - 第一个抢的成为临时地主
    /// - 后续有人抢则替换地主
    /// - 连续两位玩家过牌则结束抢地主
    /// - 如果都没人抢，则持有方块3的玩家默认成为地主
    ///
    /// 结束后，`current_player` 和 `last_player` 都会设置为地主的索引。
    pub fn bidding_phase(&mut self) {
        let mut last_bidder: Option<usize> = None; // 最后一个抢地主的玩家索引
        let mut pass_streak = 0; // 记录连续的过牌次数，如果达到2次则结束抢地主

        // 从持有方块3的玩家开始抢地主
        let start_player = self
            .players
            .iter()
            .position(|p| {
                p.hand
                    .iter()
                    .any(|c| c.rank == Rank::Three && c.suit == Some(Suit::Diamonds))
            })
            .unwrap_or(0);

        // 循环抢地主，最多进行6轮（每人两轮）
        for turn in 0..6 {
            // 计算当前玩家索引
            let player = &self.players[(start_player + turn) % 3];

            println!("玩家{}的牌: {}", player.id + 1, player.hand);
            println!("玩家{}，你要抢地主吗？(y/n)", player.id + 1);
            // 读取玩家输入，判断是否抢地主
            let input: bool = if player.player_type == PlayerType::AI {
                rand::random()
            } else {
                ask_yes_no()
            };

            // 处理玩家的选择
            if input {
                last_bidder = Some(player.id);
                pass_streak = 0; // 抢地主成功，重置过牌计数
                println!("玩家{}抢地主成功！", player.id + 1);
            } else {
                pass_streak += 1; // 过牌，增加计数
                println!("玩家{}选择过牌。", player.id + 1);
            }
            // 如果连续两位玩家过牌，结束抢地主阶段
            if pass_streak >= 2 {
                println!("连续两位玩家过牌，抢地主阶段结束。");
                break;
            }
        }
        // 确定地主玩家，如果没有人抢，则默认持有方块3的人成为地主
        let landlord_index = last_bidder.unwrap_or(start_player);
        // 设置地主玩家并发牌
        self.players[landlord_index].become_landlord(&self.landlord_cards);
        self.last_player = landlord_index;
        self.current_player = landlord_index; // 地主先出牌

        println!("玩家{}成为地主！", landlord_index + 1);
        println!("底牌: {}", self.landlord_cards);
    }
}
