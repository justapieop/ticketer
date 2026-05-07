use crate::{command::Command, ticket::domain::Ticket};

pub struct ListTicketCommand {}

impl Command for ListTicketCommand {
    fn execute(
        &self,
        state: std::sync::Arc<crate::app_state::AppState>,
        _: Vec<String>,
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
            *output =
                crate::app_state::AppOutput::Text(String::from("Usage: create <title> <subject>"));
            return Ok(());
        }

        let title = &args[0];
        let subject = &args[1..].join(" ");
        let ticket = Ticket::new(
            title,
            subject,
            crate::ticket::domain::TicketPriority::Standard,
        );

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

pub struct DisplayTicketCommand {}

impl Command for DisplayTicketCommand {
    fn execute(
        &self,
        state: std::sync::Arc<crate::app_state::AppState>,
        args: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            let mut output = state.output.lock().unwrap();
            *output = crate::app_state::AppOutput::Text(String::from("Usage: get <id>"));
            return Ok(());
        }

        let mut output = state.output.lock().unwrap();
        if let Some(ticket) = state.ticket_service.get_ticket(&args[0]) {
            *output = crate::app_state::AppOutput::Ticket(ticket);
        } else {
            *output = crate::app_state::AppOutput::Text(format!("Ticket {} not found", args[0]));
        }

        Ok(())
    }

    fn name(&self) -> String {
        String::from("get")
    }
}

pub struct EditTicketCommand {}

impl Command for EditTicketCommand {
    fn execute(
        &self,
        state: std::sync::Arc<crate::app_state::AppState>,
        args: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if args.is_empty() {
            let mut output = state.output.lock().unwrap();
            *output = crate::app_state::AppOutput::Text(String::from("Usage: edit <id>"));
            return Ok(());
        }

        let id = &args[0];

        if state.ticket_service.get_ticket(id).is_none() {
            let mut output = state.output.lock().unwrap();
            *output = crate::app_state::AppOutput::Text(format!("Ticket {} not found", id));
            return Ok(());
        }

        let mut mode = state.mode.lock().unwrap();
        *mode = crate::app_state::AppMode::ChoosingEditField(id.clone());

        let mut output = state.output.lock().unwrap();
        *output = crate::app_state::AppOutput::Text(format!(
            "Editing ticket: {}. You are now in edit mode. Choose a field to edit.",
            id
        ));

        Ok(())
    }

    fn name(&self) -> String {
        String::from("edit")
    }
}
