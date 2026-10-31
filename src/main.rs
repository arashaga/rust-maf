mod types;
mod fake_client;
mod clients;
mod agent;
use crate::agent::Agent;
use crate::fake_client::FakeChatClient;
use clients::ClientError;

fn main() {
    let client = FakeChatClient::new("Hello from your Rust agent!".to_string());
    let agent = Agent::new(client);
    let response = agent.run("Hello from user");
    match response {
        Ok(resp) => println!("{}", resp.text()),
        Err(ClientError::Unavailable) => eprintln!("The client is unavailable. Please try again later."),
        Err(ClientError::EmptyInput) => {
            eprintln!("Please enter a message.")
        }
    }
    
}
