mod app;
mod app_state;
mod command;
mod misc;
mod ticket;

use rusqlite::Connection;
use std::io::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use crate::{app::App, app_state::AppState};

fn main() -> Result<()> {
    let conn: Connection = Connection::open("data.db").expect("data.db must be created");

    let state: Arc<AppState> = Arc::new(AppState {
        conn: Arc::new(Mutex::new(conn)),
        running: AtomicBool::from(true),
    });

    ratatui::run(|term| App::new(state).run(term))
}
