mod app;
mod components {
    pub mod input;
}
mod libs;

use std::io;

use app::App;
use crossterm::{
    event::{self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| app.render(f))?;

        match event::read()? {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    if app.handle_event(key) {
                        break;
                    }
                }
            }

            Event::Paste(data) => {
                app.handle_paste(data);
            }

            _ => {}
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableBracketedPaste
    )?;
    terminal.show_cursor()?;

    Ok(())
}
