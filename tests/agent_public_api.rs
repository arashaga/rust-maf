use maf_rust::{Agent, AgentSession, FakeChatClient, ChatClient, ChatResponse, ClientError, Message};
use std::cell::RefCell;
use tokio::sync::oneshot;

#[tokio::test]
async fn public_api_supports_a_conversation(){
    let client = FakeChatClient::new("Hello!".to_string());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let result = agent.run("Hello from user", &mut session).await;
    let response = result.expect("fake client should succeed");
    assert_eq!(response.text(), "Hello!");
    assert_eq!(session.history().len(), 2);
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[1].role(), "assistant");
    assert_eq!(session.history()[0].text(), "Hello from user");
    assert_eq!(session.history()[1].text(), "Hello!");
}

#[tokio::test]
async fn agent_records_async_calls() {
    let client = FakeChatClient::new("Hello!".to_string());
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let pending = agent.run("Hello from user", &mut session); //the right side produces a future. Nothing has run yet:
    drop(pending);
    assert!(session.history().is_empty());
    let result = agent.run("Hello from user", &mut session).await; // await runs the future until it completes
    result.expect("fake client should succeed");
    assert!(session.history().len() == 2);
}

/*
oneshot::Receiver<String> — receives one String.
Option<...> — holds Some(receiver) initially; later we can take it out, leaving None.
RefCell<...> — lets our client take the receiver out even though get_response receives &self.But taking the receiver changes the option from Some(receiver) to None
remember RefCell allsows you to change the value like Vec without call mut 
We want a test client whose reply is controlled by the test. The test can decide when to send the reply: not immediately behaves like async and await
The test keeps the sender. The client holds the receiver.
we could have create new in impl impl ControlledClient {
    fn new(channel_receiver: oneshot::Receiver<String>) -> Self {
        Self {
            receiver: RefCell::new(Some(channel_receiver)),
        }
    }
}
so taht we dont; have to instiate it like the way we do now
*/
struct ControlledClient {
    //A one-shot receiver can be awaited once. After that, it’s used up.So we need our client to represent two possible states:
    receiver: RefCell<Option<oneshot::Receiver<String>>>,
}

impl ChatClient for ControlledClient {
    //borrow_mut() temporarily lets us change what’s inside the RefCell.
    //take() removes the receiver from Some(receiver), leaving None.
    //ok_or(...) turns Some(receiver) into Ok(receiver), or None into the error.
    async fn get_response(&self, messages: &[Message], instructions: Option<&str>) -> Result<ChatResponse, ClientError> {
        let receiver = self.receiver.borrow_mut().take().ok_or(ClientError::Unavailable)?;

    let reply = match receiver.await {
    Ok(text) => text,
    Err(_channel_error) => return Err(ClientError::Unavailable),
    };
    Ok(ChatResponse::new(vec![Message::new(
    "assistant".to_string(),
    vec![reply],
    None,
    None,
)]))


}
}

#[tokio::test]
async fn send_reply_before_running_the_agent(){
    let (sender, receiver) = oneshot::channel::<String>();

    let client = ControlledClient {
        receiver: RefCell::new(Some(receiver)), // some either exists ot None
    };

    sender
        .send("hello from controlled client".to_string())
        .expect("we need a receiver");
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let response = agent.run("hi", &mut session ).await.expect("some error");

    assert_eq!(session.history()[1].text(),"hello from controlled client");
    assert_eq!(session.history().len() , 2);
    assert_eq!(response.text(),"hello from controlled client")

}

//Give the agent a chance to advance, observe that it is waiting, then regain control of the test to send its reply
#[tokio::test]
async fn agent_actually_waits(){

    let (sender, receiver) = oneshot::channel::<String>();

    let client = ControlledClient {
        receiver: RefCell::new(Some(receiver)),
    };
    
    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();

    let response = {
        let pending = agent.run("Hi", &mut session);
        tokio::pin!(pending); //prepares the future so we can use it,Pinning lets us borrow and poll that saved async future using:

        tokio::select! { //select! checks multiple pieces of async work.
            biased; //biased; makes it check the agent first.Without biased;, the immediately-ready branch could be checked first. 

            _ = &mut pending => {
                panic!("Agent finished before receiving a reply"); //If the agent already finishes, the test fails with panic!.
            }

            _ = std::future::ready(()) => {} //The _ means “ignore this branch’s value.” The {} means there’s nothing to do in that branch.


            }
        sender.send("Hello".to_string()).expect("something went wrong");
        pending.await.expect("agent should succeed")
    };
 // remember .expect extracts the response
    assert_eq!(response.text(), "Hello");
    assert_eq!(session.history().len(), 2);
    assert_eq!(session.history()[0].text(), "Hi");
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[1].text(), "Hello");
    assert_eq!(session.history()[1].role(), "assistant");
}

#[tokio::test]
async fn test_client_cannot_reply(){

    let (sender, channeL_receiver) = oneshot::channel::<String>();

    let client = ControlledClient{
      receiver : RefCell::new(Some(channeL_receiver)),
    };

    drop(sender);

    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();
    let result = agent.run("Hi", &mut session).await;

    assert!(matches!(result, Err(ClientError::Unavailable)));
    assert!(session.history().is_empty());

}

#[tokio::test]
async fn what_happens_on_a_second_call(){
        let (sender, channeL_receiver) = oneshot::channel::<String>();

    let client = ControlledClient{
      receiver : RefCell::new(Some(channeL_receiver)),
    };

    sender
    .send("Hello".to_string())
    .expect("receiver should exist");

    let agent = Agent::new(client, None);
    let mut session = AgentSession::new();

    let response = agent
    .run("Hi", &mut session)
    .await
    .expect("first call should succeed");

    let result = agent.run("Again", &mut session).await;
    assert_eq!(response.text(), "Hello");
    assert!(matches!(result, Err(ClientError::Unavailable)));
    assert_eq!(session.history().len(), 2);
    assert_eq!(session.history()[0].role(), "user");
    assert_eq!(session.history()[0].text(), "Hi");
    assert_eq!(session.history()[1].role(), "assistant");
    assert_eq!(session.history()[1].text(), "Hello");
}