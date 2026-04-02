use std::{env, time::Duration};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    pub index: u32,
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ModelInput {
    model: String,
    messages: Vec<Message>,
}

pub fn request_ai_input(current_cards: String, play_history: String) -> String {
    let base_url = env::var("AI_BASE_URL").unwrap_or("https://api.deepseek.com".to_string());
    let api_key = env::var("AI_API_KEY").unwrap_or("".to_string());
    let client = reqwest::blocking::Client::builder().timeout(Duration::from_mins(3)).build().unwrap();
    let input = serde_json::to_string(&ModelInput{
        model: "deepseek-reasoner".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: r#"
                    你正在参与一局斗地主游戏，请根据给定的【出牌记录】和【你的手牌】进行决策，并输出你要出的牌。

                    【规则要求】
                    1. 只输出最终要出的牌，不要输出任何解释或多余内容。
                    2. 输出格式必须是纯字符串，例如：3 3 3 4 4 4，或 跳过(空格)。
                    3. 如果出王：
                       - 大王输出：big
                       - 小王输出：small

                    【身份判断】
                    1. 一共三名玩家，按出牌记录顺序轮流出牌。
                    2. 出牌记录中的第一个玩家是地主。
                    3. 从出牌记录的最后开始往前数：
                       - 第1个是上一个出牌的人
                       - 第2个是上上个
                       - 第3个就是你
                    4. 如果出牌记录为空：
                       - 你是地主
                       - 你先出牌

                    【出牌规则】
                    1. 必须按照斗地主规则出合法牌型。
                    2. 如果轮到你出牌且无人压制（你是当前轮第一个出牌）：
                       - 你可以自由选择任意合法牌型
                    3. 如果需要跟牌：
                       - 必须出相同牌型且更大的牌
                       - 或者选择 pass
                    4. 炸弹和王炸可以压任何非炸弹牌型
                    5. 若无法出牌，请输出：pass

                    【输入示例】
                    出牌记录：
                    [ ... ]

                    你的手牌：
                    [ ... ]

                    【你的任务】
                    根据当前局面，输出你要出的牌。"#.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("{} {}", current_cards, play_history),
            },
        ],
    }).unwrap();
    let res = client
        .post(base_url)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .body(input)
        .send()
        .expect("msg");
    let x:ApiResponse = res.json().unwrap();
    // let x: ApiResponse = serde_json::from_str(&x).unwrap();
    // println!("AI -> {}", x.choices[0].message.content);

    x.choices[0].message.content.clone()
}
