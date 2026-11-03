

use crate::clients::ChatClient;
use crate::types::{ChatResponse, Message};
use crate::clients::ClientError;
use crate::sessions::AgentSession;

pub(crate) struct Agent<C: ChatClient> {
    client: C,
  
}
impl<C: ChatClient> Agent<C> {
    // Your constructor and run method go here.
    pub(crate) fn new(client: C) -> Self {
        Self { client,
               
        }
    }

    pub(crate) fn run(&self, input: &str, session: &mut AgentSession) -> Result<ChatResponse, ClientError> {
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
        let response = self.client.get_response(&pending_messages)?; // use ? doesn;t panic like expect()
        pending_messages.extend(response.messages().iter().cloned());
        session.replace_history(pending_messages);
        Ok(response) // the run function returns the chat response after updating the history
    }


}



#[cfg(test)]
#[path = "agent_tests.rs"]
mod tests;