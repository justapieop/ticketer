use std::sync::{Arc, Mutex, atomic::AtomicBool};

use rusqlite::Connection;

pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
    pub running: AtomicBool,
}
