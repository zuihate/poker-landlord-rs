# 斗地主游戏引擎库使用指南

`poker-landlord-rs` 是一个纯逻辑的斗地主游戏引擎，设计用于在其他 Rust 项目中作为库被调用。

## 库特性

- **纯逻辑设计**：不包含 UI、网络或终端交互代码
- **可复用性强**：可在本地游戏、网络游戏、AI 等各种场景中使用
- **类型安全**：完整的错误处理和类型检查
- **易于集成**：清晰的 API 和丰富的文档注释

## 在另一个项目中使用

### 1. 添加依赖

在你的项目的 `Cargo.toml` 中添加：

```toml
[dependencies]
poker-landlord-rs = { path = "../poker-landlord-rs" }
```

或者，如果发布到 crates.io：

```toml
[dependencies]
poker-landlord-rs = "0.1"
```

### 2. 快速开始

```rust
use poker_landlord_rs::{Game, GameAction, GamePhase};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建新游戏
    let mut game = Game::new();
    
    // 查看当前阶段
    println!("游戏阶段: {:?}", game.phase());
    
    // 执行动作
    while !game.is_finished() {
        let current_player = game.current_player();
        println!("轮到玩家 {} 出牌", current_player);
        
        // 这里应该从用户输入或 AI 获取动作
        // let action = GameAction::Bid { player_id: current_player, bid: true };
        // game.apply_action(action)?;
        
        break; // 示例中只迭代一次
    }
    
    Ok(())
}
```

### 3. 主要 API

#### 创建游戏

```rust
use poker_landlord_rs::Game;

// 创建默认游戏（3 个真人玩家）
let game = Game::new();

// 创建指定玩家类型的游戏
use poker_landlord_rs::PlayerType;
let game = Game::new_with_player_types([
    PlayerType::Human,
    PlayerType::AI,
    PlayerType::Human,
]);
```

#### 执行动作

```rust
use poker_landlord_rs::{GameAction, Play, Cards};

// 抢地主动作
let bid_action = GameAction::Bid { 
    player_id: 0, 
    bid: true  // true 表示抢，false 表示过
};

// 出牌动作
let play = Play::new(Cards::default())?;
let play_action = GameAction::Play { 
    player_id: 0, 
    play 
};

let result = game.apply_action(bid_action)?;
```

#### 获取游戏状态

```rust
// 当前阶段
let phase = game.phase();

// 当前轮到的玩家
let current = game.current_player();

// 完整状态快照（用于网络同步或保存）
let state = game.game_state();

// 检查游戏是否结束
if game.is_finished() {
    if let Some(winner) = game.winner() {
        println!("玩家 {} 获胜！", winner);
    }
}
```

### 4. 主要类型

#### `Game` - 游戏状态机
核心游戏引擎，管理游戏的所有状态和逻辑。

#### `GameAction` - 玩家动作
```rust
pub enum GameAction {
    Bid { player_id: usize, bid: bool },      // 抢地主
    Play { player_id: usize, play: Play },    // 出牌
}
```

#### `GamePhase` - 游戏阶段
```rust
pub enum GamePhase {
    Bidding { start_player, pass_streak, last_bidder },  // 抢地主阶段
    Playing,                                               // 出牌阶段
    Finished { winner },                                  // 游戏结束
}
```

#### `Card` 和 `Cards` - 卡牌相关
```rust
use poker_landlord_rs::{Card, Cards, Rank, Suit};

// 创建单张卡牌
let card = Card::new(Rank::Three, Suit::Spades);

// 王牌
let big_joker = Card::joker(false);
let small_joker = Card::joker(true);

// 卡牌集合（手牌）
let mut cards = Cards::new();
cards.push(card);
```

#### `Play` - 出牌
```rust
use poker_landlord_rs::Play;

// 创建出牌
let play = Play::new(cards)?;

// 检查是否能压过上一手牌
if play.can_beat(&last_play) {
    // 可以压过
}
```

#### `Player` - 玩家
```rust
use poker_landlord_rs::{Player, Role, PlayerType};

let mut player = Player::new(
    0,                      // 玩家 ID
    cards,                  // 初始手牌
    Role::Farmer,          // 角色（地主/农民）
    PlayerType::Human,     // 玩家类型（真人/AI）
);

// 成为地主
player.become_landlord(&landlord_cards);
```

### 5. 错误处理

所有 API 都返回 `GameResult<T>` 类型，即 `Result<T, GameError>`：

```rust
match game.apply_action(action) {
    Ok(result) => {
        // 处理成功结果
        println!("动作成功: {:?}", result);
    },
    Err(err) => {
        // 处理错误
        eprintln!("游戏错误: {}", err);
    }
}
```

### 6. 网络游戏场景

为了在网络中使用，你需要：

1. **序列化 GameState**：使用 `serde` + `serde_json` 序列化 `game.game_state()`
2. **发送给客户端**：通过网络发送序列化的游戏状态
3. **接收玩家动作**：从网络接收 `GameAction`
4. **应用动作**：在服务端调用 `game.apply_action(action)`

示例（需要 `serde` 依赖）：

```rust
use serde_json;

// 序列化状态
let state = game.game_state();
let json = serde_json::to_string(&state)?;

// 通过网络发送...

// 接收并反序列化
let received_action: GameAction = serde_json::from_str(&json)?;
```

### 7. AI 集成

`PlayerType::AI` 标记了 AI 玩家，但默认行为与真人相同。你可以：

1. 在你的项目中实现 AI 决策逻辑
2. 检查 `player.player_type` 来区分真人和 AI
3. 为 AI 玩家自动生成合法的 `GameAction`

### 8. 模块组织

```
poker_landlord_rs
├── game          - 游戏状态机和流程控制
├── card          - 卡牌定义和操作
├── player        - 玩家定义
├── rules         - 出牌规则和牌型分类
└── error         - 统一错误类型
```

每个模块都可以独立导入：

```rust
use poker_landlord_rs::game::Game;
use poker_landlord_rs::card::Card;
use poker_landlord_rs::rules::Play;
```

或从根部导入常用类型：

```rust
use poker_landlord_rs::{Game, Card, Play};
```

## 开发建议

1. **了解游戏流程**：先阅读 `game::Game` 的文档
2. **测试你的集成**：创建简单的测试用例验证 API 调用
3. **处理所有错误**：使用 `?` 操作符或 `match` 处理 `GameResult`
4. **状态管理**：保存 `GameState` 用于网络同步或日志
5. **类型安全**：利用 Rust 的类型系统避免运行时错误

## 许可证

MIT License

## 贡献

欢迎提交 Issue 或 Pull Request！
