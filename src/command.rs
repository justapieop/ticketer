use std::{collections::HashMap, error::Error, sync::Arc};

use crate::app_state::AppState;

pub struct CommandManager {
    commands: HashMap<String, Box<dyn Command>>,
    state: Arc<AppState>,
}

pub trait Command: Send + Sync {
    fn execute(&self, state: Arc<AppState>) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> String;
}

impl CommandManager {
    pub fn new(state: Arc<AppState>) -> Self {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        Self { commands, state }
    }

    pub fn exec(&self, command_name: &str) -> Result<(), Box<dyn Error>> {
        let parsed_command_name: String = String::from(command_name);

        let command: &Box<dyn Command> = match self.commands.get(&parsed_command_name) {
            Some(s) => s,
            None => return Err("Command not found".into()),
        };

        command.to_owned().execute(self.state.clone())
    }
}
