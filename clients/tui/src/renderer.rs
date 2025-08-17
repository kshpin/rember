use ratatui::{
    prelude::{Buffer, Rect, Widget},
    widgets::{Block, Paragraph},
};

use crate::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let search_area = Rect::new(0, 0, area.width, 3).clamp(area);
        self.render_search_bar(&self.search_text, search_area, buf);
    }
}

impl App {
    fn render_search_bar(&self, text: &str, area: Rect, buf: &mut Buffer) {
        let search_bar = Paragraph::new(text).block(Block::bordered().title("Search"));
        search_bar.render(area, buf);
    }
}
