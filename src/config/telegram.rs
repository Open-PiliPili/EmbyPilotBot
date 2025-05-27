use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramConfig {

    pub bot_token: String,
    
    pub chat_id: String,
}

impl Default for TelegramConfig {

    fn default() -> Self {
        Self {
            bot_token: "".to_string(),
            chat_id: "".to_string(),
        }
    }
}