use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use rusqlite::Connection;

use crate::ticket::{
    domain::{Ticket, TicketPriority},
    infrastructure::SqliteTicketRepository,
};

pub trait TicketRepository: Send + Sync {
    fn save_ticket(&self, ticket: Ticket) -> Result<(), Box<dyn Error + '_>>;
    fn get_ticket(&self, id: &str) -> Option<Ticket>;
    fn set_priority(&self, id: &str, priority: TicketPriority) -> Result<(), Box<dyn Error + '_>>;
    fn list_ticket(&self) -> Vec<Ticket>;
}

pub struct TicketService {
    registry: Box<dyn TicketRepository>,
}

impl TicketService {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self {
            registry: Box::new(SqliteTicketRepository::new(conn)),
        }
    }

    pub fn get_ticket(&self, id: &str) -> Option<Ticket> {
        self.registry.get_ticket(id)
    }

    pub fn save_ticket(&self, ticket: Ticket) -> Result<(), Box<dyn Error>> {
        self.registry
            .save_ticket(ticket)
            .map_err(|e| Box::from(e.to_string()))
    }

    pub fn list_ticket(&self) -> Vec<Ticket> {
        self.registry.list_ticket()
    }
}
