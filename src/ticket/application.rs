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
    fn close_ticket(&self, id: &str) -> Result<(), Box<dyn Error + '_>>;
    fn set_title(&self, id: &str, title: &str) -> Result<(), Box<dyn Error + '_>>;
    fn set_subject(&self, id: &str, subject: &str) -> Result<(), Box<dyn Error + '_>>;
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

    pub fn set_subject(&self, id: &str, subject: &str) -> Result<(), Box<dyn Error + '_>> {
        self.registry.set_subject(id, subject)
    }

    pub fn set_title(&self, id: &str, title: &str) -> Result<(), Box<dyn Error + '_>> {
        self.registry.set_title(id, title)
    }

    pub fn close_ticket(&self, id: &str) -> Result<(), Box<dyn Error + '_>> {
        self.registry.close_ticket(id)
    }

    pub fn set_priority(
        &self,
        id: &str,
        priority: TicketPriority,
    ) -> Result<(), Box<dyn Error + '_>> {
        self.registry.set_priority(id, priority)
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
