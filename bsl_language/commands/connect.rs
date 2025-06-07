use std::net::TcpStream;

pub struct ConnectCommand;

impl ConnectCommand {
    pub fn execute(&self, args: &str, context: &mut crate::ExecutionContext) -> Result<(), String> {
        let stream = TcpStream::connect(args.trim())
            .map_err(|e| format!("Connection error: {}", e))?;
        context.stream = Some(stream);
        Ok(())
    }
}
