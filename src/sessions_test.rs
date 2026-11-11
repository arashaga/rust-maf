use serde_json::json;

use super::AgentSession;
use crate::types::{Message, Content};


#[test]
fn new_session_has_empty_history() {
    let session = AgentSession::new();
    assert_eq!(session.history().len(), 0);
}

#[test]
fn session_can_replace_history() {
    let mut session = AgentSession::new();
    let messages = vec![Message::new(
            "user".to_string(),
            vec!["Hi".to_string()],
            None,
            None,
    )];
    
    session.replace_history(messages.clone());
    assert_eq!(session.history().len(), 1);
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[0].text(), "Hi");
}

#[test]
fn session_json_round_trip_preserves_id(){
    let original = AgentSession::new();
    let json = serde_json::to_string(&original).expect("session should serialize");
    println!("{}", json);
    let restore: AgentSession = serde_json::from_str(&json).expect("session should deserialize");
    assert_eq!(original.id(), restore.id());
    assert!(restore.history().is_empty());
}

#[test]
fn session_json_round_trip_preserves_messages() {
    let mut session = AgentSession::new();
    let messages = vec![Message::new(
        "user".to_string(),
        vec!["Hello".to_string(), "there".to_string()],
        Some("Learner".to_string()),
        Some("message-1".to_string()),
    ),
    Message::new(
        "assistant".to_string(),
        vec!["Hi".to_string()],
        None,
        None,
    )
    ];
    session.replace_history(messages.clone());

    let json = serde_json::to_string(&session).expect("session should serialize");
    println!("{}", json);
    let restore: AgentSession = serde_json::from_str(&json).expect("session should deserialize");

    assert_eq!(restore.history().len(), 2);
    assert_eq!(restore.history()[0].role(), "user");
    assert_eq!(restore.id(), session.id());
    assert_eq!(restore.history()[0].text(), "Hello there");
    assert_eq!(restore.history()[0].author_name(), Some("Learner"));
    assert_eq!(restore.history()[0].message_id(), Some("message-1"));
    assert_eq!(restore.history()[0].contents().len(), 2);
    assert_eq!(restore.history()[0].contents()[0].text(), Some("Hello"));
    assert_eq!(restore.history()[0].contents()[1].text(), Some("there"));
    assert_eq!(restore.history()[1].role(), "assistant");
    assert_eq!(restore.history()[1].text(), "Hi");
    assert_eq!(restore.history()[1].author_name(), None);
    assert_eq!(restore.history()[1].message_id(), None);
}

#[test]
fn session_json_round_trip_preserves_empty_content() {
    let mut session = AgentSession::new();
    let messages = vec![Message::new(
        "user".to_string(),
        vec![],
        None,
        None,
    ), Message::new(
        "assistant".to_string(),
        vec!["".to_string()],
        None,
        None,
    )];
    session.replace_history(messages.clone());

    let json = serde_json::to_string(&session).expect("session should serialize");
    println!("{}", json);
    let restore: AgentSession = serde_json::from_str(&json).expect("session should deserialize");
    // print the content as a vec
    println!("{:?}", restore.history().iter().map(|m| m.contents()).collect::<Vec<_>>());
    assert_eq!(restore.history().len(), 2);
    assert_eq!(restore.history()[0].role(), "user");
    assert_eq!(restore.history()[0].text(), "");
    assert_eq!(restore.history()[0].author_name(), None);
    assert_eq!(restore.history()[0].message_id(), None);
    assert_eq!(restore.history()[0].contents().len(), 0);
    assert_eq!(restore.history()[1].role(), "assistant");
    assert_eq!(restore.history()[1].text(), "");
    assert_eq!(restore.history()[1].author_name(), None);
    assert_eq!(restore.history()[1].message_id(), None);
    assert_eq!(restore.history()[1].contents().len(), 1);
    assert_eq!(restore.history()[1].contents()[0].text(), Some(""));
}

#[test]
fn session_rejects_malformed_json() {
    let json = "{ this is not valid JSON"; // it won;t mapped

    let result = serde_json::from_str::<AgentSession>(json);

    assert!(result.is_err());
}

#[test]
fn session_rejects_missing_required_fields() {
    let json = "{}"; // missing required fields for AgentSession

    let result = serde_json::from_str::<AgentSession>(json);

    assert!(result.is_err());
}

#[test]
fn session_rejects_numeric_id(){
    let json = r#"{
        "id": 123,
        "history": []
    }"#;

    let result = serde_json::from_str::<AgentSession>(json);

    assert!(result.is_err());
}

#[test]
fn session_rejects_history_that_is_not_an_array(){
    let json = json!({
        "id": "valid-id",
        "history": "not-an-array"
    }).to_string();

    let result = serde_json::from_str::<AgentSession>(&json);

    assert!(result.is_err());
}


#[test]
fn session_json_round_trip_preserves_function_result() {
    let content = Content::from_function_result(
        "c1".to_string(),
        "Found three pages".to_string(),
    );

    let messages = vec![Message::from_contents(
        "tool".to_string(),
        vec![content],
        None,
        None,
    )];
    let mut session = AgentSession::new();
    session.replace_history(messages);

    let json = session.to_json().expect("session should serialize");
    let restored = AgentSession::from_json(&json).expect("session should deserialize");

    assert_eq!(restored.history().len(), 1);
    assert_eq!(restored.history()[0].role(), "tool");
    assert_eq!(restored.history()[0].text(), "");
    assert_eq!(restored.history()[0].contents().len(), 1);
    assert_eq!(restored.id(), session.id());

    let actual = serde_json::to_value(&restored.history()[0].contents()[0])
        .expect("restored function result should serialize");

    let expected = json!({
        "type": "function_result",
        "call_id": "c1",
        "result": "Found three pages",
        "items": [
            {
                "type": "text",
                "text": "Found three pages"
            }
        ]
    });

    assert_eq!(actual, expected);
}