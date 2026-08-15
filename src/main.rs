mod config;
mod tools;

use config::Config;
use clap::Parser;
use serde_json::{json, Value};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::new();
    let tools = tools::get_tool_definitions();

    let mut messages = vec![json!({
        "role": "user",
        "content": args.prompt
    })];

    // Agent Loop
    loop {
        let response: Value = config
            .client
            .chat()
            .create_byot(json!({
                "model": "anthropic/claude-haiku-4.5",
                "messages": messages,
                "tools": tools
            }))
            .await?;

        let message = response["choices"][0]["message"].clone();
        messages.push(message.clone());

        if let Some(tool_calls) = message["tool_calls"].as_array() {
            if !tool_calls.is_empty() {
                for tool_call in tool_calls {
                    let fn_name = tool_call["function"]["name"].as_str().unwrap_or("");
                    let call_id = tool_call["id"].as_str().unwrap_or("");
                    let args_str = tool_call["function"]["arguments"].as_str().unwrap_or("{}");

                    let tool_content = tools::execute_tool(fn_name, args_str);

                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": call_id,
                        "content": tool_content
                    }));
                }
                continue;
            }
        }

        if let Some(content) = message["content"].as_str() {
            println!("{}", content);
        }

        break;
    }

    Ok(())
}