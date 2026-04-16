use crate::card::Cards;
use crate::player::PlayerType;
use crate::rules::Play;
use crate::rules::PlayCategory;

use super::Game;

impl Game {
    /// 出牌阶段
    ///
    /// 从地主开始轮流出牌，直到某个玩家手牌全部出完为止。
    /// 其他玩家必须以相同或更大的牌型跟牌，否则可以选择跳过。
    /// 连续两位玩家跳过后，上一手牌被放弃，当前玩家可以重新开牌。
    pub fn play_phase(&mut self) {
        println!("出牌阶段开始。");

        loop {
            if self.players[self.current_player].player_type == PlayerType::Human {
                println!(
                    "{}{}的牌: {}",
                    self.players[self.current_player].player_type,
                    self.players[self.current_player].id + 1,
                    self.players[self.current_player].hand,
                );
                println!(
                    "{}{}，请选择要出的牌（输入牌的点数，例如 '3 3 4 5'）为空则跳过:",
                    self.players[self.current_player].player_type,
                    self.players[self.current_player].id + 1,
                );
            }

            let play = match self.players[self.current_player].choose_play() {
                Ok(play) => play,
                Err(error) => {
                    println!("选择牌错误: {}", error);
                    continue;
                }
            };

            if !play.can_beat(&self.last_played_cards) {
                if play.category == PlayCategory::Pass {
                    println!(
                        "{}{}选择跳过。",
                        self.players[self.current_player].player_type,
                        self.players[self.current_player].id + 1
                    );

                    self.current_player = (self.current_player + 1) % 3;

                    if self.last_player == self.current_player {
                        println!("连续两位玩家过牌，上一手牌被放弃，重新开始出牌。");
                        self.last_played_cards = Play::new(Cards::new()).unwrap();
                    }
                    continue;
                }
                println!(
                    "你的牌不能压过上一手: {}，请重新出牌。",
                    &self.last_played_cards.cards
                );
                continue;
            }

            match self.players[self.current_player].play_cards(&play.cards) {
                Ok(_) => {
                    println!(
                        "{}{}出牌: {}",
                        self.players[self.current_player].player_type,
                        self.players[self.current_player].id + 1,
                        play.cards
                    );
                    self.last_played_cards = play;
                    self.last_player = self.current_player;
                }
                Err(error) => {
                    println!("出牌失败: {}", error);
                    continue;
                }
            }

            if self.players[self.current_player].hand.is_empty() {
                println!(
                    "{}{} {} 获胜！",
                    self.players[self.current_player].player_type,
                    self.players[self.current_player].id + 1,
                    self.players[self.current_player].role
                );
                break;
            }
            self.current_player = (self.current_player + 1) % 3;
        }
    }
}
