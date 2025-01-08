use std::collections::HashMap;

use tera::{Error, Result, Value};

pub fn not_primary(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    Ok(value
        .clone()
        .as_array()
        .ok_or(Error::msg(Value::String("Error".to_string())))?
        .clone()
        .into_iter()
        .filter(|item| !item["is_primary_key"].as_bool().unwrap_or(false))
        .collect())
}

pub fn primary(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    Ok(value
        .clone()
        .as_array()
        .ok_or(Error::msg(Value::String("Error".to_string())))?
        .clone()
        .into_iter()
        .filter(|item| item["is_primary_key"].as_bool().unwrap_or(false))
        .collect())
}

pub fn foreign(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    Ok(value
        .clone()
        .as_array()
        .ok_or(Error::msg(Value::String("Error".to_string())))?
        .clone()
        .into_iter()
        .filter(|item| item["references"].is_object())
        .collect())
}

pub fn snake_to_pascal(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    let value = value.as_str().unwrap_or("").to_string();
    let mut chars = value.chars();
    let mut buffer = String::new();

    let mut idx = 0;
    while let Some(c) = &chars.next() {
        if c == &'_' {
            buffer.push(chars.next().unwrap().to_ascii_uppercase());
        } else if idx == 0 {
            buffer.push(c.clone().to_ascii_uppercase());
        } else {
            buffer.push(c.clone());
        }
        idx += 1;
    }

    Ok(Value::String(buffer))
}
