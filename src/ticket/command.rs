use crate::command::Command;

pub struct TicketCommand {}

impl Command for TicketCommand {
    fn execute(
        &self,
        state: std::sync::Arc<crate::app_state::AppState>,
        args: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn name(&self) -> String {
        String::from("ticket")
    }
}
