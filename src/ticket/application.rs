use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use rusqlite::Connection;

use crate::{
    app_state::AppState,
    ticket::{
        domain::{Ticket, TicketPriority},
        infrastructure::SqliteTicketRepository,
    },
};

pub trait TicketRepository: Send + Sync {
    fn save_ticket(&self, ticket: Ticket) -> Result<(), Box<dyn Error + '_>>;
    fn get_ticket(&self, id: &str) -> Result<Ticket, Box<dyn Error + '_>>;
    fn set_priority(&self, id: &str, priority: TicketPriority) -> Result<(), Box<dyn Error + '_>>;
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
}
