use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
}
