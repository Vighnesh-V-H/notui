use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::{components::input::InputBox, libs::credential_manager::CredentialManager};

pub enum Mode {
    Setup,
    Running,
}

pub struct App {
    pub mode: Mode,
    pub input: InputBox,
    pub api_key: Option<String>,
    pub credential_manager: CredentialManager,
    pub error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let credential_manager = CredentialManager::new("notui_app", "notion_api_key");

        let api_key = match credential_manager.get_api_key() {
            Ok(key) => Some(key),
            Err(e) => {
                println!("Failed to load API key: {:?}", e);
                None
            }
        };

        let mode = if api_key.is_some() {
            Mode::Running
        } else {
            Mode::Setup
        };
        Self {
            mode: mode,
            input: InputBox::new(),
            api_key,
            credential_manager,
            error: None,
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> bool {
        match self.mode {
            Mode::Setup => match key.code {
                KeyCode::Enter => {
                    let value = self.input.value().to_string();

                    if !value.is_empty() {
                        match self.credential_manager.save_api_key(&value) {
                            Ok(_) => {
                                self.api_key = Some(value);
                                self.mode = Mode::Running;
                                self.error = None;
                            }
                            Err(e) => {
                                self.error = Some(e.to_string());
                            }
                        }
                    }
                }

                KeyCode::Esc => return true,

                _ => self.input.handle_key(key),
            },

            Mode::Running => match key.code {
                KeyCode::Char('q') => return true,
                _ => {}
            },
        }

        false
    }

    pub fn handle_paste(&mut self, data: String) {
        self.input.handle_paste(data);
    }

    pub fn render(&self, f: &mut Frame) {
        match self.mode {
            Mode::Setup => {
                let layout = Layout::vertical([
                    Constraint::Percentage(40),
                    Constraint::Length(3),
                    Constraint::Length(2),
                    Constraint::Percentage(40),
                ]);

                let [_, input_area, error_area, _] = f.area().layout(&layout);

                self.input.render(f, input_area, true);

                if let Some(err) = &self.error {
                    let error_widget = ratatui::widgets::Paragraph::new(err.clone())
                        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Red))
                        .block(ratatui::widgets::Block::bordered().title("Error"));

                    f.render_widget(error_widget, error_area);
                }
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
