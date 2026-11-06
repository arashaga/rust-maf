

use crate::clients::ChatClient;
use crate::types::{ChatResponse, Message};
use crate::clients::ClientError;
use crate::sessions::AgentSession;

pub struct Agent<C: ChatClient> {
    client: C,
    instructions: Option<String>,
}
impl<C: ChatClient> Agent<C> {
    // Your constructor and run method go here.
    pub fn new(client: C, instructions: Option<String>) -> Self {
        Self { client,
               instructions,
        }
    }

    pub async fn run(&self, input: &str, session: &mut AgentSession) -> Result<ChatResponse, ClientError> {
        if input.trim().is_empty() {
            return Err(ClientError::EmptyInput);
        }
        let message = Message::new(
            "user".to_string(),
            vec![input.to_string()],
            None,
            None,
        );
        
        // we do this so if the conversation fails, the original history remains unchanged until we successfully get a response

        let mut pending_messages = session.history().to_vec();
        pending_messages.push(message);
        //sending the whole history / context to the get_response method of the client
        let response = self.client.get_response(&pending_messages, self.instructions.as_deref()).await?; // use ? doesn;t panic like expect() also deref lets the client use you instruction without copying it
        pending_messages.extend(response.messages().iter().cloned());
        session.replace_history(pending_messages);
        Ok(response) // the run function returns the not the chat response but envelope(OK) containing the response 
    }


}



#[cfg(test)]
#[path = "agent_tests.rs"]
mod tests;