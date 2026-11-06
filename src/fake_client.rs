use crate::types::{ChatResponse, Message};
use crate::clients::{ChatClient, ClientError};

pub struct FakeChatClient {
    reply: String,
}

impl  FakeChatClient {
    pub fn new(reply: String) -> Self {
        Self { reply }
    }
}

impl ChatClient for FakeChatClient {
        fn get_response(&self, _messages: &[Message], _instructions: Option<&str>) -> Result<ChatResponse, ClientError> {
        Ok(ChatResponse::new(vec![Message::new(
            "assistant".to_string(),
            vec![self.reply.clone()],
            None,
            None,
        )]))
    }
}

pub(crate)struct FailingClient;

impl ChatClient for FailingClient {
    fn get_response(&self, _messages: &[Message], _instructions: Option<&str>) -> Result<ChatResponse, ClientError> {
        Err(ClientError::Unavailable)
    }
}

#[cfg(test)]
#[path = "fake_client_tests.rs"]
mod tests;