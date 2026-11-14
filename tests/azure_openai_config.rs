use maf_rust::{AzureOpenAIConfig, AzureOpenAIConfigError};

#[test]
fn config_rejects_empty_deployment() {
    let result = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        String::new(),
    );

    assert!(
        matches!(
            result,
            Err(AzureOpenAIConfigError::EmptyDeployment)
        ),
        "an empty deployment name must be rejected"
    );
}

#[test]
fn config_preserves_valid_settings() {
    let config = AzureOpenAIConfig::new(
        "https://example.openai.azure.com".to_string(),
        "learning-deployment".to_string(),
    )
    .expect("valid settings should be accepted");

    assert_eq!(
        config.endpoint(),
        "https://example.openai.azure.com"
    );
    assert_eq!(config.deployment(), "learning-deployment");
}

#[test]
fn config_rejects_empty_endpoint() {
    let result = AzureOpenAIConfig::new(
        String::new(),
        "learning-deployment".to_string(),
    );

    assert!(
        matches!(
            result,
            Err(AzureOpenAIConfigError::EmptyEndpoint)
        ),
        "an empty endpoint must be rejected"
    );
}