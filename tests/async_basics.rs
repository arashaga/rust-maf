

async fn get_reply() -> &'static str {
    let reply: &'static str = "hi";
    reply
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