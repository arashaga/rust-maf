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

#[test]
fn text_content_serializes_with_maf_type_tag() {
    let content = super::Content::Text {
        text: "Hello".to_string(),
        annotations: None,
    };

    let json = serde_json::to_value(content)
        .expect("content should serialize");

    assert_eq!(
        json,
        serde_json::json!({
            "type": "text",
            "text": "Hello"
        })
    );
}

#[test]
fn functiona_call_omits_no_arguments(){
    let content = super::Content::FunctionCall { 
        call_id: "xzy".to_string(), 
        name: "search".to_string(), 
        arguments: None };

    let json = serde_json::to_value(content)
    .expect("content should be serialized");

    assert_eq!(json,
    serde_json::json!({
        "type": "function_call", // comes from the tag and changes FunctionCall to snake case
        "call_id": "xzy",
        "name": "search"
    }))
}

#[test]
fn function_call_preserves_provided_arguments(){
        let content = super::Content::FunctionCall { 
        call_id: "xzy".to_string(), 
        name: "search".to_string(), 
        arguments: Some(serde_json::json!({
            "query" : "Rust"
        }))};

    let json = serde_json::to_value(content)
    .expect("content should be serialized");

    assert_eq!(json,
    serde_json::json!({
        "type": "function_call", // comes from the tag and changes FunctionCall to snake case
        "call_id": "xzy",
        "name": "search",
        "arguments" : {"query" : "Rust"}
    }))
}

#[test]
fn function_call_preserves_raw_string_arguments(){
            let content = super::Content::FunctionCall { 
        call_id: "xzy".to_string(), 
        name: "search".to_string(), 
        arguments: Some(serde_json::json!("{"))
    };

    let json = serde_json::to_value(content)
    .expect("content should be serialized");

    assert_eq!(json,
    serde_json::json!({
        "type": "function_call", // comes from the tag and changes FunctionCall to snake case
        "call_id": "xzy",
        "name": "search",
        "arguments" : "{"
    }))
}

#[test]
fn function_call_without_arguments_round_trips() {
    let original = serde_json::json!({
        "type": "function_call",
        "call_id": "xzy",
        "name": "search"
    });

    // clone it so you can do the final comparsion since from_value take the ownerhship
    let content: super::Content =
        serde_json::from_value(original.clone())
            .expect("function call should deserialize");

    let restored_json = serde_json::to_value(content)
        .expect("function call should serialize");

    assert_eq!(restored_json, original);
}

#[test]
fn content_text_returns_only_ordinary_test(){
        let text_content = super::Content::Text {
        text: "Hello".to_string(),
        annotations: None,
    };

    let function_call = super::Content::FunctionCall {
        call_id: "c1".to_string(),
        name: "search".to_string(),
        arguments: None,
    };

    assert_eq!(text_content.text(), Some("Hello"));
    assert_eq!(function_call.text(), None);
}

#[test]
fn message_text_skips_function_calls() {
    let json = serde_json::json!({
        "role": "assistant",
        "contents": [
            {
                "type": "text",
                "text": "Checking now."
            },
            {
                "type": "function_call",
                "call_id": "c1",
                "name": "search",
                "arguments": {"query": "Rust"}
            },
            {
                "type": "text",
                "text": "Here is the result."
            }
        ]
    });

    let message: super::Message = serde_json::from_value(json)
        .expect("message should accept structured content");

    assert_eq!(
        message.text(),
        "Checking now. Here is the result."
    );
    assert_eq!(message.contents().len(), 3);
}

#[test]
fn content_from_text_preserves_text() {
    let content = super::Content::from_text("Hello".to_string());

    assert_eq!(content.text(), Some("Hello"));
}

#[test]
fn function_result_from_text_matches_maf_shape() {
    let content = super::Content::from_function_result(
        "c1".to_string(),
        "Found three pages".to_string(),
    );

    let actual = serde_json::to_value(content)
        .expect("function result should serialize");

    let expected = serde_json::json!({
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

#[test]
fn message_text_skips_function_results() {
        let content = super::Content::from_function_result(
        "c1".to_string(),
        "Found three pages".to_string(),
    );

    let message = Message::from_contents("tool".to_string(), 
    vec![content], None, None);


    assert_eq!(message.text(), "");
    assert_eq!(message.contents.len(),1);
    assert_eq!(message.contents[0].text(),None);
}


#[test]
fn message_text_preserves_text_around_function_result() {
        let content = super::Content::from_function_result(
        "c1".to_string(),
        "Found three pages".to_string(),
        );

        let before = super::Content::Text { text: "Before".to_string(), annotations: None };
        let after = super::Content::Text { text: "After".to_string(), annotations: None };
        let message = Message::from_contents("tool".to_string(), 
    vec![before,content, after], None, None);

    assert_eq!(message.text(),"Before After");
    assert_eq!(message.contents.len(),3);
    assert_eq!(message.contents[1].text(),None);
}

#[test]
fn function_result_from_items_preserves_order_and_joins_text_with_newlines() {
    let content = super::Content::from_function_result_items(
        "c1".to_string(),
        vec![
            super::Content::from_text("First page".to_string()),
            super::Content::from_text("Second page".to_string()),
        ],
    );

    let actual = serde_json::to_value(content)
        .expect("function result should serialize");

    assert_eq!(
        actual,
        serde_json::json!({
            "type": "function_result",
            "call_id": "c1",
            "result": "First page\nSecond page",
            "items": [
                {"type": "text", "text": "First page"},
                {"type": "text", "text": "Second page"}
            ]
        })
    );
}

#[test]
fn function_result_from_items_skips_empty_text_but_keeps_whitespace() {
    let content = super::Content::from_function_result_items(
        "c1".to_string(),
        vec![
            super::Content::from_text("First".to_string()),
            super::Content::from_text(String::new()),
            super::Content::from_text(" ".to_string()),
            super::Content::from_text("Second".to_string()),
        ],
    );

    let actual = serde_json::to_value(content)
        .expect("function result should serialize");

    assert_eq!(
        actual,
        serde_json::json!({
            "type": "function_result",
            "call_id": "c1",
            "result": "First\n \nSecond",
            "items": [
                {"type": "text", "text": "First"},
                {"type": "text", "text": ""},
                {"type": "text", "text": " "},
                {"type": "text", "text": "Second"}
            ]
        })
    );
}

#[test]
fn function_result_from_items_preserves_nested_result_without_flattening() {
    let content = super::Content::from_function_result_items(
        "c1".to_string(),
        vec![
            super::Content::from_text("Before".to_string()),
            super::Content::from_function_result(
                "c2".to_string(),
                "Nested text".to_string(),
            ),
            super::Content::from_text("After".to_string()),
        ],
    );

    let actual = serde_json::to_value(content)
        .expect("function result should serialize");

    assert_eq!(
        actual,
        serde_json::json!({
            "type": "function_result",
            "call_id": "c1",
            "result": "Before\nAfter",
            "items": [
                {"type": "text", "text": "Before"},
                {
                    "type": "function_result",
                    "call_id": "c2",
                    "result": "Nested text",
                    "items": [
                        {"type": "text", "text": "Nested text"}
                    ]
                },
                {"type": "text", "text": "After"}
            ]
        })
    );
}

//Next, check an empty list. MAF preserves "items": [] and produces an empty summary—not null or a missing field.

#[test]
fn function_result_from_items_preserves_empty_list() {
    let content = super::Content::from_function_result_items(
        "c1".to_string(),
        vec![],
    );

    let actual = serde_json::to_value(content)
        .expect("empty function result should serialize");

    assert_eq!(
        actual,
        serde_json::json!({
            "type": "function_result",
            "call_id": "c1",
            "result": "",
            "items": []
        })
    );
}

//MAF has a content kind named text_reasoning, distinct from ordinary text. We’re preserving explicitly returned provider content—not obtaining private internal reasoning.
#[test]
fn text_reasoning_content_round_trips_without_becoming_answer_text() {
    let original = serde_json::json!({
        "type": "text_reasoning",
        "text": "Compared the available options."
    });

    let content: super::Content =
        serde_json::from_value(original.clone())
            .expect("reasoning content should deserialize");

    assert_eq!(content.text(), None);

    let restored = serde_json::to_value(&content)
        .expect("reasoning content should serialize");

    assert_eq!(restored, original);
}


#[test]
fn text_reasoning_content_preserves_absent_and_empty_text() {
    for original in [
        serde_json::json!({"type": "text_reasoning"}),
        serde_json::json!({"type": "text_reasoning", "text": ""}),
    ] {
        let content: super::Content =
            serde_json::from_value(original.clone())
                .expect("reasoning content should deserialize");

        let actual = serde_json::to_value(&content)
            .expect("reasoning content should serialize");

        assert_eq!(actual, original);
    }
}

//verify that a message retains reasoning content without including it in its answer-text view.
#[test]
fn message_text_skips_reasoning_but_preserves_content() {
    let original = serde_json::json!({
        "role": "assistant",
        "contents": [
            {
                "type": "text_reasoning",
                "text": "Compared the available options."
            },
            {
                "type": "text",
                "text": "Choose option A."
            }
        ]
    });

    let message: super::Message =
        serde_json::from_value(original.clone())
            .expect("message should accept reasoning content");

    assert_eq!(message.text(), "Choose option A.");

    let stored_contents = serde_json::to_value(message.contents())
        .expect("message contents should serialize");

    assert_eq!(stored_contents, original["contents"]);
}

//annotation test
#[test]
fn citation_annotation_round_trips() {
    let json = serde_json::json!({
        "type": "citation",
        "title": "The Rust Programming Language",
        "url": "https://doc.rust-lang.org/book/"
    });

    let annotation: super::Annotation =
        serde_json::from_value(json.clone()).unwrap();

    let restored = serde_json::to_value(&annotation).unwrap();

    assert_eq!(restored, json);
}

#[test]
fn citation_annotation_omits_absent_fields() {
    for original in [
        serde_json::json!({"type": "citation"}),
        serde_json::json!({
            "type": "citation",
            "title": "Rust Book"
        }),
        serde_json::json!({
            "type": "citation",
            "url": "https://doc.rust-lang.org/book/"
        }),
        serde_json::json!({
            "type": "citation",
            "title": "",
            "url": ""
        }),
    ] {
        let annotation: super::Annotation =
            serde_json::from_value(original.clone())
                .expect("citation should deserialize");

        let restored = serde_json::to_value(&annotation)
            .expect("citation should serialize");

        assert_eq!(restored, original);
    }
}

#[test]
fn text_content_preserves_optional_citation_annotations() {
    for original in [
        serde_json::json!({
            "type": "text",
            "text": "Rust source",
            "annotations": [{
                "type": "citation",
                "title": "The Rust Programming Language",
                "url": "https://doc.rust-lang.org/book/"
            }]
        }),
        serde_json::json!({
            "type": "text",
            "text": "Plain text"
        }),
    ] {
        let content: super::Content =
            serde_json::from_value(original.clone())
                .expect("text content should deserialize");

        assert_eq!(content.text(), original["text"].as_str());

        let restored = serde_json::to_value(&content)
            .expect("text content should serialize");

        assert_eq!(restored, original);
    }
}

#[test]
fn message_preserves_citation_without_adding_it_to_answer_text() {
    let original = serde_json::json!({
        "role": "assistant",
        "contents": [{
            "type": "text",
            "text": "Rust source",
            "annotations": [{
                "type": "citation",
                "title": "The Rust Programming Language",
                "url": "https://doc.rust-lang.org/book/"
            }]
        }]
    });

    let message: super::Message =
        serde_json::from_value(original.clone())
            .expect("message should accept annotated text");

    assert_eq!(message.text(), "Rust source");

    let stored_contents = serde_json::to_value(message.contents())
        .expect("message contents should serialize");

    assert_eq!(stored_contents, original["contents"]);
}

#[test]
fn response_keeps_optional_id_without_changing_text() {
    let message = Message::new(
        "assistant".to_string(),
        vec!["Hello".to_string()],
        None,
        None,
    );

    let mut response = super::ChatResponse::new(vec![message]);

    assert_eq!(response.response_id(), None);

    response.set_response_id(Some("resp-123".to_string()));
    assert_eq!(response.response_id(), Some("resp-123"));
    assert_eq!(response.text(), "Hello");

    response.set_response_id(None);
    assert_eq!(response.response_id(), None);
}

#[test]
fn response_json_round_trip_preserves_response_id() {
    let mut response = super::ChatResponse::new(vec![]);
    response.set_response_id(Some("resp-123".to_string()));

    let json = serde_json::to_value(&response)
        .expect("response should serialize");

    assert_eq!(
        json,
        serde_json::json!({
            "type": "chat_response",
            "messages": [],
            "response_id": "resp-123"
        })
    );

    let restored: super::ChatResponse = serde_json::from_value(json)
        .expect("response should deserialize");

    assert_eq!(restored.response_id(), Some("resp-123"));
    assert!(restored.messages().is_empty());
}

#[test]
fn response_json_omits_absent_response_id() {
    let response = super::ChatResponse::new(vec![]);

    let json = serde_json::to_value(&response)
        .expect("response should serialize");

    assert_eq!(
        json,
        serde_json::json!({
            "type": "chat_response",
            "messages": []
        })
    );

    let restored: super::ChatResponse = serde_json::from_value(json)
        .expect("response should deserialize");

    assert_eq!(restored.response_id(), None);
}

#[test]
fn response_json_rejects_mismatched_type() {
    for json in [
        serde_json::json!({"type": "chat_response", "messages": []}),
        serde_json::json!({"messages": []}),
    ] {
        let result: Result<super::ChatResponse, _> =
            serde_json::from_value(json);

        assert!(result.is_ok(), "valid response JSON should deserialize");
    }

    let wrong_type = serde_json::json!({
        "type": "message",
        "messages": []
    });

    let result: Result<super::ChatResponse, _> = //Rust, infer the error type for me.
        serde_json::from_value(wrong_type);

    assert!(result.is_err(), "a message type is not a chat response");
}

#[test]
fn response_json_accepts_null_and_empty_type() {
    for (description, json) in [
        ("null", serde_json::json!({"type": null, "messages": []})),// remember null is a valid JSON value. don;t pass None
        ("empty", serde_json::json!({"type": "", "messages": []})),
    ] {
        let result: Result<super::ChatResponse, _> =
            serde_json::from_value(json);

        assert!(
            result.is_ok(),
            "{description} type should be accepted; got {result:?}"
        );
    }
}