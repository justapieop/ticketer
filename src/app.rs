use std::sync::Arc;

use ratatui::DefaultTerminal;
use std::io::Result;

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

    pub fn run(&mut self, term: &DefaultTerminal) -> Result<()> {
        while self.running {
            self.handle_events()?;
            self.render();
        }

        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<()> {
        unimplemented!()
    }

    pub fn render(&self) {}
}
