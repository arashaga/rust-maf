

#[derive(Debug)]
struct Message {
    // represnet a chat message
    role: String,
    contents: Vec<String>,
    author_name: Option<String>,
    message_id: Option<String>,


}

impl Message {
    fn new(role: String, contents: Vec<String>, author_name: Option<String>, message_id: Option<String>) -> Self {
        Self {
            role,
            contents,
            author_name,
            message_id,
        }
    }

    fn text(&self) -> String {
        //" ".join(content.text for content in self.contents if content.type == "text")
        self.contents.join(" ")
    }
}