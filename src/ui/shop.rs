use std::{cell::RefCell, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind}, style::{Style, Stylize}, widgets::{Block, Row, Table, TableState}
};

use crate::data::shop::Shop;

use super::page::RenderablePage;

pub struct ShopPage {
    shop: Rc<RefCell<Shop>>,
    inventory_table_state: TableState,
}

impl ShopPage {
    pub fn new(shop: Rc<RefCell<Shop>>) -> Self {
        Self {
            inventory_table_state: TableState::default().with_selected(
                if !shop.borrow().get_inventory().is_empty() {
                    Some(0)
                } else {
                    None
                },
            ),
            shop,
        }
    }
}

impl RenderablePage for ShopPage {
    fn title(&self) -> String {
        self.shop.borrow().name.clone()
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let table = Table::new(
            self.shop
                .borrow()
                .get_inventory()
                .iter()
                .map(|item| Row::new(vec![item.rarity.as_string(), item.name.clone()])),
            [1, 50],
        )
        .block(Block::bordered().title(self.shop.borrow().name.clone()))
        .row_highlight_style(Style::new().white().on_green())
        .highlight_symbol(">> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        frame.render_stateful_widget(table, area, &mut self.inventory_table_state);
    }

    fn handle_and_transact(
        &mut self,
        event: ratatui::crossterm::event::Event,
    ) -> Option<ratatui::crossterm::event::Event> {
        if let Event::Key(key_event) = &event {
            return match key_event.code {
                KeyCode::Up if key_event.kind == KeyEventKind::Press => { self.inventory_table_state.scroll_up_by(1); None },
                KeyCode::Down if key_event.kind == KeyEventKind::Press => { self.inventory_table_state.scroll_down_by(1); None }
                _ => Some(event)
            };
        }

        Some(event)
    }
}
