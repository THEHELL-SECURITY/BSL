pub struct CloseCommand;

impl CloseCommand {
    pub fn execute(&self, _args: &str, context: &mut crate::ExecutionContext) -> Result<(), String> {
        context.stream = None;
        Ok(())
    }
}
