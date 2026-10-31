use super::Agent;
use crate::fake_client::{FakeChatClient, FailingClient};
use crate::clients::{ChatClient, ClientError};
use crate::types::{ChatResponse, Message};

#[test]
fn agent_returns_client_reply() {
    let reply = "Hello from agent".to_string();
    let client = FakeChatClient::new(reply.clone());
    let agent = Agent::new(client);
    let response = agent.run("Hello from user").unwrap();
    assert_eq!(response.text(), reply);
}

struct CheckingClient;

impl ChatClient for CheckingClient {
    fn get_response(&self, messages: &[Message]) -> Result<ChatResponse, ClientError> {
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role(), "user");
        assert_eq!(messages[0].text(), "Hello from user");

        Ok(ChatResponse::new(vec![]))
    }
}

#[test]
fn agent_sends_user_message() {
    let client = CheckingClient;
    let agent = Agent::new(client);
    let _response = agent.run("Hello from user").unwrap();

}

#[test]
fn agent_propagates_client_error() {
    
    let client = FailingClient;
    let agent = Agent::new(client);
    let response = agent.run("Hi");
    assert!(matches!(response, Err(ClientError::Unavailable)));
}

#[test]
fn agent_returns_empty_input_error() {
    let client = FailingClient;
    let agent = Agent::new(client);
    let response = agent.run("   ");
    assert!(matches!(response, Err(ClientError::EmptyInput)));
}