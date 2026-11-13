use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]//tells Serde to write a MAF-style type label, "text", into the serialized content.
pub enum Content {
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Vec<Annotation>>,
    },
    FunctionCall {
        call_id: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]//When converting this content into JSON, leave out the arguments field if its value is None.
        arguments: Option<serde_json::Value>
    },
    FunctionResult {
        call_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        items: Option<Vec<Content>>

    },
    TextReasoning {
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
}

impl Content {
    pub fn from_text(text: String) -> Self {
    Self::Text {
        text,
        annotations: None,
    }
}

    pub fn text(&self) -> Option<&str> {
        match self {
            //text here is the vlue text of Text
            //.. means ignore the annotations
            Self::Text { text, .. } => Some(text.as_str()), //“If this is Content::Text, borrow its text field and return it inside Some.”
            Self::FunctionCall { .. } => None,//“If this is a function call, return no ordinary text.”
            //.. means we don’t need to inspect its fields 
            Self::FunctionResult { .. } => None,
            Self::TextReasoning { ..} => None,
        }
        
    }

    pub fn from_function_result(
        call_id: String,
        result: String
    )-> Self{
        Self::from_function_result_items(
            call_id,
            vec![Self::from_text(result)],
        )
          
    }
    pub fn from_function_result_items(
        call_id: String,
        items: Vec<Content>,
    ) -> Self {
        let result = items
            .iter()
            .filter_map(Content::text)//filter_map() expects a function that returns an Option.
            .filter(|text| !text.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        Self::FunctionResult {
            call_id,
            result: Some(serde_json::Value::String(result)),
            items: Some(items),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    citation {
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    // represnet a chat message
    role: String,
    contents: Vec<Content>,
    author_name: Option<String>,
    message_id: Option<String>,


}


impl Message {
   pub fn new(role: String, contents: Vec<String>, author_name: Option<String>, message_id: Option<String>) -> Self {
       
        Self::from_contents(
        role,
        contents.into_iter().map(Content::from_text).collect(),
        author_name,
        message_id,
    )
        
    }
    pub fn from_contents(
        role: String,
        contents: Vec<Content>,
        author_name: Option<String>,
        message_id: Option<String>,
    ) -> Self {
        Self {
            role,
            contents,
            author_name,
            message_id,
        }
    }



    pub fn text(&self) -> String {
        let mut text_parts: Vec<&str> = Vec::new();
        for content in &self.contents {
            match content.text() {
                Some(text) => text_parts.push(text),
                None => {}
            }
         }
         text_parts.join(" ")
    }

    
    pub fn role(&self) -> &str {
        // Borrow the stored role.
        &self.role
    }

  pub fn contents(&self) -> &[Content] {
    // Borrow the stored contents.
    &self.contents
}
    pub fn author_name(&self) -> Option<&str> {
        self.author_name.as_deref() // this lets it borrow Option<string> as Option<&String> remember as_deref was used for &str conversion jus to get the &str
    }

    pub fn message_id(&self) -> Option<&str> {
        self.message_id.as_deref()
    }

}

#[derive(Deserialize)]
struct ChatResponseInput {
    #[serde(rename = "type")]
    kind: Option<String>,
    #[serde(default)]
    messages: Option<Vec<Message>>,
    response_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "chat_response", try_from = "ChatResponseInput")]
pub struct ChatResponse {
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_id: Option<String>,
}
 impl ChatResponse {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            messages,
            response_id: None,
        }
    }

    pub fn text(&self)-> String {
        // execute the text fn from message struct and collect it into a vectos and infer the type<_>
        self.messages().iter().map(|m| m.text()).collect::<Vec<_>>().join("\n").trim().to_string()
    }

    pub fn messages(&self) -> &[Message] {

        // Borrow the stored messages.
        &self.messages
    }

    pub fn response_id(&self) -> Option<&str> {
        self.response_id.as_deref()
    }

    pub fn set_response_id(&mut self, response_id: Option<String>) {
        self.response_id = response_id;
    }
}

impl TryFrom<ChatResponseInput> for ChatResponse {
    type Error = String; //The TryFrom trait requires you to specify which type represents a conversion failure.

    fn try_from(input: ChatResponseInput) -> Result<Self, Self::Error> {
        // shorthand for match input.kind { Some(kind) => ..., None => ... }
        if let Some(kind) = input.kind { //If input.kind contains a value, extract that value and call it kind.
            if kind != "chat_response" && !kind.is_empty() {
                return Err(format!("expected type \"chat_response\", got \"{kind}\""));
            } 
        }

        Ok(Self {
            //unwrap(), this does not panic on None
            messages: input.messages.unwrap_or_default(),//“If there is a list inside Some, give me that list. Otherwise, give me an empty list.”
            response_id: input.response_id,
        })
    }
}

#[cfg(test)]
#[path = "types_tests.rs"]
mod tests; 
