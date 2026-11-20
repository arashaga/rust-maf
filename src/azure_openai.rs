use std::{env::var_os, str::SplitTerminator};

use std::{future::Future, pin::Pin};

// define aliases
// FN() is  atrain for something that can be called like a function here Something that can be called with zero arguments and returns a TokenFuture 
//dyn Some callable object that returns a TokenFuture. I don't need to know its exact closure type.
// Box handles the size versusu type wher eit's handled by dyn
//Pin prevents moving the future out of that pinned allocation through safe operations when the future requires a stable location.
// Sync and Send These are Rust's thread-safety marker traits.Send: safe to transfer between threads,Sync: safe to share references between threads
/*
Result<String, String>: produces either a token or an error.
Future<Output = ...>: represents an asynchronous computation producing that result.
dyn Future: we don't care which concrete future implementation it is.
+ Send: it can safely move between threads.
Box: store the future behind an owned heap pointer.
Pin: preserve the future's pinning guarantees.
*/
// Future is a train and Output is an associated type declared by that trait 
type TokenFuture = Pin<Box<dyn Future<Output = Result<String, String>> + Send>>;
type TokenProvider = Box<dyn Fn() -> TokenFuture + Send + Sync>;

use crate::{ChatResponse, types::{Content, Message}};
use serde_json::{json, Value};

pub struct AzureOpenAIConfig {
    endpoint: String,
    deployment: String,
    token_provider: Option<TokenProvider>,
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
            token_provider: None,
        })
        
    }

    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub fn deployment(&self) -> &str {
        &self.deployment
    }

 /**/
    pub fn build_request_body(
        &self,
        messages: &[Message],
        instructions: Option<&str>,
    ) -> Result<serde_json::Value, AzureOpenAIRequestError> {

        // Step 1: Create an empty vector for JSON messages
        let mut input: Vec<serde_json::Value> = Vec::new();

        if messages.is_empty(){
            return Err(AzureOpenAIRequestError::EmptyInput);
        }
        // Step 2: Loop through each Message
        for message in messages {
            let role = message.role();

            if role != "user" && role != "assistant" {
                return Err(AzureOpenAIRequestError::UnsupportedRole);
            }

            if message.contents().is_empty(){
                continue;
            }


            // Create an empty vector for message content
            let mut content: Vec<serde_json::Value> = Vec::new();

            // Step 3: Loop through each text in the message
            for item in message.contents() {
                match item {
                    Content::Text { text, annotations} => {
                        if annotations.as_ref().is_some_and(|items| !items.is_empty()) {
                            return Err(AzureOpenAIRequestError::AnnotatedText);
                        }

                        let text_json = if role == "user" {serde_json::json!({
                            "type": "input_text",
                            "text": text
                        })
                    } else {
                        serde_json::json!({
                            "type": "output_text",
                            "text": text,
                            "annotations": [], 
                        })
                    };
                        content.push(text_json);
                    }
                    _ => return Err(AzureOpenAIRequestError::UnsupportedContent),
                }
            }

            // Step 4: Create JSON for the entire message
            let message_json = serde_json::json!({
                "type": "message",
                "role": message.role(),
                "content": content
            });

            // Add this message to the input vector
            input.push(message_json);
        }

        if input.is_empty(){
                return Err(AzureOpenAIRequestError::EmptyInput);
            }
        // Step 5: Build the final request body
        let mut request_body = serde_json::json!({
            "model": self.deployment,
            "input": input
        });

        //If instructions contains a string, extract it into a variable called text. Otherwise, skip the block.
        /*
        match instructions {
            Some(text) => {
                request_body["instructions"] = serde_json::json!(text);
            },
            None => {
                // Do nothing
            },
    }
         */
        if let Some(instruction) = instructions{
            request_body["instructions"] = serde_json::json!(instruction)
        }

        // Step 6: Return success
        Ok(request_body)
    }

    pub fn build_http_request(&self,
         messages: &[Message],
        instructions: Option<&str>,
        api_key: Option<&str>,
        bearer_token: Option<&str>
    ) -> Result<reqwest::Request, String>{

        // converts the error defined in the class to a string
        let body =self.build_request_body(messages, instructions)
        .map_err(|error| format!("failed to build Azure request body: {error:?}"))?;
        let url = format!(
            "{}/openai/v1/responses",
            self.endpoint().trim_end_matches('/')
        );

        let mut request = reqwest::Client::new()
            .post(url)
            .json(&body);
        
        if let Some(token) = bearer_token{
            request = request.bearer_auth(token);
        }else if let Some(key) = api_key {
            request = request.header("api-key", key);
        } else {
            return Err("request construction should fail without an API key or bearer token".to_string());
        }

        request.build()
        .map_err(|error| format!("failed to build Azure HTTP request: {error:?}"))
        

    }

    pub fn parse_response(&self,response : &Value) -> Result<ChatResponse, String>{

        let mut messages: Vec<Message> = Vec::new();
        // let mut contents: Vec<Content> = Vec::new();

        let status = response.get("status").and_then(|value| value.as_str());

        if status == Some("failed") {
            let code = response.get("error")
                    .and_then(|value| value.get("code"))
                    .and_then(|value| value.as_str());

            let error = response.get("error")
                    .and_then(|value| value.get("message"))
                    .and_then(|value| value.as_str())
                    .unwrap_or("failed response from the provider"); // does not panic and provides a default value if None
           
            if let Some(code_value) = code {
                    if !code_value.is_empty() {
                        return Err(format!("{}: {}", code_value, error));
                    }
       
            }
            return Err(error.to_string());
            
        }else if status == Some("in_progress"){
            return Err("an in-progress response must not become an empty final answer".to_string());
        }else if status == Some("queued"){
            return Err("a queued response must not become an empty final answer".to_string());
        }else if status == Some("incomplete"){
            return Err("an incomplete response must not be presented as a completed answer".to_string());
        }else if status == Some("cancelled"){
            return Err("a cancelled response must not become an empty successful answer".to_string());
        }


        let response_id = response.get("id").and_then(|value| value.as_str()).ok_or("Missing response id")?.to_string();
        let output_value = response
            .get("output")
            .ok_or("Missing output".to_string())?;

        let response_array = output_value
            .as_array()
            .ok_or("Output must be an array".to_string())?;


        for item in response_array{
            let item_type = item
                .get("type")
                .and_then(|value| value.as_str())
                .ok_or("Missing or invalid output type")?;
            if item_type != "message"{
                return Err("Unsuported output type".to_string());
            }
            // and then gets the Option value out
            let message_id = item.get("id").and_then(|value| value.as_str()).ok_or("missing or invalid message id")?.to_string();
            let message_role = item.get("role").and_then(|value| value.as_str()).ok_or("missing or invalid message role")?.to_string();
            if message_role != "assistant"{
                return Err("response provider role must be assistant!".to_string());
            }
            let content_array = item.get("content").and_then(|value| value.as_array()).ok_or("content missing")?;



            let mut contents: Vec<String> = Vec::new();

            for content in content_array{
                if content["type"] != "output_text"{
                    return Err("Unsuported output type".to_string());
                }

                let text = content.get("text").and_then(|value|value.as_str()).ok_or("content missing")?.to_string();
                let annotations = content.get("annotations")
                    .and_then(|value| value
                    .as_array())
                    .ok_or("Annotation doesn;t exist")?;

                if !annotations.is_empty(){
                    return Err("Unsupported annotations!".to_string());
                }
                contents.push(text);

            }
            let message = Message::new(message_role, contents, None, Some(message_id));
            messages.push(message);




        }

        if messages.is_empty(){
                messages.push(Message::new("assistant".to_string(), vec![], None, None));
            }
        let mut chat_response = ChatResponse::new(messages);
        chat_response.set_response_id(Some(response_id));
        Ok(chat_response)

    }

    // F is a placeholder for concrete type when the fucntion gets called later Rust understands that F is the compiler-generated type of that function
    pub fn with_token_provider<F, Fut>(mut self, provider: F) -> Self
    where // provider must implmnted all of these
    // the implmentor shoul dbe able to supply it's own token function hence the type of F in <F, Fut> is Fn() -> that returns a
    F: Fn() -> Fut + Send + Sync + 'static,//'static means Does it avoid borrowing data that might expire before it's finished being used?
    Fut: Future<Output = Result<String, String>> + Send + 'static,
    {
        let wrapped_provider = move || -> TokenFuture {
            Box::pin(provider())
        };

    self.token_provider = Some(Box::new(wrapped_provider));

    self
    }

    pub async fn build_http_request_with_token_provider(&self, //lets the method read the particular config it was called on without taking ownership of it.
    messages: &[Message],
    instructions: Option<&str>,
    api_key: Option<&str>) -> Result<reqwest::Request, String>{ // successful results is reqwest returning request headers
        
        match &self.token_provider {
            Some(provider) => {
                let bearer_token = provider().await?;
                self.build_http_request(messages, instructions, api_key, Some(bearer_token.as_str()))
            }
            None =>{
             
               self.build_http_request(messages, instructions, api_key, None)
            }



        }
    }
}
/*You can add a descriptive variant to your AzureOpenAIRequestError enum for that case. Then rerun the test. The existing test only checks unannotated text, so we’ll need a separate test for the rejection afterward.*/
#[derive(Debug)]
pub enum AzureOpenAIRequestError {
    Json(serde_json::Error),
    UnsupportedContent,
    AnnotatedText,
    UnsupportedRole,
    EmptyInput,
}

impl From<serde_json::Error> for AzureOpenAIRequestError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

