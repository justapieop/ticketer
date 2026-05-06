use crate::{command::Command, ticket::domain::Ticket};

pub struct ListTicketCommand {}

impl Command for ListTicketCommand {
    fn execute(
        &self,
        state: std::sync::Arc<crate::app_state::AppState>,
        args: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tickets: Vec<Ticket> = state.ticket_service.list_ticket();

        let mut output = state.output.lock().unwrap();
        *output = crate::app_state::AppOutput::TicketList(tickets);

        Ok(())
    }

    fn name(&self) -> String {
        String::from("list")
    }
}

pub struct CreateTicketCommand {}

impl Command for CreateTicketCommand {
    fn execute(
        &self,
        state: std::sync::Arc<crate::app_state::AppState>,
        args: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if args.len() < 2 {
            let mut output = state.output.lock().unwrap();
            *output = crate::app_state::AppOutput::Text(String::from("Usage: create <title> <subject>"));
            return Ok(());
        }

        let title = &args[0];
        let subject = &args[1..].join(" ");
        let ticket = Ticket::new(title, subject, crate::ticket::domain::TicketPriority::Standard);
        
        let id_str = ticket.id.to_string();
        state.ticket_service.save_ticket(ticket)?;

        let mut output = state.output.lock().unwrap();
        *output = crate::app_state::AppOutput::Text(format!("Created ticket {}", id_str));

        Ok(())
    }

    fn name(&self) -> String {
        String::from("create")
    }
}
