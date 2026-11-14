use crate::types::{Message, Content};

pub struct AzureOpenAIConfig {
    endpoint: String,
    deployment: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AzureOpenAIConfigError {
    EmptyDeployment,
    EmptyEndpoint,
}

impl AzureOpenAIConfig {
    pub fn new(
        endpoint: String,
        deployment: String,
    ) -> Result<Self, AzureOpenAIConfigError> {
        if deployment.is_empty() {
            return Err(AzureOpenAIConfigError::EmptyDeployment);
        }
        if endpoint.is_empty(){
            return Err(AzureOpenAIConfigError::EmptyEndpoint);
        }

        Ok(Self {
            endpoint,
            deployment,
        })
        
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn deployment(&self) -> &str {
        &self.deployment
    }

    /*Next step: in azure_openai.rs, import Content alongside Message. Inside the loop, use match to recognize Content::Text and borrow its text field. Put that field—not the entire content item—into the JSON.

For other content variants, return a meaningful request-building error rather than skipping them. You’ll need your own error enum for that: serde_json::Error describes JSON errors, not unsupported content. Your error type needs Debug so the test’s .expect(...) can display it.

*/
    pub fn build_request_body(
        &self,
        messages: &[Message],
        instructions: Option<&str>,
    ) -> Result<serde_json::Value, AzureOpenAIRequestError> {

        // Step 1: Create an empty vector for JSON messages
        let mut input: Vec<serde_json::Value> = Vec::new();

        // Step 2: Loop through each Message
        for message in messages {

            // Create an empty vector for message content
            let mut content: Vec<serde_json::Value> = Vec::new();

            // Step 3: Loop through each text in the message
            for item in message.contents() {
                match item {
                    Content::Text { text, annotations} => {
                        if annotations.as_ref().is_some_and(|items| !items.is_empty()) {
                            return Err(AzureOpenAIRequestError::AnnotatedText);
                        }

                        let text_json = serde_json::json!({
                            "type": "input_text",
                            "text": text
                        });
                        content.push(text_json);
                    }
                    _ => return Err(AzureOpenAIRequestError::UnsupportedContent),
                }
            }

            // Step 4: Create JSON for the entire message
            let message_json = serde_json::json!({
                "role": message.role(),
                "content": content
            });

            // Add this message to the input vector
            input.push(message_json);
        }

        // Step 5: Build the final request body
        let request_body = serde_json::json!({
            "model": self.deployment,
            "input": input
        });

        // Step 6: Return success
        Ok(request_body)
    }

}
/*You can add a descriptive variant to your AzureOpenAIRequestError enum for that case. Then rerun the test. The existing test only checks unannotated text, so we’ll need a separate test for the rejection afterward.*/
#[derive(Debug)]
pub enum AzureOpenAIRequestError {
    Json(serde_json::Error),
    UnsupportedContent,
    AnnotatedText,
}

impl From<serde_json::Error> for AzureOpenAIRequestError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}