use ratatui::{crossterm::event::Event, layout::Rect, Frame};

use super::translator::I18ner;

pub trait RenderablePage {
    fn title(&self) -> String;
    fn draw(&mut self, frame: &mut Frame, area: Rect, i18n: &dyn I18ner);
    fn handle_and_transact(&mut self, event: &Event);
}
