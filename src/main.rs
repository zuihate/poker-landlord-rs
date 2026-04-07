use poker_landlord_rs::game::Game;

fn main() {
    println!("=== 欢迎来到斗地主游戏 ===\n");

    let mut game = Game::new();

    println!("游戏初始化完成，开始抢地主阶段...\n");
    game.bidding_phase();

    println!(
        "\n抢地主阶段结束。地主: 玩家{}。\n",
        game.current_player + 1
    );

    println!("开始出牌阶段...\n");
    game.play_phase();

    println!("\n游戏结束！");
}
