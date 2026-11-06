use crate::types::{Message};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSession {
    history: Vec<Message>,
    id: String,
}

impl AgentSession {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            id: Uuid::new_v4().to_string(),
        }
    }

    pub fn history(&self) -> &[Message] {
        &self.history
    }   

    pub(crate) fn replace_history(&mut self, messages: Vec<Message>) {
        self.history = messages;
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // Serialize this session.
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        // Restore a session from JSON.
        serde_json::from_str(json)
    }

}




#[cfg(test)]
#[path = "sessions_test.rs"]
mod tests;