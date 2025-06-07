use std::io::Read;

pub struct ReceiveCommand;

impl ReceiveCommand {
    pub fn execute(&self, args: &str, context: &mut crate::ExecutionContext) -> Result<(), String> {
        if let Some(stream) = context.stream.as_mut() {
            let size: usize = args.trim().parse()
                .map_err(|_| "Invalid size".to_string())?;
            let mut buffer = vec![0u8; size];
            let n = stream.read(&mut buffer)
                .map_err(|e| format!("Read error: {}", e))?;
            buffer.truncate(n);
            Ok(())
        } else {
            Err("No connection established, please use the 'connect' command.".to_string())
        }
    }
}
