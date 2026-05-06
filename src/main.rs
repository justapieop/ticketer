mod app;

use std::io;

use crate::app::App;

fn main() -> io::Result<()> {
    let mut app: App = App::new();

    ratatui::run(|term| app.run(term))
}
