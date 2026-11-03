use super::Agent;
use crate::fake_client::{FakeChatClient, FailingClient};
use crate::clients::{ChatClient, ClientError};
use crate::types::{ChatResponse, Message};
use std::cell::Cell; // make the interio imutable so we can track call count in &self
use crate::sessions::AgentSession;

#[test]
fn agent_returns_client_reply() {
    let reply = "Hello from agent".to_string();
    let client = FakeChatClient::new(reply.clone());
    let  agent = Agent::new(client);
    let mut session = AgentSession::new();
    let response = agent.run("Hello from user", &mut session).unwrap();
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
    let mut session = AgentSession::new();
    let _response = agent.run("Hello from user", &mut session).unwrap();

}

#[test]
fn agent_propagates_client_error() {
    
    let client = FailingClient;
    let agent = Agent::new(client);
    let mut session = AgentSession::new();
    let response = agent.run("Hi", &mut session);
    assert!(matches!(response, Err(ClientError::Unavailable)));
}

#[test]
fn agent_returns_empty_input_error() {
    let client = FailingClient;
    let agent = Agent::new(client);
    let mut session = AgentSession::new();
    let response = agent.run("   ", &mut session);
    assert!(matches!(response, Err(ClientError::EmptyInput)));
}



#[test]
fn agent_remembers_conversation_history() {
    let reply = "Hello from agent".to_string();
    let client = FakeChatClient::new(reply.clone());
    let agent = Agent::new(client);
    let mut session = AgentSession::new();
    let response1 = agent.run("Hello from user", &mut session).unwrap();
    assert_eq!(response1.text(), reply);
    let response2 = agent.run("How are you?", &mut session).unwrap();
    assert_eq!(response2.text(), reply);
}

struct ConversationClient {
    calls: Cell<usize>,
}

impl ConversationClient {
    fn new() -> Self {
        Self {
            calls: Cell::new(0),
        }
    }


}

impl ChatClient for ConversationClient {
        fn get_response(&self, messages: &[Message]) -> Result<ChatResponse, ClientError> {
        let call_number = self.calls.get() +1;
        self.calls.set(call_number);
        
        if call_number == 1 {
            assert_eq!(messages.len(), 1);
            assert_eq!(messages[0].role(), "user");
            assert_eq!(messages[0].text(), "Hi");

            
        } else {
            assert_eq!(messages.len(), 3);
            assert_eq!(messages[0].role(), "user");
            assert_eq!(messages[0].text(), "Hi");
            assert_eq!(messages[1].role(), "assistant");
            assert_eq!(messages[1].text(), "Hello");
            assert_eq!(messages[2].role(), "user");
            assert_eq!(messages[2].text(), "How are you?");
        }
        let messages = vec![Message::new(
            "assistant".to_string(),
            vec!["Hello".to_string()],
            None,
            None,
        )];
    return Ok(ChatResponse::new(messages));
    }
}

/*
Construct ConversationClient::new().
Call client.get_response(...) with one user message containing "Hi".
Use .expect("first request should succeed").
Assert that the response text is "Hello"
*/
#[test]
fn test_client_that_behaves_differently() {
    let client = ConversationClient::new();
    let agent = Agent::new(client);
    let mut session = AgentSession::new();
    let response = agent.run("Hi", &mut session).expect("first request should succeed");
    let response2 = agent.run("How are you?", &mut session).expect("second request should succeed");
    assert_eq!(response2.text(), "Hello");
    assert_eq!(response.text(), "Hello");
}

#[test]
fn failed_request_does_not_change_history() {
    let client = FailingClient;
    let agent = Agent::new(client);
    let mut session = AgentSession::new();
    let result = agent.run("Hi", &mut session);
    assert!(matches!(result, Err(ClientError::Unavailable)));
    assert!(session.history().is_empty());
}

struct SucceedOnceClient {
    calls: Cell<usize>,
}

impl SucceedOnceClient {
    fn new() -> Self {
        Self {
            calls: Cell::new(0),
        }
    }
}

/*
Create a test client named SucceedsOnceClient.
Give it a Cell<usize> call counter, just like ConversationClient.
Implement ChatClient:
First call: return an assistant message containing "Hello".
Second call: return Err(ClientError::Unavailable).
 */

impl ChatClient for SucceedOnceClient {
    fn get_response(&self, messages: &[Message]) -> Result<ChatResponse, ClientError> {
        let call_number = self.calls.get() + 1;
        self.calls.set(call_number);

        if call_number == 1 {
        
            let messages = vec![Message::new(
                "assistant".to_string(),
                vec!["Hello".to_string()],
                None,
                None,
            )];
            Ok(ChatResponse::new(messages))
        } else {
            Err(ClientError::Unavailable)
        }
    }
}


#[test]
fn failed_second_request_preserves_history(){
    let client = SucceedOnceClient::new();
    let agent = Agent::new(client);
    let mut session = AgentSession::new();
    let response = agent.run("Hi", &mut session).expect("first request should succeed");
    assert_eq!(response.text(), "Hello");
    let result = agent.run("How are you?", &mut session);
    assert!(matches!(result, Err(ClientError::Unavailable)));

    let history = session.history();
    assert_eq!(history.len(), 2);

    assert_eq!(history[0].role(), "user");
    assert_eq!(history[0].text(), "Hi");

    assert_eq!(history[1].role(), "assistant");
    assert_eq!(history[1].text(), "Hello");
}