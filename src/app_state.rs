use std::sync::{Arc, Mutex, atomic::AtomicBool};

use rusqlite::Connection;

use crate::ticket::application::TicketService;

pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
    pub running: AtomicBool,
    pub ticket_service: Arc<TicketService>,
}
