use maf_rust::{AzureOpenAIConfig, Message, AzureOpenAIRequestError};

#[test]
fn request_body_maps_user_text_to_responses_input() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::new(
        "user".to_string(),
        vec!["Hello".to_string()],
        None,
        None,
    )];

    let body = config
        .build_request_body(&messages, None)
        .expect("ordinary user text should map to a request body");

    assert_eq!(
        body,
        serde_json::json!({
            "model": "learning-deployment",
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Hello"
                }]
            }]
        })
    );
}

#[test]
fn request_body_rejects_text_with_citation() {
    use maf_rust::{Annotation, AzureOpenAIRequestError, Content};

    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let cited_text = Content::Text {
        text: "Hello".to_string(),
        annotations: Some(vec![Annotation::citation {
            title: Some("A source".to_string()),
            url: Some("https://example.com".to_string()),
        }]),
    };

    let messages = vec![Message::from_contents(
        "user".to_string(),
        vec![cited_text],
        None,
        None,
    )];

    let result = config.build_request_body(&messages, None);

    assert!(
        matches!(result, Err(AzureOpenAIRequestError::AnnotatedText)),
        "text with a citation should be rejected"
    );
}

#[test]
fn request_body_accepts_empty_annotations() {
    use maf_rust::Content;

    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let text = Content::Text {
        text: "Hello".to_string(),
        annotations: Some(vec![]),
    };

    let messages = vec![Message::from_contents(
        "user".to_string(),
        vec![text],
        None,
        None,
    )];

    let body = config
        .build_request_body(&messages, None)
        .expect("an empty annotation list should be accepted");

    assert_eq!(
        body,
        serde_json::json!({
            "model": "learning-deployment",
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Hello"
                }]
            }]
        })
    );
}

#[test]
fn request_body_rejects_unsupported_function_call() {
    use maf_rust::{AzureOpenAIRequestError, Content};

    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::from_contents(
        "assistant".to_string(),
        vec![
            Content::from_text("Searching".to_string()),
            Content::FunctionCall {
                call_id: "call-1".to_string(),
                name: "search".to_string(),
                arguments: Some(serde_json::json!({"query": "rust"})),
            },
        ],
        None,
        None,
    )];

    let result = config.build_request_body(&messages, None);

    assert!(
        matches!(result, Err(AzureOpenAIRequestError::UnsupportedContent)),
        "unsupported content must not be silently dropped"
    );
}

#[test]
fn request_body_includes_top_level_instructions() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::new(
        "user".to_string(),
        vec!["Hello".to_string()],
        None,
        None,
    )];

    let body = config
        .build_request_body(&messages, Some("Reply in uppercase."))
        .expect("user text and instructions should map successfully");

    assert_eq!(
        body,
        serde_json::json!({
            "model": "learning-deployment",
            "instructions": "Reply in uppercase.",
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Hello"
                }]
            }]
        })
    );
}

#[test]
fn request_body_preserves_empty_instructions() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::new(
        "user".to_string(),
        vec!["Hello".to_string()],
        None,
        None,
    )];

    let body = config
        .build_request_body(&messages, Some(""))
        .expect("empty instructions should be accepted");

    assert_eq!(
        body,
        serde_json::json!({
            "model": "learning-deployment",
            "instructions": "",
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Hello"
                }]
            }]
        })
    );
}

#[test]
fn request_body_labels_user_input_as_message() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::new(
        "user".to_string(),
        vec!["Hello".to_string()],
        None,
        None,
    )];

    let body = config
        .build_request_body(&messages, None)
        .expect("user text should map successfully");

    assert_eq!(body["input"][0]["type"], serde_json::json!("message"));
}

#[test]
fn request_body_maps_assistant_history_as_output_text() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![
        Message::new("user".to_string(), vec!["Hi".to_string()], None, None),
        Message::new("assistant".to_string(), vec!["Hello".to_string()], None, None),
    ];

    let body = config
        .build_request_body(&messages, None)
        .expect("user and assistant text should map successfully");

    assert_eq!(
        body,
        serde_json::json!({
            "model": "learning-deployment",
            "input": [
                {
                    "type": "message",
                    "role": "user",
                    "content": [{
                        "type": "input_text",
                        "text": "Hi"
                    }]
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "content": [{
                        "type": "output_text",
                        "text": "Hello",
                        "annotations": []
                    }]
                }
            ]
        })
    );
}

#[test]
fn request_body_rejects_system_role_in_initial_subset() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::new(
        "system".to_string(),
        vec!["Use concise answers.".to_string()],
        None,
        None,
    )];

    let result = config.build_request_body(&messages, None);

    assert!(
        matches!(result, Err(AzureOpenAIRequestError::UnsupportedRole)),
        "system messages are outside this initial mapper subset"
    );
}

#[test]
fn request_body_rejects_empty_input() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let result = config.build_request_body(&[], None);

    assert!(
        matches!(result, Err(AzureOpenAIRequestError::EmptyInput)),
        "an empty request input should be rejected"
    );
}

#[test]
fn request_body_rejects_message_without_content() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::from_contents(
        "user".to_string(),
        vec![],
        None,
        None,
    )];

    let result = config.build_request_body(&messages, None);

    assert!(
        matches!(result, Err(AzureOpenAIRequestError::EmptyInput)),
        "a message with no content should not create a request by itself"
    );
}

#[test]
fn request_body_omits_empty_message_when_other_input_exists() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![
        Message::from_contents("user".to_string(), vec![], None, None),
        Message::new("user".to_string(), vec!["Hello".to_string()], None, None),
    ];

    let body = config
        .build_request_body(&messages, None)
        .expect("the nonempty message should provide valid input");

    assert_eq!(
        body,
        serde_json::json!({
            "model": "learning-deployment",
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Hello"
                }]
            }]
        })
    );
}

#[test]
fn request_body_preserves_text_item_order() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::new(
        "user".to_string(),
        vec!["First part".to_string(), "Second part".to_string()],
        None,
        None,
    )];

    let body = config
        .build_request_body(&messages, None)
        .expect("multiple user text parts should map successfully");

    assert_eq!(
        body["input"][0]["content"],
        serde_json::json!([
            {
                "type": "input_text",
                "text": "First part"
            },
            {
                "type": "input_text",
                "text": "Second part"
            }
        ])
    );
}

#[test]
fn request_body_preserves_empty_text_item() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let messages = vec![Message::new(
        "user".to_string(),
        vec!["".to_string()],
        None,
        None,
    )];

    let body = config
        .build_request_body(&messages, None)
        .expect("a Text item with empty text should still be mapped");

    assert_eq!(
        body["input"][0]["content"],
        serde_json::json!([
            {
                "type": "input_text",
                "text": ""
            }
        ])
    );
}

#[test]
fn response_maps_single_output_text_to_assistant_message() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": [{
            "type": "message",
            "id": "msg_123",
            "role": "assistant",
            "content": [{
                "type": "output_text",
                "text": "Hello",
                "annotations": []
            }]
        }]
    });

    let chat_response = config
        .parse_response(&provider_response)
        .expect("a simple text response should map");

    assert_eq!(chat_response.response_id(), Some("resp_123"));
    assert_eq!(chat_response.messages().len(), 1);

    let message = &chat_response.messages()[0];
    assert_eq!(message.role(), "assistant");
    assert_eq!(message.message_id(), Some("msg_123"));
    assert_eq!(message.text(), "Hello");
}

#[test]
fn response_rejects_unsupported_content_without_dropping_it() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": [{
            "type": "message",
            "id": "msg_123",
            "role": "assistant",
            "content": [
                {
                    "type": "output_text",
                    "text": "Hello",
                    "annotations": []
                },
                {
                    "type": "refusal",
                    "refusal": "I cannot help with that."
                }
            ]
        }]
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "unsupported content must not be silently discarded"
    );
}

#[test]
fn response_rejects_unsupported_output_without_dropping_it() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": [
            {
                "type": "message",
                "id": "msg_123",
                "role": "assistant",
                "content": [{
                    "type": "output_text",
                    "text": "Hello",
                    "annotations": []
                }]
            },
            {
                "type": "function_call",
                "id": "fc_123",
                "call_id": "call_123",
                "name": "search",
                "arguments": "{}",
                "status": "completed"
            }
        ]
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "unsupported output must not be silently discarded"
    );
}

#[test]
fn response_rejects_text_with_citation() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": [{
            "type": "message",
            "id": "msg_123",
            "role": "assistant",
            "content": [{
                "type": "output_text",
                "text": "Rust is fast.",
                "annotations": [{
                    "type": "url_citation",
                    "title": "A source",
                    "url": "https://example.com",
                    "start_index": 0,
                    "end_index": 4
                }]
            }]
        }]
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "a citation must not be silently discarded"
    );
}

#[test]
fn response_maps_empty_output_to_empty_assistant_message() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": []
    });

    let response = config
        .parse_response(&provider_response)
        .expect("empty output should map successfully");

    assert_eq!(response.response_id(), Some("resp_123"));
    assert_eq!(response.messages().len(), 1);

    let message = &response.messages()[0];
    assert_eq!(message.role(), "assistant");
    assert!(message.contents().is_empty());
    assert_eq!(message.message_id(), None);
}

#[test]
fn response_preserves_message_order_and_ids() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": [
            {
                "type": "message",
                "id": "msg_1",
                "role": "assistant",
                "content": [{
                    "type": "output_text",
                    "text": "First",
                    "annotations": []
                }]
            },
            {
                "type": "message",
                "id": "msg_2",
                "role": "assistant",
                "content": [{
                    "type": "output_text",
                    "text": "Second",
                    "annotations": []
                }]
            }
        ]
    });

    let response = config
        .parse_response(&provider_response)
        .expect("both messages should map successfully");

    let messages = response.messages();
    assert_eq!(messages.len(), 2);

    assert_eq!(messages[0].message_id(), Some("msg_1"));
    assert_eq!(messages[0].text(), "First");

    assert_eq!(messages[1].message_id(), Some("msg_2"));
    assert_eq!(messages[1].text(), "Second");
}

#[test]
fn response_rejects_non_assistant_message_role() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": [{
            "type": "message",
            "id": "msg_123",
            "role": "user",
            "content": [{
                "type": "output_text",
                "text": "Hello",
                "annotations": []
            }]
        }]
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "a provider output message must not become a user message"
    );
}

#[test]
fn response_rejects_output_that_is_not_an_array() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_123",
        "output": {} //<- json not an array
    });

    let result = config.parse_response(&provider_response);

    assert!(result.is_err(), "output must be an array");
}

#[test]
fn response_rejects_failed_provider_status() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_failed",
        "status": "failed",
        "error": {
            "code": "server_error",
            "message": "The provider failed"
        },
        "output": []
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "a failed response must not become a successful empty message"
    );
}

#[test]
fn response_failure_includes_provider_error_message() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_failed",
        "status": "failed",
        "error": {
            "code": "server_error",
            "message": "The provider failed"
        },
        "output": []
    });

    let error = config
        .parse_response(&provider_response)
        .expect_err("a failed provider response should return an error");

    assert!(
        error.contains("The provider failed"),
        "the error should include the provider's message, got: {error}"
    );
}

#[test]
fn response_failed_without_provider_message_keeps_generic_error() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_failed",
        "status": "failed",
        "error": {
            "code": "server_error"
        },
        "output": []
    });

    let error = config
        .parse_response(&provider_response)
        .expect_err("a failed provider response should return an error");

    assert!(
        error.contains("failed response"),
        "a missing provider message should preserve the generic failure, got: {error}"
    );
}

#[test]
fn response_failure_includes_provider_error_code_and_message() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_failed",
        "status": "failed",
        "error": {
            "code": "server_error",
            "message": "The provider failed"
        },
        "output": []
    });

    let error = config
        .parse_response(&provider_response)
        .expect_err("a failed provider response should return an error");

    assert!(
        error.contains("server_error"),
        "the error should include the provider code, got: {error}"
    );
    assert!(
        error.contains("The provider failed"),
        "the error should include the provider message, got: {error}"
    );
}

#[test]
fn response_failure_without_code_uses_message_only() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_failed",
        "status": "failed",
        "error": {
            "message": "The provider failed"
        },
        "output": []
    });

    let error = config
        .parse_response(&provider_response)
        .expect_err("a failed provider response should return an error");

    assert_eq!(
        error.as_str(),
        "The provider failed",
        "without a code, do not add a placeholder; got: {error}"
    );
}

#[test]
fn response_failure_with_empty_code_uses_message_only() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_failed",
        "status": "failed",
        "error": {
            "code": "",
            "message": "The provider failed"
        },
        "output": []
    });

    let error = config
        .parse_response(&provider_response)
        .expect_err("a failed provider response should return an error");

    assert_eq!(
        error.as_str(),
        "The provider failed",
        "an empty code should not add a prefix; got: {error}"
    );
}

#[test]
fn response_rejects_in_progress_status_without_continuation_support() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_running",
        "status": "in_progress",
        "output": []
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "an in-progress response must not become an empty final answer"
    );
}

#[test]
fn response_rejects_queued_status_without_continuation_support() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_queued",
        "status": "queued",
        "output": []
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "a queued response must not become an empty final answer"
    );
}

#[test]
fn response_rejects_incomplete_status_without_finish_reason_support() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_incomplete",
        "status": "incomplete",
        "incomplete_details": {
            "reason": "max_output_tokens"
        },
        "output": []
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "an incomplete response must not be presented as a completed answer"
    );
}

#[test]
fn response_rejects_cancelled_status_without_cancelled_result_support() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_cancelled",
        "status": "cancelled",
        "output": []
    });

    let result = config.parse_response(&provider_response);

    assert!(
        result.is_err(),
        "a cancelled response must not become an empty successful answer"
    );
}

#[test]
fn response_maps_completed_status_as_successful_response() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid configuration should be accepted");

    let provider_response = serde_json::json!({
        "id": "resp_completed",
        "status": "completed",
        "output": [{
            "type": "message",
            "id": "msg_completed",
            "role": "assistant",
            "content": [{
                "type": "output_text",
                "text": "Hello",
                "annotations": []
            }]
        }]
    });

    let chat_response = config
        .parse_response(&provider_response)
        .expect("a completed response should map successfully");

    assert_eq!(chat_response.response_id(), Some("resp_completed"));
    assert_eq!(chat_response.messages()[0].text(), "Hello");
}