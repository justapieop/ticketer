use std::{collections::HashMap, error::Error, sync::Arc};

use regex::Regex;

use crate::{app_state::AppState, misc::exit::ExitCommand};

pub struct CommandManager {
    commands: HashMap<String, Box<dyn Command>>,
    state: Arc<AppState>,
}

pub trait Command: Send + Sync {
    fn execute(&self, state: Arc<AppState>, args: Vec<String>) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> String;
}

impl CommandManager {
    pub fn new(state: Arc<AppState>) -> Self {
        let commands = [Box::new(ExitCommand {}) as Box<dyn Command>]
            .into_iter()
            .map(|cmd| (cmd.name(), cmd))
            .collect();

        Self { commands, state }
    }

    pub fn parse_message(&self, msg: String) -> Vec<String> {
        let re = Regex::new(r"\s+").unwrap();
        re.replace_all(&msg, " ")
            .trim()
            .split(" ")
            .map(|s| String::from(s))
            .collect()
    }

    pub fn exec(&self, command_name: &str, args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let parsed_command_name: String = String::from(command_name);

        let command: &Box<dyn Command> = match self.commands.get(&parsed_command_name) {
            Some(s) => s,
            None => return Err("Command not found".into()),
        };

        command.execute(self.state.clone(), args)
    }
}
