mod types;
mod fake_client;
mod clients;
mod agent;
use crate::agent::Agent;
use crate::fake_client::FakeChatClient;

fn main() {
    let client = FakeChatClient::new("Hello from your Rust agent!".to_string());
    let agent = Agent::new(client);
    let response = agent.run("Hello from user");
    println!("{}", response.expect("fake client should return a response").text());
}
