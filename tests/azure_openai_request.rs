use maf_rust::{AzureOpenAIConfig, Message};

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
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Hello"
                }]
            }]
        })
    );
}