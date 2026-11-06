use maf_rust::{Agent, AgentSession, FakeChatClient};

#[test]
fn public_api_supports_a_conversation(){
    let client = FakeChatClient::new("Hello!".to_string());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let result = agent.run("Hello from user", &mut session);
    let response = result.expect("fake client should succeed");
    assert_eq!(response.text(), "Hello!");
    assert_eq!(session.history().len(), 2);
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[1].role(), "assistant");
    assert_eq!(session.history()[0].text(), "Hello from user");
    assert_eq!(session.history()[1].text(), "Hello!");
}