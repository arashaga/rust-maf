use crate::types::{ChatResponse, Message};

#[derive(Debug, PartialEq, Eq)]// compares errors for equality
pub enum ClientError {
    Unavailable,
    EmptyInput,
}

pub trait ChatClient {
    async fn get_response(&self, messages: &[Message], instructions: Option<&str>) -> Result<ChatResponse, ClientError>;
}