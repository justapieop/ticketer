use rusqlite::Connection;
use std::io::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use ticketer::ticket::application::TicketService;
use ticketer::{
    app::App,
    app_state::{AppMode, AppOutput, AppState},
};

fn main() -> Result<()> {
    let conn: Arc<Mutex<Connection>> = Arc::new(Mutex::new(
        Connection::open("data.db").expect("data.db must be created"),
    ));

    let state: Arc<AppState> = Arc::new(AppState {
        conn: conn.clone(),
        running: AtomicBool::from(true),
        ticket_service: Arc::new(TicketService::new(conn.clone())),
        output: Mutex::new(AppOutput::None),
        mode: Mutex::new(AppMode::Normal),
    });

    ratatui::run(|term| App::new(state).run(term))
}
