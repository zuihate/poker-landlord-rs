# 快速集成指南

## 一句话总结

`poker-landlord-rs` 是一个**纯逻辑的斗地主游戏引擎**，可以被另一个 Rust 项目作为库调用，负责游戏规则、状态管理和动作处理。

## 集成步骤

### 1️⃣ 添加依赖

在你的项目 `Cargo.toml` 中：

```toml
[dependencies]
poker-landlord-rs = { path = "../poker-landlord-rs" }
```

### 2️⃣ 基础使用

```rust
use poker_landlord_rs::{Game, GameAction, GamePhase};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();
    
    // 查询游戏状态
    println!("当前玩家: {}", game.current_player());
    println!("当前阶段: {:?}", game.phase());
    
    // 执行玩家动作
    let action = GameAction::Bid { player_id: 0, bid: true };
    let result = game.apply_action(action)?;
    println!("结果: {:?}", result);
    
    // 获取完整状态（用于网络同步或保存）
    let state = game.game_state();
    
    // 检查游戏是否结束
    if game.is_finished() {
        println!("胜者: {}", game.winner().unwrap());
    }
    
    Ok(())
}
```

### 3️⃣ 核心 API

| 方法 | 说明 |
|------|------|
| `Game::new()` | 创建新游戏（默认3个真人） |
| `game.apply_action(action)` | 执行玩家动作 |
| `game.current_player()` | 获取当前轮到的玩家 |
| `game.phase()` | 获取当前游戏阶段 |
| `game.game_state()` | 获取完整状态快照 |
| `game.is_finished()` | 检查游戏是否结束 |
| `game.winner()` | 获取胜者（游戏结束时） |

### 4️⃣ 动作类型

```rust
use poker_landlord_rs::{GameAction, Play, Cards};

// 抢地主
let bid = GameAction::Bid { 
    player_id: 0, 
    bid: true  // true=抢，false=过
};

// 出牌
let play = Play::new(cards)?;
let play_action = GameAction::Play { 
    player_id: 0, 
    play 
};
```

### 5️⃣ 游戏阶段

```rust
use poker_landlord_rs::GamePhase;

match game.phase() {
    GamePhase::Bidding { start_player, pass_streak, last_bidder } => {
        println!("抢地主阶段");
    },
    GamePhase::Playing => {
        println!("出牌阶段");
    },
    GamePhase::Finished { winner } => {
        println!("游戏结束，胜者: {}", winner);
    },
}
```

## 使用场景

| 场景 | 说明 |
|------|------|
| **本地 CLI 游戏** | 处理输入、显示状态、调用库 |
| **网络游戏服务端** | 维护游戏状态、接收玩家动作、序列化状态 |
| **网络游戏客户端** | 显示游戏状态、收集玩家输入、序列化/反序列化 |
| **AI 对战** | 为 AI 生成合法动作、评估游戏状态 |
| **自动化测试** | 模拟多个游戏流程、验证规则 |

## 常见任务

### 🎮 在 CLI 中使用

```rust
loop {
    let current = game.current_player();
    let input = get_user_input(current);
    let action = parse_input_to_action(input)?;
    game.apply_action(action)?;
    display_game_state(&game);
    
    if game.is_finished() { break; }
}
```

### 🌐 在网络中使用

```rust
use serde_json;

// 服务端
let state = game.game_state();
let json = serde_json::to_string(&state)?;
send_to_client(json);

// 客户端
let state: GameState = serde_json::from_str(&json)?;
render_ui(&state);
```

### 🤖 AI 集成

```rust
use poker_landlord_rs::PlayerType;

// 标记 AI 玩家
let game = Game::new_with_player_types([
    PlayerType::Human,
    PlayerType::AI,
    PlayerType::Human,
]);

// 为 AI 生成合法动作
if game.current_player_type() == PlayerType::AI {
    let action = ai_decide(&game);
    game.apply_action(action)?;
}
```

## 文件结构

```
你的项目/
├── Cargo.toml
├── src/
│   ├── main.rs      # 你的 CLI 界面/网络层
│   ├── ai.rs        # AI 决策逻辑
│   ├── network.rs   # 网络通信
│   └── ui.rs        # 用户界面
└── poker-landlord-rs/  # 库目录（path 依赖）
```

## 类型导出

所有主要类型都可从 crate 根部导入：

```rust
use poker_landlord_rs::{
    // 核心
    Game, GameAction, GameActionResult, GamePhase, GameState, GameResult,
    // 玩家
    Player, Role, PlayerType,
    // 卡牌
    Card, Cards, Rank, Suit,
    // 规则
    Play, PlayCategory,
    // 错误
    GameError, PlayerError, PlayError,
};
```

## 错误处理

所有操作返回 `Result<T, GameError>`：

```rust
match game.apply_action(action) {
    Ok(result) => { /* 处理成功 */ },
    Err(GameError::ActionNotAllowed(msg)) => { /* 动作非法 */ },
    Err(GameError::InvalidPlayerId(id)) => { /* 玩家编号错误 */ },
    Err(e) => { /* 其他错误 */ },
}
```

## 高级特性

- ✅ 支持 `serde` 序列化（GameState 可序列化）
- ✅ 完整的文档注释（在 IDE 中查看）
- ✅ 模块化设计（可独立使用 `game`, `rules`, `card` 等）
- ✅ 类型安全（编译时错误检查）

## 下一步

详细 API 文档见 [LIBRARY_USAGE.md](./LIBRARY_USAGE.md)

---

**需要帮助？** 查看代码中的文档注释，或参考 `examples/` 目录中的示例。
