use std::cell::Cell;

async fn get_reply() -> &'static str {
    let reply: &'static str = "hi";
    reply
}

async fn record_call(cell: &Cell<bool>) {
    cell.set(true);
}

#[tokio::test]
async fn test_get_reply() {
    assert_eq!(get_reply().await, "hi");
}

async fn get_checked_reply(
    available: bool,
) -> Result<&'static str, &'static str> {
    // Your implementation.
    if available {
        Ok("hi")
    } else {
        Err("client not available")
    }
}

#[tokio::test]
async fn test_get_checked_reply() {
    assert_eq!(get_checked_reply(true).await, Ok("hi"));
    assert_eq!(get_checked_reply(false).await, Err("client not available"));
}

#[tokio::test]
async fn test_record_call() {
    let cell = Cell::new(false);


    let pending = record_call(&cell);
    assert!(!cell.get()); // Still false: the body hasn't run.

    pending.await;
    assert!(cell.get());  // True: the body has run.
    }

    
async fn make_greeting(available: bool) -> Result<String, &'static str> {
   
    let reply = get_checked_reply(available).await?;
    Ok(format!("Agent says: {}", reply))
}

#[tokio::test]
async fn greeting_succeeds_when_client_is_available() {
    // Call make_greeting(true).await.
    // Assert the result equals Ok("Agent says: hi".to_string()).
    let result = make_greeting(true).await;
    assert_eq!(result, Ok("Agent says: hi".to_string()));
}

#[tokio::test]
async fn greeting_fails_when_client_is_unavailable() {
    // Call make_greeting(false).await
    // Assert the result equals Err("client not available")
    let result = make_greeting(false).await;
    assert_eq!(result, Err("client not available"));
}