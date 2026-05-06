use std::sync::Arc;

use crate::{app_state::AppState, command::CommandManager};

pub struct App {
    state: Arc<AppState>,
    running: bool,
    command_manager: CommandManager,
}

impl App {
    pub fn new(state: Arc<AppState>) -> Self {
        let cloned_state: Arc<AppState> = state.clone();

        Self {
            state,
            command_manager: CommandManager::new(cloned_state),
            running: true,
        }
    }

    pub fn run(&self) {
        while self.running {}
    }

    pub fn render() {}
}
