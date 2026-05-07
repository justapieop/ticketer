use std::{collections::HashMap, error::Error, sync::Arc};

use regex::Regex;

use crate::{
    app_state::AppState,
    misc::exit::ExitCommand,
    ticket::command::{
        CreateTicketCommand, DisplayTicketCommand, EditTicketCommand, ListTicketCommand,
    },
};

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
        let mut commands = HashMap::new();
        let mut register = |cmd: Box<dyn Command>| {
            commands.insert(cmd.name(), cmd);
        };

        register(Box::new(ExitCommand {}));
        register(Box::new(ListTicketCommand {}));
        register(Box::new(CreateTicketCommand {}));
        register(Box::new(DisplayTicketCommand {}));
        register(Box::new(EditTicketCommand {}));

        Self { commands, state }
    }

    pub fn parse_message(&self, msg: String) -> Vec<String> {
        let re = Regex::new(r"\s+").unwrap();
        re.replace_all(&msg, " ")
            .trim()
            .split(" ")
            .map(String::from)
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
