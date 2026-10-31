use crate::clients::ChatClient;
use crate::types::{ChatResponse, Message};
use crate::clients::ClientError;

pub(crate) struct Agent<C: ChatClient> {
    client: C,
}
impl<C: ChatClient> Agent<C> {
    // Your constructor and run method go here.
    pub(crate) fn new(client: C) -> Self {
        Self { client }
    }

    pub(crate) fn run(&self, input: &str) -> Result<ChatResponse, ClientError> {
        if input.trim().is_empty() {
            return Err(ClientError::EmptyInput);
        }
        let messages = vec![Message::new(
            "user".to_string(),
            vec![input.to_string()],
            None,
            None,
        )];
        self.client.get_response(&messages)
    }
}



#[cfg(test)]
#[path = "agent_tests.rs"]
mod tests;