use maf_rust::{Agent, AgentSession, FakeChatClient};

#[tokio::test]
async fn public_api_supports_a_conversation(){
    let client = FakeChatClient::new("Hello!".to_string());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let result = agent.run("Hello from user", &mut session).await;
    let response = result.expect("fake client should succeed");
    assert_eq!(response.text(), "Hello!");
    assert_eq!(session.history().len(), 2);
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[1].role(), "assistant");
    assert_eq!(session.history()[0].text(), "Hello from user");
    assert_eq!(session.history()[1].text(), "Hello!");
}

#[tokio::test]
async fn agent_records_async_calls() {
    let client = FakeChatClient::new("Hello!".to_string());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let pending = agent.run("Hello from user", &mut session); //the right side produces a future. Nothing has run yet:
    drop(pending);
    assert!(session.history().is_empty());
    let result = agent.run("Hello from user", &mut session).await; // await runs the future until it completes
    result.expect("fake client should succeed");
    assert!(session.history().len() == 2);
}