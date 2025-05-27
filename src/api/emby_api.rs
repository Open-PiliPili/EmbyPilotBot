use std::collections::HashMap;

use crate::network::{
    HttpMethod,
    NetworkTarget,
    NetworkTask
};

pub enum EmbyAPI {
    GetUser { user_id: String },
}

impl NetworkTarget for EmbyAPI {

    fn base_url(&self) -> String {
        "127.0.0.1".to_string()
    }

    fn path(&self) -> String {
        match self {
            EmbyAPI::GetUser { user_id, .. } => {
                format!("emby/Users/{}", user_id)
            }
        }
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn task(&self) -> NetworkTask {
        match self {
            EmbyAPI::GetUser { user_id: _ } => {
                let api_key = "".to_string();
                let mut params = HashMap::new();
                params.insert("api_key".to_string(), api_key);
                NetworkTask::RequestParameters(params)
            }
        }
    }

    fn headers(&self) -> Option<Vec<(&'static str, String)>> {
        Some(vec![
            ("accept", "application/json".to_string()),
            ("origin", "".to_string()),
            ("referer", format!("{}/", "".to_string())),
            ("user-agent", "PilotBot/0.0.1".to_string()),
        ])
    }
}
