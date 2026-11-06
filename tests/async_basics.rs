

async fn get_reply() -> &'static str {
    let reply: &'static str = "hi";
    reply
}

#[tokio::test]
async fn test_get_reply() {
    assert_eq!(get_reply().await, "hi");
}
