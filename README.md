# poker-landlord

**用 Rust 实现的命令行斗地主游戏**  
A command-line Dou Dizhu (Landlord) game implemented in Rust.


##  **注意**：README.md、单元测试、内部文档**都是AI写的**

## ✨ 特性 / Features

- ✅ **完整斗地主规则引擎**：支持单牌、对子、三不带、三带一/二、四带二、炸弹、王炸、顺子、连对、飞机（含带单/带对）
- ✅ **智能牌型识别与大小比较**：严格按照经典斗地主规则（炸弹压非炸弹，王炸最强）
- ✅ **友好输入系统**：支持多种输入格式（`3 3 4 5`、`3345`、`s3 h4`、`小王`、`sj` 等）
- ✅ **抢地主阶段**：从方块3持有者开始，支持多人抢地主，连续过牌结束
- ✅ **出牌阶段**：轮流出牌、跟牌、跳过，自动判断是否能压牌
- ✅ **清晰的卡牌显示**：使用花色符号（♠♥♣♦）+ 中文王牌显示
- ✅ **模块化设计**：卡牌系统、规则引擎、游戏流程完全分离，便于扩展
- ✅ **完善的错误处理**：输入错误、非法牌型都有友好提示

> **实则不然**

**当前状态**：可完整单机游玩（三人本地输入），暂无 AI（后续计划添加简单 AI）。

## 🚀 快速开始 / Quick Start

### 1. 运行项目
```bash
git clone https://github.com/zuihate/poker-landlord-rs.git
```
```bash
cd poker-landlord
```
```bash
cargo run
```

### 2. 游戏流程
1. 游戏自动发牌（17+17+17+3底牌）
2. **抢地主阶段**：从持有 **♦3** 的玩家开始，输入 `y` / `yes` 抢地主，`n` / `no` 过牌
3. **出牌阶段**：地主先出牌，按提示输入要出的牌点数（空格分隔或连续输入），空行表示跳过

**输入示例**：
- 单牌：`5` 或 `A`
- 对子：`88` 或 `8 8`
- 三带一：`999 4`
- 顺子：`34567`
- 连对：`334455`
- 飞机：`333444`
- 王炸：`小王 大王` 或 `sj bj`
- 跳过：直接按回车

## 📋 支持的牌型 / Supported Play Types

| 牌型             | 英文名称              | 张数   | 说明                         |
|------------------|-----------------------|--------|------------------------------|
| 单牌             | Single               | 1      | 任意一张牌                    |
| 对子             | Pair                 | 2      | 两张相同点数                  |
| 三不带           | Triple               | 3      | 三张相同点数                  |
| 三带一           | ThreeWithOne         | 4      | 三张 + 一张单牌               |
| 三带二           | ThreeWithTwo         | 5      | 三张 + 一对                   |
| 四带二           | FourWithTwo          | 6      | 四张 + 一对（当前支持）       |
| 炸弹             | Bomb                 | 4      | 四张相同点数                  |
| **王炸**         | **JokerBomb**        | 2      | 大小王各一张（最强）          |
| 顺子             | Straight             | ≥5     | 连续单牌（不含2和王）         |
| 连对             | PairSequence         | ≥6     | 连续对子（至少3对）           |
| 飞机             | Plane                | ≥6     | 连续三张（至少2组）           |
| 飞机带单         | PlaneWithSingle      | -      | 飞机 + 等量单牌               |
| 飞机带对         | PlaneWithPair        | -      | 飞机 + 等量对子               |

> **注意**：2 和王牌不能参与顺子、连对、飞机。

## 🛠️ 项目结构 / Project Structure

```
poker-landlord/
├── src/
│   ├── lib.rs                 # 库入口
│   ├── main.rs                # 命令行入口
│   ├── card/                  # 基础卡片（Rank, Suit, Card, Cards）
│   ├── game/                  # 游戏核心（发牌、抢地主、出牌阶段）
│   ├── rules.rs               # 牌型分类与大小比较
│   ├── player.rs              # 玩家手牌管理
│   └── error.rs               # 错误处理
├── Cargo.toml
└── README.md
```

## 🧪 测试 / Testing

```bash
cargo test                  # 运行所有单元测试
cargo test -- --nocapture   # 显示测试输出
cargo fmt -- --check        # 检查代码格式
cargo clippy                # 静态检查
```

## 📝 TODO（后续计划）

- [ ] 添加简单 AI（至少能自动出牌和跟牌）
- [ ] 支持更多输入方式

## 📄 License

本项目采用 [MIT License](LICENSE) 开源。

---


有任何问题或建议，欢迎在 [Issues](https://github.com/zuihate/poker-landlord/issues) 中提出。