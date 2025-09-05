use rig::{client::ProviderClient, completion::Prompt, providers::deepseek::Client};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let client = Client::from_env();
    let agent = rig::client::CompletionClient::agent(&client, "deepseek-chat")
        .preamble("You are a helpful assistant.")
        .build();
    match agent.prompt("Tell me a joke").await {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e) => {
            // Attempt to extract and pretty print the JSON error
            let error_str = format!("{}", e);
            if let Some(json_start) = error_str.find('{') {
                if let Ok(json_value) = serde_json::from_str::<Value>(&error_str[json_start..]) {
                    let pretty_json = serde_json::to_string_pretty(&json_value)
                        .unwrap_or_else(|_| error_str.clone());
                    println!(
                        "Error: {}\n{}",
                        std::any::type_name_of_val(&e)
                            .split("::")
                            .last()
                            .unwrap_or("UnknownError"),
                        pretty_json
                    );
                } else {
                    println!("Error: {}", e);
                }
            } else {
                println!("Error: {}", e);
            }
        }
    }
}
