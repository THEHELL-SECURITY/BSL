use crate::commands::{Command, Value};
use std::io::Write;

pub struct SendCommand;

impl Command for SendCommand {
    fn execute(&self, args: &str, context: &mut crate::ExecutionContext) -> Result<(), String> {
        if context.stream.is_none() {
            return Err("No connection established, please use the 'connect' command.".to_string());
        }

        let stream = context.stream.as_mut().unwrap();
        let trimmed = args.trim();

        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            let cluster_name = &trimmed[1..trimmed.len()-1];
            if let Some(set) = context.sets.get(cluster_name) {
                if set.is_empty() {
                    return Err(format!("'{}' kümesi boş, gönderilecek veri yok.", cluster_name));
                }

                for val in set {
                    let line = match val {
                        Value::Str(s) => s.clone(),
                        Value::Int(i) => i.to_string(),
                        Value::Bool(b) => b.to_string(),
                    };
                    writeln!(stream, "{}", line)
                        .map_err(|e| format!("Veri gönderilemedi: {}", e))?;
                }
                stream.flush().map_err(|e| format!("Flush hatası: {}", e))?;
                Ok(())
            } else {
                Err(format!("Küme bulunamadı: '{}'", cluster_name))
            }
        } else {
            writeln!(stream, "{}", trimmed)
                .map_err(|e| format!("Veri gönderilemedi: {}", e))?;
            stream.flush().map_err(|e| format!("Flush hatası: {}", e))?;
            Ok(())
        }
    }
}
