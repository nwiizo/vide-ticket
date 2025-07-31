//! Ticket management MCP tool handlers

use rmcp::model::Tool;
use serde_json::{json, Map, Value};
use std::borrow::Cow;
use std::sync::Arc;

/// Register all ticket management tools
pub fn register_tools() -> Vec<Tool> {
    let mut schema = Map::new();
    schema.insert("type".to_string(), json!("object"));

    let mut properties = Map::new();
    let mut status_prop = Map::new();
    status_prop.insert("type".to_string(), json!("string"));
    status_prop.insert(
        "enum".to_string(),
        json!(["todo", "doing", "done", "blocked", "review"]),
    );
    status_prop.insert("description".to_string(), json!("Filter by status"));
    properties.insert("status".to_string(), Value::Object(status_prop));

    schema.insert("properties".to_string(), Value::Object(properties));

    vec![Tool {
        name: Cow::Borrowed("vibe-ticket.list"),
        description: Some(Cow::Borrowed("List tickets")),
        input_schema: Arc::new(schema),
        annotations: None,
    }]
}
