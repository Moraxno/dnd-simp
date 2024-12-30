use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    style::{Style, Stylize},
    widgets::{Block, Row, Table, TableState},
};

use crate::data::shop::Shop;

use super::page::RenderablePage;

pub struct ShopsPage<'a> {
    shops: &'a Vec<Shop>,
    shop_table_state: TableState,
}

impl<'a> ShopsPage<'a> {
    pub fn new(shops: &'a mut Vec<Shop>) -> Self {
        Self {
            shops,
            shop_table_state: TableState::default().with_selected(if shops.len() > 0 {
                Some(0)
            } else {
                None
            }),
        }
    }
}

impl<'a> RenderablePage for ShopsPage<'a> {
    fn title(&self) -> &'static str {
        "Shops"
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let table = Table::new(
            self.shops
                .iter()
                .map(|shop| Row::new(vec!["Generic", shop.name.as_str()])),
            [1, 50],
        )
        .header(Row::new(vec!["Category", "Name"]))
        .block(Block::bordered().title("List of Shops"))
        .style(Style::new().white())
        .row_highlight_style(Style::new().white().on_green())
        .highlight_symbol(">> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        frame.render_stateful_widget(table, area, &mut self.shop_table_state);
    }

    fn handle_and_transact(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up =>   { self.shop_table_state.scroll_up_by(1); None },
                    KeyCode::Down => { self.shop_table_state.scroll_down_by(1); None },
                    _ => Some(event),
                }
            }
            _ => Some(event),
        }
    }
}
