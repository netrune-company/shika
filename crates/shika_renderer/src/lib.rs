mod database;
mod error;

use database::Database;
use shika_workspace::Workspace;
use tera::{Context, Tera};

pub use error::Error;

pub struct Renderer {
    engine: Tera,
}

impl Renderer {
    pub fn new(workspace: &Workspace) -> Result<Self, Error> {
        let path = workspace
            .path
            .join(".shika")
            .join("templates")
            .join("**")
            .join("*")
            .display()
            .to_string();

        let mut engine = Tera::new(path.as_ref())?;

        engine.register_filter("primary_keys", filters::primary_keys);
        engine.register_filter("foreign_keys", filters::foreign_keys);
        engine.register_filter("no_keys", filters::no_keys);

        engine.register_filter("upper", filters::upper);
        engine.register_filter("pascal", filters::pascal);
        engine.register_filter("snake", filters::snake);
        engine.register_filter("camel", filters::camel);

        Ok(Self { engine })
    }

    pub fn render(&self, template: &str, data: &Database) -> Result<String, Error> {
        let context = Context::from_serialize(data)?;
        Ok(self.engine.render(template, &context)?)
    }
}

mod filters {
    use std::collections::HashMap;

    use convert_case::{Case, Casing};
    use tera::{Error, Result, Value};

    pub fn primary_keys(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        Ok(value
            .clone()
            .as_array()
            .ok_or(Error::msg(Value::String("Value is not array".to_string())))?
            .clone()
            .into_iter()
            .filter(|item| item["is_primary_key"] == true)
            .collect())
    }

    pub fn foreign_keys(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        Ok(value
            .clone()
            .as_array()
            .ok_or(Error::msg(Value::String("Value is not array".to_string())))?
            .clone()
            .into_iter()
            .filter(|item| {
                let Some(references) = item.get("references") else {
                    return false;
                };

                !references.is_null()
            })
            .collect())
    }

    pub fn no_keys(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        Ok(value
            .as_array()
            .ok_or(Error::msg(Value::String("Value is not array".to_string())))?
            .clone()
            .into_iter()
            .filter(|item| {
                if item["is_primary_key"] == true {
                    return false;
                }

                if let Some(references) = item.get("references") {
                    return references.is_null();
                };

                true
            })
            .collect())
    }

    pub fn upper(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or(Error::msg(Value::String(
                "Value is not a string".to_string(),
            )))?
            .to_case(Case::Upper);

        Ok(Value::String(text))
    }

    pub fn pascal(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or(Error::msg(Value::String(
                "Value is not a string".to_string(),
            )))?
            .to_case(Case::Pascal);

        Ok(Value::String(text))
    }

    pub fn snake(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or(Error::msg(Value::String(
                "Value is not a string".to_string(),
            )))?
            .to_case(Case::Snake);

        Ok(Value::String(text))
    }

    pub fn camel(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let text = value
            .as_str()
            .ok_or(Error::msg(Value::String(
                "Value is not a string".to_string(),
            )))?
            .to_case(Case::Camel);

        Ok(Value::String(text))
    }
}
