use serde_json::{Value, json};

use crate::state::AppState;

pub fn handle_tools_list(state: &AppState) -> Value {
    json!({
        "tools": [
            {
                "name": "mega/generate",
                "description": "Generate Mega.nz accounts using temporary email addresses",
                "inputSchema": {
                    "type": "object",
                    "properties": {
                        "count": {
                            "type": "integer",
                            "minimum": 1,
                            "maximum": state.max_count,
                            "default": 1
                        },
                        "password": {
                            "type": "string",
                            "description": "Password for the generated account",
                            "default": state.default_password.as_str()
                        }
                    }
                }
            }
        ]
    })
}
