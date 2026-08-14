use async_openai::{config::OpenAIConfig, Client};
use clap::Parser;
use serde_json::{json, Value};
use std::{env, fs, process};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);

    // Define tools
    let tools = json!([
        {
            "type": "function",
            "function": {
                "name": "Read",
                "description": "Read and return the contents of a file",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file to read"
                        }
                    },
                    "required": ["file_path"]
                }
            }
        }
    ]);

    // Build initial message history
    let mut messages = vec![json!({
        "role": "user",
        "content": args.prompt
    })];

    // Agent Loop
    loop {
        let response: Value = client
            .chat()
            .create_byot(json!({
                "model": "anthropic/claude-haiku-4.5",
                "messages": messages,
                "tools": tools
            }))
            .await?;

        let message = response["choices"][0]["message"].clone();

        // Append assistant response to message history
        messages.push(message.clone());

        // Check if the LLM called any tools
        if let Some(tool_calls) = message["tool_calls"].as_array() {
            if !tool_calls.is_empty() {
                // Process each tool call
                for tool_call in tool_calls {
                    let fn_name = tool_call["function"]["name"].as_str().unwrap_or("");
                    let call_id = tool_call["id"].as_str().unwrap_or("");

                    if fn_name == "Read" {
                        let args_str = tool_call["function"]["arguments"].as_str().unwrap_or("{}");
                        let args_val: Value = serde_json::from_str(args_str)?;

                        // Gracefully handle read failures and send errors back to the model
                        let tool_content = if let Some(file_path) = args_val["file_path"].as_str() {
                            match fs::read_to_string(file_path) {
                                Ok(content) => content,
                                Err(err) => format!("Error reading file: {}", err),
                            }
                        } else {
                            "Missing file_path argument".to_string()
                        };

                        // Append tool result message
                        messages.push(json!({
                            "role": "tool",
                            "tool_call_id": call_id,
                            "content": tool_content
                        }));
                    }
                }

                // Restart the loop to feed tool execution results back to the model
                continue;
            }
        }

        // If no tool calls were returned, output final response and terminate loop
        if let Some(content) = message["content"].as_str() {
            println!("{}", content);
        }

        break;
    }

    Ok(())
}