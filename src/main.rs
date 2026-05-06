mod app;
mod app_state;
mod command;
mod ticket;

use rusqlite::Connection;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{app::App, app_state::AppState};

fn main() -> Result<(), Box<dyn Error>> {
    let conn: Connection = Connection::open("data.db")?;

    let state: Arc<AppState> = Arc::new(AppState {
        conn: Arc::new(Mutex::new(conn)),
    });

    App::new(state).run();

    Ok(())
}
