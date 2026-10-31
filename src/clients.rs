use crate::types::{ChatResponse, Message};

#[derive(Debug, PartialEq, Eq)]// compares errors for equality
pub(crate) enum ClientError {
    Unavailable,
}

pub(crate) trait ChatClient {
    fn get_response(&self, messages: &[Message]) -> Result<ChatResponse, ClientError>;
}