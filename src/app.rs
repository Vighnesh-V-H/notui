use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::components::input::InputBox;

pub enum Mode {
    Setup,
    Running,
}

pub struct App {
    pub mode: Mode,
    pub input: InputBox,
    pub api_key: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::Setup,
            input: InputBox::new(),
            api_key: None,
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> bool {
        match self.mode {
            Mode::Setup => {
                match key.code {
                    KeyCode::Enter => {
                        let value = self.input.value().to_string();

                        if !value.is_empty() {
                            self.api_key = Some(value);
                            self.mode = Mode::Running;
                        }
                    }
                    KeyCode::Esc => return true, // exit app
                    _ => self.input.handle_key(key),
                }
            }

            Mode::Running => match key.code {
                KeyCode::Char('q') => return true,
                _ => {}
            },
        }

        false
    }

    pub fn render(&self, f: &mut Frame) {
        match self.mode {
            Mode::Setup => {
                let layout = Layout::vertical([
                    Constraint::Percentage(40),
                    Constraint::Length(3),
                    Constraint::Percentage(40),
                ]);

                let [_, input_area, _] = f.area().layout(&layout);

                self.input.render(f, input_area, true);
            }

            Mode::Running => {
                let text = format!(
                    "App running. API Key: {}",
                    self.api_key.as_deref().unwrap_or("None")
                );

                let paragraph = ratatui::widgets::Paragraph::new(text)
                    .block(ratatui::widgets::Block::bordered().title("Main App"));

                f.render_widget(paragraph, f.area());
            }
        }
    }
}
