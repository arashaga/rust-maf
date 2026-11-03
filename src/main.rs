mod types;
mod fake_client;
mod clients;
mod agent;
mod sessions;
use crate::agent::Agent;
use crate::fake_client::FakeChatClient;
use clients::ClientError;
use crate::sessions::AgentSession;

fn main() {
    let client = FakeChatClient::new("Hello from your Rust agent!".to_string());
    let agent = Agent::new(client);
    let mut session = AgentSession::new();
    let response = agent.run("Hello from user", &mut session);
    match response {
        Ok(resp) => println!("{}", resp.text()),
        Err(ClientError::Unavailable) => eprintln!("The client is unavailable. Please try again later."),
        Err(ClientError::EmptyInput) => {
            eprintln!("Please enter a message.")
        }
    
    }
    println!("Coversation History:");
    for message in session.history() {
        println!("{}: {}", message.role(), message.text());
    }

    let response2 = agent.run("How are you?", &mut session);
    match response2 {
        Ok(resp) => println!("{}", resp.text()),
        Err(ClientError::Unavailable) => eprintln!("The client is unavailable. Please try again later."),
        Err(ClientError::EmptyInput) => {
            eprintln!("Please enter a message.")
        }
    }
        println!("Coversation History:");
    for message in session.history() {
        println!("{}: {}", message.role(), message.text());
    }
}
