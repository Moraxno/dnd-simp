use ratatui::{crossterm::event::Event, layout::Rect, Frame};

pub trait RenderablePage {
    fn title(&self) -> String;
    fn draw(&mut self, frame: &mut Frame, area: Rect);
    fn handle_and_transact(&mut self, event: &Event);
}
