use crossterm::event::KeyCode;
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Paragraph},
};

pub struct InputBox {
    input: String,
    cursor: usize,
}
impl InputBox {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
        }
    }

    pub fn handle_paste(&mut self, data: String) {
        self.input.push_str(&data);
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        match key.code {
            KeyCode::Char(c) => self.enter_char(c),
            KeyCode::Backspace => self.delete_char(),
            KeyCode::Left => self.move_left(),
            KeyCode::Right => self.move_right(),
            _ => {}
        }
    }

    fn enter_char(&mut self, c: char) {
        let idx = self.byte_index();
        self.input.insert(idx, c);
        self.cursor += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor > 0 {
            let before = self.input.chars().take(self.cursor - 1);
            let after = self.input.chars().skip(self.cursor);
            self.input = before.chain(after).collect();
            self.cursor -= 1;
        }
    }

    fn move_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    fn move_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.input.chars().count());
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.input.len())
    }

    pub fn value(&self) -> &str {
        &self.input
    }

    pub fn render(&self, f: &mut Frame, area: Rect, active: bool) {
        let input = Paragraph::new(self.input.as_str()).block(Block::bordered().title("Input"));

        f.render_widget(input, area);

        if active {
            f.set_cursor_position((area.x + self.cursor as u16 + 1, area.y + 1));
        }
    }
}
