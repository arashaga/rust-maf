use maf_rust::{Agent, AgentSession, FakeChatClient};
/*
let client = FakeChatClient::new("Hi".to_string());
let agent = Agent::new(client, None);
let mut session = AgentSession::new();

agent
    .run("Hello", &mut session)
    .expect("conversation should succeed");
*/
#[tokio::test]
async fn session_json_round_trip_json_preserves_messages() {

    let client = FakeChatClient::new("Hi".to_string());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();

    agent
        .run("Hello", &mut session)
        .await.expect("conversation should succeed");
   
    
    let json = session.to_json().expect("should serialize");
    let mut restored = AgentSession::from_json(&json).expect("should deserialize");

    agent
    .run("How are you?", &mut restored)
    .await.expect("restored conversation should succeed");

   
    assert_eq!(restored.history().len(), 4);
    assert_eq!(restored.history()[0].role(), "user");
    assert_eq!(restored.history()[0].contents()[0].text(), Some("Hello"));
    assert_eq!(restored.history()[1].role(), "assistant");
    assert_eq!(restored.history()[1].contents()[0].text(), Some("Hi"));
    assert_eq!(restored.history()[2].role(), "user");
    assert_eq!(restored.history()[2].contents()[0].text(), Some("How are you?"));
    assert_eq!(restored.history()[3].role(), "assistant");
    assert_eq!(restored.history()[3].text(), "Hi");
    assert_eq!(session.history().len(), 2);
    assert_eq!(restored.id(), session.id());

}

#[test]
fn session_with_malformed_json_should_fail() {
    let json = r#"{"invalid": "json"#;
    let result = AgentSession::from_json(json);
    assert!(result.is_err());
}