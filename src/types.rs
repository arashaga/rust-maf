use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]//tells Serde to write a MAF-style type label, "text", into the serialized content.
pub enum Content {
    Text {
        text: String,
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
    Self::Text { text }
}

    pub fn text(&self) -> Option<&str> {
        match self {
            //text here is the vlue text of Text
            Self::Text { text } => Some(text.as_str()), //“If this is Content::Text, borrow its text field and return it inside Some.”
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

pub struct ChatResponse {
    messages: Vec<Message>,
}
 impl ChatResponse {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub fn text(&self)-> String {
        // execute the text fn from message struct and collect it into a vectos and infer the type<_>
        self.messages.iter().map(|m| m.text()).collect::<Vec<_>>().join("\n").trim().to_string()
    }

    pub fn messages(&self) -> &[Message] {
        // Borrow the stored messages.
        &self.messages
    }
}


#[cfg(test)]
#[path = "types_tests.rs"]
mod tests; 
