use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorJson {
    pub message: String,
}

impl ErrorJson {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }

    pub fn default() -> Self {
        Self {
            message: "Error".to_string(),
        }
    }
}
