use std::{
    error::Error,
    sync::{Arc, atomic::Ordering},
};

use crate::{app_state::AppState, command::Command};

pub struct ExitCommand {}

impl Command for ExitCommand {
    fn execute(&self, state: Arc<AppState>, _: Vec<String>) -> Result<(), Box<dyn Error>> {
        state.running.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn name(&self) -> String {
        String::from("exit")
    }
}
