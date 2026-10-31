

#[derive(Debug)]
pub(crate) struct Message {
    // represnet a chat message
    role: String,
    contents: Vec<String>,
    author_name: Option<String>,
    message_id: Option<String>,


}

impl Message {
   pub(crate) fn new(role: String, contents: Vec<String>, author_name: Option<String>, message_id: Option<String>) -> Self {
        Self {
            role,
            contents,
            author_name,
            message_id,
        }
    }

    pub(crate) fn text(&self) -> String {
        //" ".join(content.text for content in self.contents if content.type == "text")
        self.contents.join(" ")
    }

    
    pub(crate) fn role(&self) -> &str {
        // Borrow the stored role.
        &self.role
    }

  

}

pub(crate)struct ChatResponse {
    messages: Vec<Message>,
}
 impl ChatResponse {
    pub(crate) fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }

    pub(crate) fn text(&self)-> String {
        // execute the text fn from message struct and collect it into a vectos and infer the type<_>
        self.messages.iter().map(|m| m.text()).collect::<Vec<_>>().join("\n").trim().to_string()
    }

    pub(crate) fn messages(&self) -> &[Message] {
        // Borrow the stored messages.
        &self.messages
    }
}


#[cfg(test)]
#[path = "types_tests.rs"]
mod tests; 
