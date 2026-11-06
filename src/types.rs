use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    // represnet a chat message
    role: String,
    contents: Vec<String>,
    author_name: Option<String>,
    message_id: Option<String>,


}


impl Message {
   pub fn new(role: String, contents: Vec<String>, author_name: Option<String>, message_id: Option<String>) -> Self {
        Self {
            role,
            contents,
            author_name,
            message_id,
        }
    }

    pub fn text(&self) -> String {
        //" ".join(content.text for content in self.contents if content.type == "text")
        self.contents.join(" ")
    }

    
    pub fn role(&self) -> &str {
        // Borrow the stored role.
        &self.role
    }

  pub fn contents(&self) -> &[String] {
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
