# poker-landlord-rs 作为库的完整指南

## 🎯 项目现状

你的 `poker-landlord-rs` 项目已优化为**可被其他 Rust 项目调用的库**。

### ✅ 完成的工作

1. **Cargo.toml 优化**
   - ✅ 修复 edition 版本（`2024` → `2021`）
   - ✅ 更新描述为 "pure logic" 游戏引擎
   - ✅ 添加库配置 `[lib]`
   - ✅ 添加 readme 字段

2. **API 导出**
   - ✅ 在 `src/lib.rs` 中重新导出主要类型
   - ✅ 所有模块都标记为 `pub`
   - ✅ 核心类型：`Game`, `GameAction`, `GamePhase`, `Play`, `Card`, etc.

3. **兼容性修复**
   - ✅ 移除 Rust 2024 特性（let chains）
   - ✅ 全部代码兼容 Rust 2021 edition
   - ✅ 编译通过 ✓

4. **文档**
   - ✅ 库级文档注释（`lib.rs`）
   - ✅ 各模块完整注释
   - ✅ 方法参数和返回值说明
   - ✅ 快速开始指南：`QUICK_START_LIBRARY.md`
   - ✅ 详细文档：`LIBRARY_USAGE.md`

---

## 📚 使用文档

### 给其他项目的开发者

1. **快速入门**：[QUICK_START_LIBRARY.md](./QUICK_START_LIBRARY.md)（5分钟快速集成）
2. **详细文档**：[LIBRARY_USAGE.md](./LIBRARY_USAGE.md)（完整 API 参考）

### 在自己项目中使用

**Cargo.toml：**
```toml
[dependencies]
poker-landlord-rs = { path = "../poker-landlord-rs" }
```

**代码示例：**
```rust
use poker_landlord_rs::{Game, GameAction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();
    let action = GameAction::Bid { player_id: 0, bid: true };
    game.apply_action(action)?;
    Ok(())
}
```

---

## 🗂️ 项目结构

```
poker-landlord-rs/
├── Cargo.toml                    # 库配置
├── README.md                     # 项目概述
├── QUICK_START_LIBRARY.md        # 快速集成指南 ⭐ 给其他项目开发者看
├── LIBRARY_USAGE.md              # 完整文档 ⭐ 详细参考
│
├── src/
│   ├── lib.rs                    # 库入口，重新导出主要类型
│   ├── main.rs                   # 空文件（预留给未来的二进制）
│   │
│   ├── game/
│   │   ├── mod.rs               # 游戏流程控制
│   │   ├── engine.rs            # 游戏状态机
│   │   ├── types.rs             # 游戏类型定义
│   │   └── dealer.rs            # 发牌逻辑
│   │
│   ├── card/
│   │   ├── mod.rs               # 卡牌模块入口
│   │   ├── rank.rs              # 点数定义
│   │   ├── suit.rs              # 花色定义
│   │   ├── cards.rs             # 卡牌集合
│   │   └── parser.rs            # 输入解析
│   │
│   ├── player.rs                # 玩家定义和操作
│   ├── rules.rs                 # 牌型分类和规则引擎
│   └── error.rs                 # 错误类型
│
└── target/
    └── doc/
        └── poker_landlord_rs/   # 自动生成的文档
```

---

## 🔑 核心 API

### 创建游戏
```rust
use poker_landlord_rs::Game;

let game = Game::new();  // 3个真人玩家
```

### 执行动作
```rust
use poker_landlord_rs::GameAction;

let action = GameAction::Bid { player_id: 0, bid: true };
let result = game.apply_action(action)?;
```

### 查询状态
```rust
game.current_player()          // 当前玩家
game.phase()                   // 游戏阶段
game.game_state()             // 完整状态快照
game.is_finished()            // 是否结束
game.winner()                 // 获胜者（如果已结束）
```

### 主要类型
| 类型 | 用途 |
|------|------|
| `Game` | 游戏状态机 |
| `GameAction` | 玩家动作 |
| `GamePhase` | 游戏阶段 |
| `GameState` | 完整状态快照 |
| `Card` / `Cards` | 卡牌 |
| `Play` | 出牌 |
| `Player` | 玩家 |

---

## 💡 使用场景

### 场景 1：CLI 客户端（本地 3 人）
```
你的项目（CLI 代码）
   ↓ 调用
poker-landlord-rs（游戏引擎）
```

### 场景 2：网络游戏服务端
```
客户端 A          客户端 B          客户端 C
   ↓                  ↓                ↓
   └─────────────────────────────────┘
            你的网络服务
               ↓ 调用
          poker-landlord-rs
```

### 场景 3：AI 对战
```
你的项目（AI 决策 + UI）
   ↓ 调用
poker-landlord-rs（游戏规则 + 状态）
```

---

## ✨ 特性

- **纯逻辑设计**：无 UI、网络、IO 代码，只管游戏规则
- **类型安全**：Rust 类型系统保证正确性
- **易于集成**：清晰的 API，最少 3 行代码开始
- **完整文档**：代码注释 + 指南文档
- **错误处理**：统一的 `GameResult<T>` 错误处理
- **序列化就绪**：`GameState` 可用 `serde` 序列化

---

## 📋 库的职责

### ✅ 库做的事
- 管理游戏状态（阶段、玩家、手牌等）
- 验证动作合法性（按规则判断）
- 处理牌型分类和大小比较
- 提供状态快照用于网络同步

### ❌ 库不做的事
- ~~显示 UI~~（你的项目负责）
- ~~网络通信~~（你的项目负责）
- ~~AI 决策~~（你的项目负责）
- ~~终端输入~~（你的项目负责）

---

## 🚀 快速集成检查清单

- [ ] 添加 `poker-landlord-rs` 到 `Cargo.toml`
- [ ] 导入核心类型：`use poker_landlord_rs::{Game, GameAction};`
- [ ] 创建游戏：`let mut game = Game::new();`
- [ ] 执行动作：`game.apply_action(action)?;`
- [ ] 查询状态：`game.current_player()`, `game.phase()`, etc.
- [ ] 处理错误：`match result { Ok(...) => ..., Err(...) => ... }`
- [ ] 测试通过 ✓

---

## 📖 推荐阅读顺序

1. 本文件（了解全貌）
2. [QUICK_START_LIBRARY.md](./QUICK_START_LIBRARY.md)（快速开始）
3. [LIBRARY_USAGE.md](./LIBRARY_USAGE.md)（深入学习）
4. 代码中的文档注释（IDE 中 Ctrl+Space 查看）

---

## 🎓 示例代码

### 完整游戏流程
```rust
use poker_landlord_rs::{Game, GameAction, GamePhase};

fn play_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Game::new();
    
    // 抢地主
    while matches!(game.phase(), GamePhase::Bidding { .. }) {
        let action = GameAction::Bid { 
            player_id: game.current_player(), 
            bid: true 
        };
        game.apply_action(action)?;
    }
    
    // 出牌
    while !game.is_finished() {
        let state = game.game_state();
        println!("当前玩家: {}", state.current_player);
        // ... 获取玩家动作 ...
    }
    
    if let Some(winner) = game.winner() {
        println!("胜者: 玩家 {}", winner);
    }
    
    Ok(())
}
```

### 网络序列化
```rust
use poker_landlord_rs::Game;

// 获取状态
let state = game.game_state();

// 序列化发送（需要 serde 依赖）
let json = serde_json::to_string(&state)?;

// 接收反序列化
let received_state: GameState = serde_json::from_str(&json)?;
```

---

## ❓ FAQ

**Q: 这个库能做什么？**  
A: 管理斗地主游戏的所有规则和状态逻辑。不包含 UI、网络、AI。

**Q: 怎么给玩家显示界面？**  
A: 这不是库的责任。你的项目根据 `game.game_state()` 的信息去显示。

**Q: 怎么实现 AI？**  
A: 在你的项目中实现 AI 逻辑，调用 `game` 获取合法动作列表，生成 `GameAction`。

**Q: 能在网络中使用吗？**  
A: 可以！序列化 `GameState`，通过网络发送，另一端反序列化后显示。

**Q: 如何处理错误？**  
A: 所有 `game` 方法返回 `Result`，使用 `?` 或 `match` 处理。

---

## 📞 后续支持

- 库代码已有完整注释，可在 IDE 中查看
- 文档已生成：`cargo doc --lib --no-deps --open`
- 有问题参考 `LIBRARY_USAGE.md` 详细说明

---

**恭喜！你的 poker-landlord-rs 现在已是一个可供使用的高质量 Rust 库。** 🎉

选择一个你喜欢的项目（CLI、网络服务、AI 对战），在里面调用它吧！
