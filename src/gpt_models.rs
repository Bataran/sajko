use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}
#[derive(Serialize, Deserialize)]
pub struct Prompt {
    pub model: String,
    pub messages: Vec<Message>,
}
#[derive(Deserialize, Debug)]
pub struct GptResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
    system_fingerprint: String,
}
#[derive(Deserialize, Debug)]
pub struct Choice {
    index: i32,
    message: Message,
    logprobs: Option<bool>,
    finish_reason: String,
}
#[derive(Deserialize, Debug)]
pub struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}
