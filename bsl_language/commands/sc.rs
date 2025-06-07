use crate::commands::{Command, Value};
use std::collections::HashSet;
use regex::Regex;

pub struct SetCommand;

impl SetCommand {
    pub fn new() -> Self {
        SetCommand
    }
}

impl Command for SetCommand {
    fn execute(&self, args: &str, context: &mut crate::ExecutionContext) -> Result<(), String> {
        let re = Regex::new(r#"\(\s*(\w+)\s*\)\s*\(\s*(\w+)\s*\)\s*"([^"]*)""#)
            .map_err(|e| format!("Regex oluşturulamadı: {}", e))?;

        let caps = re.captures(args).ok_or("Argümanlar regex ile eşleşmedi")?;

        let cluster_part = caps.get(1).unwrap().as_str();
        let type_part = caps.get(2).unwrap().as_str();
        let value_part = caps.get(3).unwrap().as_str();

        

        let set = context.sets.entry(cluster_part.to_string()).or_insert_with(HashSet::new);

        let val = match type_part.to_lowercase().as_str() {
            "int" => value_part.parse::<i64>().map(Value::Int)
                .map_err(|_| "Geçersiz int değeri".to_string())?,
            "str" => Value::Str(value_part.to_string()),
            "bool" => match value_part.to_lowercase().as_str() {
                "true" => Value::Bool(true),
                "false" => Value::Bool(false),
                _ => return Err("Geçersiz bool değeri".into()),
            },
            _ => return Err(format!("Desteklenmeyen tip: '{}'", type_part)),
        };

        let added = set.insert(val);
        

        Ok(())
    }
}
