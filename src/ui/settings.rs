use ratatui::{crossterm::event::{Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout}, text::Line};

use super::{key, page::RenderablePage};

#[derive(Debug)]
pub struct SettingsPage {
    my_string: String,
}

impl SettingsPage {
    pub fn new() -> Self {
        Self {
            my_string: String::new()
        }
    }
}

impl RenderablePage for SettingsPage {
    fn title(&self) -> String {
        "Settings".into()
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, i18n: &dyn super::translator::I18ner) {
        let [upper_area, below_area] = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![Constraint::Fill(1), Constraint::Fill(1),])
            .areas(area);

        frame.render_widget(Line::raw("There are no settings yet. But you can type some nice text below..."), upper_area);
        frame.render_widget(Line::raw(self.my_string.as_str()), below_area);
    }

    fn handle_and_transact(&mut self, event: &ratatui::crossterm::event::Event) {
        if let Event::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return
            }
    
            match key_event.code {
                KeyCode::Char(c) => self.my_string.push(c),
                _ => {},
            }
        }
    }
}