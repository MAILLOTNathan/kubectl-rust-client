use serde_json::Value;
use std::fs;


pub fn parse_file(file_name: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string(file_name)?;
    let json_data: Value = serde_json::from_str(&file_content)?;

    return Ok(json_data);
}