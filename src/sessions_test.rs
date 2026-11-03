use super::AgentSession;
use crate::types::Message;

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