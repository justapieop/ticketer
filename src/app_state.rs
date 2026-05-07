use std::sync::{Arc, Mutex, atomic::AtomicBool};

use rusqlite::Connection;

use crate::ticket::{application::TicketService, domain::Ticket};

pub enum AppMode {
    Normal,
    ChoosingEditField(String),
    EditingTitle(String),
    EditingSubject(String),
    EditingPriority(String),
}

pub enum AppOutput {
    None,
    Text(String),
    TicketList(Vec<Ticket>),
    Ticket(Ticket),
}

pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
    pub running: AtomicBool,
    pub ticket_service: Arc<TicketService>,
    pub output: Mutex<AppOutput>,
    pub mode: Mutex<AppMode>,
}
