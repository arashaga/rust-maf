use crate::types::{Message};

pub(crate) struct AgentSession {
    history: Vec<Message>,
}

impl AgentSession {
    pub(crate) fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub(crate) fn history(&self) -> &[Message] {
        &self.history
    }   

    pub(crate) fn replace_history(&mut self, messages: Vec<Message>) {
        self.history = messages;
    }
}

#[cfg(test)]
#[path = "sessions_test.rs"]
mod tests;