use crate::types::{Message};
use uuid::Uuid;

pub(crate) struct AgentSession {
    history: Vec<Message>,
    id: String,
}

impl AgentSession {
    pub(crate) fn new() -> Self {
        Self {
            history: Vec::new(),
            id: Uuid::new_v4().to_string(),
        }
    }

    pub(crate) fn history(&self) -> &[Message] {
        &self.history
    }   

    pub(crate) fn replace_history(&mut self, messages: Vec<Message>) {
        self.history = messages;
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }
}




#[cfg(test)]
#[path = "sessions_test.rs"]
mod tests;