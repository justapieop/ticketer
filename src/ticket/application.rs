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

pub struct TicketService<T> {
    state: Arc<AppState>,
    registry: T,
}

impl TicketService<Box<dyn TicketRepository>> {
    pub fn new(state: Arc<AppState>) -> Self {
        let conn: Arc<Mutex<Connection>> = state.conn.clone();

        Self {
            state,
            registry: Box::new(SqliteTicketRepository::new(conn)),
        }
    }
}
