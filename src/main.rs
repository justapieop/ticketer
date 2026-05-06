mod app;
mod app_state;
mod command;
mod misc;
mod ticket;

use rusqlite::Connection;
use std::io::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use crate::ticket::application::TicketService;
use crate::{app::App, app_state::AppState};

fn main() -> Result<()> {
    let conn: Arc<Mutex<Connection>> = Arc::new(Mutex::new(
        Connection::open("data.db").expect("data.db must be created"),
    ));

    let state: Arc<AppState> = Arc::new(AppState {
        conn: conn.clone(),
        running: AtomicBool::from(true),
        ticket_service: Arc::new(TicketService::new(conn.clone())),
    });

    ratatui::run(|term| App::new(state).run(term))
}
