use super::FakeChatClient;
use crate::clients::ChatClient;
use crate::clients::ClientError;
/*
 use ? to propagate failure.
*/

#[test]
fn fake_client_returns_configured_reply() {
    let reply = "Hello from fake client".to_string();
    let client = FakeChatClient::new(reply.clone());
    let response = client.get_response(&[]).expect("fake client should return a response");
    assert_eq!(response.text(), reply);
    assert_eq!(response.messages().len(), 1);
    assert_eq!(response.messages()[0].role(), "assistant");
}

#[test]
fn fake_client_can_reply_twice() {
    let reply = "Hello again".to_string();
    let client = FakeChatClient::new(reply.clone());
    let response1 = client.get_response(&[]).expect("fake client should return a response");
    let response2 = client.get_response(&[]).expect("fake client should return a response");
    assert_eq!(response1.text(), reply);
    assert_eq!(response2.text(), reply);
}


fn ask_client<C: ChatClient>(client: &C) -> String {
    // Ask the client for a response, then return its text.
    let response = client.get_response(&[]).expect("fake client should return a response");
    response.text()
}

#[test]
fn can_ask_client_through_trait() { 
    let reply = "A predictable answer".to_string();
    let client = FakeChatClient::new(reply.clone());
    let text = ask_client(&client);
    assert_eq!(text, reply);
}

fn simulate_connection(avaiable: bool) -> Result<String, String> {
    if avaiable {
        Ok("Connection successful".to_string())
    } else {
        Err("Connection failed".to_string())
    }
}

#[test]
fn available_service_connects() {
    let result = simulate_connection(true);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Connection successful");
}

#[test]
fn unavailable_service_fails_to_connect() {
    let result = simulate_connection(false);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Connection failed");
}

fn connection_message(available: bool) -> String {
    match simulate_connection(available) {
        Ok(message) => {
            // Return a string starting with "Success: ".
            format!("Success: {}", message)
        }
        Err(error) => {
            // Return a string starting with "Error: ".
            format!("Error: {}", error)
        }
    }
}

#[test]
fn connection_success_message() {
    assert_eq!(connection_message(true), "Success: Connection successful");
}

#[test]
fn connection_error_message() {
assert_eq!(connection_message(false), "Error: Connection failed");
}


fn check_availability(available: bool) -> Result<(), ClientError> {
    if available {
        Ok(())
    } else {
        Err(ClientError::Unavailable)
    }
}

#[test]
fn check_availability_ok() {
    assert_eq!(check_availability(true), Ok(()));
}

#[test]
fn check_availability_err() {
    assert_eq!(check_availability(false), Err(ClientError::Unavailable));
}
