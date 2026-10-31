use super::Message;
#[test]
fn text_joins_multiple_contents() {
    // Arrange: construct a Message with "Hello" and "there".
    let message = Message::new(
        "user".to_string(),
        vec!["Hello".to_string(), "there".to_string()],
        Some("Arash".to_string()),
        Some("123".to_string()),
    );

    // Act: call its text() method.
    let text = message.text();
    // Assert: check that the result equals "Hello there".
    assert_eq!(text, "Hello there");
}

#[test]
fn empty_text_is_preserved_as_one_item() {
    // Your turn.
    let message = Message::new(
        "assistant".to_string(),
        vec!["".to_string()],
        None,
        None,
    );

    let text = message.text();
    assert_eq!(text, "");
    assert_eq!(message.contents.len(), 1);
}

#[test]
fn no_contents_produces_empty_text() {
    // Your implementation.
    let message = Message::new(
        "assistant".to_string(),
        vec![],
        None,
        None,
    );

    let text = message.text();
    assert_eq!(text, "");
    assert_eq!(message.contents.len(), 0);
}

#[test]
fn response_preserves_message() {

    let message = Message::new(
        "user".to_string(),
        vec!["Hello".to_string()],
        Some("Arash".to_string()),
        Some("123".to_string()),
    );

    let response = super::ChatResponse::new(vec![message]);

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].text(), "Hello");
}

#[test]
fn response_preserves_message_order() {

    let message1 = Message::new(
        "user".to_string(),
        vec!["Hello".to_string()],
        Some("Arash".to_string()),
        Some("123".to_string()),
    );
    let message2 = Message::new(
        "assistant".to_string(),
        vec!["Hi".to_string()],
        None,
        None,
    );

    let response = super::ChatResponse::new(vec![message1, message2]);

    assert_eq!(response.messages.len(), 2);
    assert_eq!(response.messages[0].text(), "Hello");
    assert_eq!(response.messages[1].text(), "Hi");
}

#[test]
fn response_test_combines_messages(){
    let message1 = Message::new(
        "user".to_string(),
        vec![" Hello".to_string()],
        Some("Arash".to_string()),
        Some("123".to_string()),
    );
    let message2 = Message::new(
        "assistant".to_string(),
        vec!["Hi ".to_string()],
        None,
        None,
    );

    let response = super::ChatResponse::new(vec![message1, message2]);

    let combined_text = response.text();
    assert_eq!(combined_text, "Hello\nHi");
}

#[test]
fn response_with_empty_messages_produces_empty_text() {
    let response = super::ChatResponse::new(vec![]);

    let combined_text = response.text();
    assert_eq!(combined_text, "");
    assert_eq!(response.messages.len(), 0);
}

#[test]
fn response_with_trimming() {
    let message1 = Message::new(
        "user".to_string(),
        vec!["Hello ".to_string()],
        Some("Arash".to_string()),
        Some("123".to_string()),
    );
    let message2 = Message::new(
        "assistant".to_string(),
        vec![" Hi".to_string()],
        None,
        None,
    );

    let response = super::ChatResponse::new(vec![message1, message2]);

    let combined_text = response.text();
    assert_eq!(combined_text, "Hello \n Hi");
}