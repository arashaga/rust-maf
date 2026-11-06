use super::Agent;
use crate::fake_client::{FakeChatClient, FailingClient};
use crate::clients::{ChatClient, ClientError};
use crate::types::{ChatResponse, Message};
use std::cell::Cell; // make the interio imutable so we can track call count in &self
use crate::sessions::AgentSession;


#[tokio::test]
async fn agent_returns_client_reply() {
    let reply = "Hello from agent".to_string();
    let client = FakeChatClient::new(reply.clone());
    let  agent = Agent::new(client, None   );
    let mut session = AgentSession::new();
    let response = agent.run("Hello from user", &mut session).await.unwrap();
    assert_eq!(response.text(), reply);
}

struct CheckingClient;

impl ChatClient for CheckingClient {
    async fn get_response(&self, messages: &[Message], _instructions: Option<&str>) -> Result<ChatResponse, ClientError> {
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role(), "user");
        assert_eq!(messages[0].text(), "Hello from user");

        Ok(ChatResponse::new(vec![]))
    }
}

#[tokio::test]
async fn agent_sends_user_message() {
    let client = CheckingClient;
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let _response = agent.run("Hello from user", &mut session).await.unwrap();

}

#[tokio::test]
async fn agent_propagates_client_error() {
    
    let client = FailingClient;
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let response = agent.run("Hi", &mut session).await;
    assert!(matches!(response, Err(ClientError::Unavailable)));
}

#[tokio::test]
async fn agent_returns_empty_input_error() {
    let client = FailingClient;
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let response = agent.run("   ", &mut session).await;
    assert!(matches!(response, Err(ClientError::EmptyInput)));
}



#[tokio::test]
async fn agent_remembers_conversation_history() {
    let reply = "Hello from agent".to_string();
    let client = FakeChatClient::new(reply.clone());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let response1 = agent.run("Hello from user", &mut session).await.unwrap();
    assert_eq!(response1.text(), reply);
    let response2 = agent.run("How are you?", &mut session).await.unwrap();
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
    async fn get_response(&self, messages: &[Message], _instructions: Option<&str>) -> Result<ChatResponse, ClientError> {
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
#[tokio::test]
async fn test_client_that_behaves_differently() {
    let client = ConversationClient::new();
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let response = agent.run("Hi", &mut session).await.expect("first request should succeed");
    let response2 = agent.run("How are you?", &mut session).await.expect("second request should succeed");
    assert_eq!(response2.text(), "Hello");
    assert_eq!(response.text(), "Hello");
}

#[tokio::test]
async fn failed_request_does_not_change_history() {
    let client = FailingClient;
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let result = agent.run("Hi", &mut session).await;
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
    async fn get_response(&self, messages: &[Message], _instructions: Option<&str>) -> Result<ChatResponse, ClientError> {
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


#[tokio::test]
async fn failed_second_request_preserves_history(){
    let client = SucceedOnceClient::new();
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let response = agent.run("Hi", &mut session).await.expect("first request should succeed");
    assert_eq!(response.text(), "Hello");
    let result = agent.run("How are you?", &mut session).await;
    assert!(matches!(result, Err(ClientError::Unavailable)));

    let history = session.history();
    assert_eq!(history.len(), 2);

    assert_eq!(history[0].role(), "user");
    assert_eq!(history[0].text(), "Hi");

    assert_eq!(history[1].role(), "assistant");
    assert_eq!(history[1].text(), "Hello");
}

#[tokio::test]
async fn sessions_keep_their_own_history() {
    let client = FakeChatClient::new("Hello from the agent".to_string());
    let agent = Agent::new(client, None);
    let mut session1 = AgentSession::new();
    let mut session2 = AgentSession::new();
    agent.run("Hi from A", &mut session1).await.expect("request should succeed");
    agent.run("Hello from B", &mut session2).await.expect("request should succeed");
    assert_eq!(session1.history()[0].text(), "Hi from A");
    assert_eq!(session2.history()[0].text(), "Hello from B");
    assert_eq!(session1.history().len(), 2);
    assert_eq!(session2.history().len(), 2);
}

#[test]
fn new_sessions_have_unique_ids() {
    let session1 = AgentSession::new();
    let session2 = AgentSession::new();
    assert_ne!(session1.id(), session2.id());
}

#[test]
fn replacing_history_preserves_session_id() {
    let mut session = AgentSession::new();
    let original_id = session.id().to_string();
    session.replace_history(Vec::new());
    assert_eq!(session.id(), original_id);
}


#[tokio::test]
async fn conversation_preserves_session_id() {
    // 1. Create a FakeChatClient and an Agent.
    // 2. Create a mutable AgentSession.
    // 3. Save an owned copy of its ID.
    // 4. Run the agent with "Hello".
    // 5. Assert that the session ID is unchanged.
    // 6. Assert that history now contains two messages.
    let client = FakeChatClient::new("Hello from the agent".to_string());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let original_id = session.id().to_string();
    agent.run("Hello", &mut session).await.expect("request should succeed");
    assert_eq!(session.id(), original_id);
    assert_eq!(session.history().len(), 2);
}


struct InstructionsCheckingClient;

impl ChatClient for InstructionsCheckingClient {
    async fn get_response(
        &self,
        messages: &[Message],
        instructions: Option<&str>,
    ) -> Result<ChatResponse, ClientError> {
        // Check instructions equal Some("Explain simply").
        // Check there is one user message containing "Hi".
        assert_eq!(instructions, Some("Explain simply"));
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role(), "user");
        assert_eq!(messages[0].text(), "Hi");
        Ok(ChatResponse::new(vec![]))
    }
}


#[tokio::test]
async fn agent_passes_instructions_to_client(){
    let client = InstructionsCheckingClient;
    let agent = Agent::new(client, Some("Explain simply".to_string()));
    let mut session = AgentSession::new();
    agent.run("Hi", &mut session).await.expect("request should succeed"); 
    assert_eq!(session.history().len(), 1);
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[0].text(), "Hi");

}

struct NoInstructionsClient;

impl ChatClient for NoInstructionsClient {
    async fn get_response(
        &self,
        messages: &[Message],
        instructions: Option<&str>,
    ) -> Result<ChatResponse, ClientError> {
        // Check instructions is None.
        assert_eq!(instructions, None);
        Ok(ChatResponse::new(vec![]))
    }
}

#[tokio::test]
async fn agent_passes_none_when_instructions_are_absent(){
    let client = NoInstructionsClient;
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    agent.run("Hello", &mut session).await.expect("request should succeed");
    assert_eq!(session.history().len(), 1);
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[0].text(), "Hello");
}