//! 卡牌字符串的词汇分析（tokenization）
//!
//! 将用户输入字符串分解为卡牌点数标记（tokens），
//! 为后续的 Rank 和 Card 解析做准备。

/// 将用户输入字符串分解为卡牌点数标记
///
/// 不涉及花色的解析，只识别卡牌的点数部分。
/// 花色识别由 Suit 的 FromStr 实现负责。
///
/// # 输入格式支持
///
/// 用户可以输入多种格式，所有格式可混用：
///
/// - **空格分隔**：`"3 3 4 5"` -> ["3", "3", "4", "5"]
/// - **连贯输入**：`"3345"` -> ["3", "3", "4", "5"]
/// - **混合输入**：`"33 4 5"` -> ["3", "3", "4", "5"]
/// - **花色字母**：`"s3 h4 d5"`（花色被忽略）-> ["3", "4", "5"]
/// - **英文王牌（完整）**：`"small big"` -> ["small", "big"]
/// - **英文王牌（缩写）**：`"s b"` -> ["s", "b"]
/// - **英文花牌（完整）**：`"jack queen king ace"`
/// - **英文花牌（缩写）**：`"j q k a"`
///
/// # 解析规则
///
/// 1. 跳过所有空白字符（空格、制表符、换行等）
///
/// 2. **优先匹配长字符串（贪心匹配）**
///    按以下顺序尝试匹配：
///    - "jokersmall"
///    - "jokerbig"
///    - "small"
///    - "queen"
///    - "jack"
///    - "king"
///    - "ace"
///    - "big"
///
///    这样可以避免 `"small"` 被错误拆分为 `"s"` + `"mall"`
///
/// 3. 数字识别：
///    - `"10"` → 作为整体识别
///    - `"0"` → 视为 `"10"`
///    - `"2" - "9"` → 单字符数字
///    - `"1"` → 忽略
///
/// 4. 单字符牌：
///    - 花牌：`j q k a`
///    - 王牌缩写：`s`（small）, `b`（big）
///
/// 5. 无法识别的字符会被忽略（包括花色符号 ♠ ♥ ♣ ♦）
///
/// # 返回值
///
/// 返回一个 `Vec<&str>`，其中：
///
/// - 部分 token 是原输入字符串的切片（如 `"3"`）
/// - 部分 token 是静态字符串（如 `"10"`、`"small"`）
///
/// 因此返回值**不保证全部来源于原字符串切片**
///
/// # 示例
///
/// ```
/// use poker_landlord_rs::card::parser::tokenize_card_input;
///
/// let input = "3 3 4 5";
/// let tokens = tokenize_card_input(input);
/// assert_eq!(tokens, vec!["3", "3", "4", "5"]);
///
/// let input = "small big";
/// let tokens = tokenize_card_input(input);
/// assert_eq!(tokens, vec!["small", "big"]);
///
/// let input = "sj";
/// let tokens = tokenize_card_input(input);
/// assert_eq!(tokens, vec!["s", "j"]);
/// ```
pub fn tokenize_card_input(input: &str) -> Vec<&str> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::with_capacity(16);
    let mut i = 0;

    /// 尝试匹配一个固定字符串
    ///
    /// 如果匹配成功，则将其添加到 tokens，指针前进，然后继续主循环
    macro_rules! try_match {
        ($pat:expr) => {
            if bytes[i..].starts_with($pat.as_bytes()) {
                tokens.push($pat);
                i += $pat.len();
                continue;
            }
        };
    }

    while i < bytes.len() {
        let c = bytes[i].to_ascii_lowercase();

        // ===== 跳过空白字符 =====
        if (c as char).is_whitespace() {
            i += 1;
            continue;
        }

        // ===== 优先匹配长字符串 =====
        // 这很关键！必须先尝试长的，再尝试短的
        // 例如 "small" 和 "s" 都能匹配 "small" 的首字符
        // 如果先检查短的，"small" 会被拆成 "s" + "mall"，导致解析错误
        try_match!("jokersmall"); // 最长的王牌标记
        try_match!("jokerbig"); // 次长的王牌标记
        try_match!("small"); // 小王的英文
        try_match!("queen"); // 花牌 Q
        try_match!("jack"); // 花牌 J
        try_match!("king"); // 花牌 K
        try_match!("ace"); // 花牌 A
        try_match!("big"); // 大王的英文

        // ===== 数字识别 =====
        if c.is_ascii_digit() {
            // 特殊处理 "10"
            if c == b'1' && i + 1 < bytes.len() && bytes[i + 1] == b'0' {
                tokens.push("10");
                i += 2;
                continue;
            }
            // "0" 也表示 10
            if c == b'0' {
                tokens.push("10");
                i += 1;
                continue;
            }
            // 其他数字 2-9
            if (b'2'..=b'9').contains(&c) {
                tokens.push(std::str::from_utf8(&bytes[i..i + 1]).unwrap());
                i += 1;
                continue;
            }
            // 其他数字（如 1）直接跳过
            i += 1;
            continue;
        }

        // ===== 单字符牌 - 花牌和王牌快速输入 =====
        // J(ack), Q(ueen), K(ing), A(ce) - 花牌
        // S(mall) - 小王快速输入
        // B(ig) - 大王快速输入
        if matches!(c, b'j' | b'q' | b'k' | b'a' | b's' | b'b') {
            tokens.push(std::str::from_utf8(&bytes[i..i + 1]).unwrap());
            i += 1;
            continue;
        }

        // ===== 无法识别字符 - 直接跳过 =====
        // 这包括花色符号 ♠ ♥ ♣ ♦ 和其他无关字符
        i += 1;
    }

    tokens
}
