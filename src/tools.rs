use serde_json::{json, Value};
use std::fs;

/// Returns the JSON schema for all available agent tools.
pub fn get_tool_definitions() -> Value {
    json!([
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
        },
        {
            "type": "function",
            "function": {
                "name": "Write",
                "description": "Write content to a file at the specified path",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file to write to"
                        },
                        "content": {
                            "type": "string",
                            "description": "The text content to write to the file"
                        }
                    },
                    "required": ["file_path", "content"]
                }
            }
        }
    ])
}

/// Executes a tool call based on its function name and JSON arguments.
pub fn execute_tool(fn_name: &str, args_json: &str) -> String {
    let args_val: Value = match serde_json::from_str(args_json) {
        Ok(v) => v,
        Err(err) => return format!("Failed to parse arguments JSON: {}", err),
    };

    match fn_name {
        "Read" => {
            if let Some(file_path) = args_val["file_path"].as_str() {
                match fs::read_to_string(file_path) {
                    Ok(content) => content,
                    Err(err) => format!("Error reading file: {}", err),
                }
            } else {
                "Missing 'file_path' argument".to_string()
            }
        }
        "Write" => {
            let file_path = args_val["file_path"].as_str();
            let content = args_val["content"].as_str();

            match (file_path, content) {
                (Some(path), Some(text)) => match fs::write(path, text) {
                    Ok(_) => format!("Successfully wrote to {}", path),
                    Err(err) => format!("Error writing to file: {}", err),
                },
                _ => "Missing required arguments ('file_path' or 'content')".to_string(),
            }
        }
        _ => format!("Unknown tool function: {}", fn_name),
    }
}