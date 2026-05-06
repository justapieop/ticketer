use std::sync::{Arc, atomic::Ordering};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{
        Alignment,
        Constraint::{Length, Min},
        Layout,
    },
    style::Stylize,
    widgets::Paragraph,
};
use std::io::Result;

use crate::{app_state::AppState, command::CommandManager};

pub struct App {
    state: Arc<AppState>,
    command_manager: CommandManager,
    input: String,
    last_error: Option<String>,
}

impl App {
    pub fn new(state: Arc<AppState>) -> Self {
        let cloned_state: Arc<AppState> = state.clone();

        Self {
            state,
            command_manager: CommandManager::new(cloned_state),
            input: String::new(),
            last_error: None,
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> Result<()> {
        while self.state.running.load(Ordering::Relaxed) {
            self.handle_events()?;
            term.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char(c) => {
                self.input.push(c);
            }
            KeyCode::Backspace => {
                self.input.pop();
            }
            KeyCode::Enter => {
                if !self.input.trim().is_empty() {
                    let args = self.command_manager.parse_message(self.input.clone());
                    if let Some((cmd_name, exec_args)) = args.split_first() {
                        self.last_error = self
                            .command_manager
                            .exec(cmd_name, exec_args.to_vec())
                            .err()
                            .map(|e| e.to_string());
                    }
                }
                self.input.clear();
            }
            _ => {}
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [title_area, main_area, status_area] = vertical.areas(frame.area());

        let title = Paragraph::new(" Ticketer ")
            .bold()
            .alignment(Alignment::Center)
            .reversed();
        frame.render_widget(title, title_area);

        if let Some(error_msg) = &self.last_error {
            let error_widget = Paragraph::new(format!("Error: {}", error_msg))
                .red()
                .alignment(Alignment::Center);
            frame.render_widget(error_widget, main_area);
        }

        let input_text = format!("> {}", self.input);
        let input_widget = Paragraph::new(input_text);
        frame.render_widget(input_widget, status_area);

        frame.set_cursor_position((status_area.x + 2 + self.input.len() as u16, status_area.y));
    }
}
