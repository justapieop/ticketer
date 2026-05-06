use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub struct App {
    running: bool,
    selection: i8,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            selection: 1,
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        self.running = true;
        while self.running {
            self.handle_events()?;
            self.update();
            term.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn update(&mut self) {}

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn set_selection(&mut self, selection: i8) {
        self.selection = selection;
    }

    fn exit(&mut self) {
        self.running = false;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title: Line = Line::from(" Ticketer ".bold());

        let instruction: Line = Line::from(vec![
            " Select ".into(),
            "<Down> <Up>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block: Block = Block::bordered()
            .title(title.centered())
            .title_bottom(instruction.centered())
            .border_set(border::THICK);

        let desc: Text = Text::from(vec![" Ticket management system ".into()]);

        Paragraph::new(desc)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
