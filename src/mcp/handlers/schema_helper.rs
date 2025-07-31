//! Helper functions for creating MCP tool schemas

use serde_json::{json, Map, Value};

/// Convert a JSON value to a Map for use as tool schema
pub fn json_to_schema(value: Value) -> Map<String, Value> {
    match value {
        Value::Object(map) => map,
        _ => {
            let mut map = Map::new();
            map.insert("type".to_string(), json!("object"));
            map.insert("properties".to_string(), Value::Object(Map::new()));
            map
        },
    }
}
