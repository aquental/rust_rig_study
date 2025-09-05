use rig::{client::ProviderClient, completion::Prompt, providers::deepseek::Client};

#[tokio::main]
async fn main() {
    let client = Client::from_env();
    let agent = rig::client::CompletionClient::agent(&client, "deepseek-chat")
        .preamble("You are a helpful assistant.")
        .build();
    let answer = agent.prompt("Tell me a joke").await.unwrap();

    println!("Answer: {answer}");
}
