use std::{cell::RefCell, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    style::{palette::material::GREEN, Style, Stylize},
    widgets::{Block, Row, Table, TableState},
};

use crate::data::shop::Shop;

use crate::ui::page::RenderablePage;

use super::shop::ShopPage;

pub struct ShopsPage {
    shops: Vec<Rc<RefCell<Shop>>>,
    shop_table_state: TableState,

    open_shop_page: Option<ShopPage>,
}

impl ShopsPage {
    pub fn new(shops: Vec<Rc<RefCell<Shop>>>) -> Self {
        Self {
            shop_table_state: TableState::default().with_selected(if !shops.is_empty() {
                Some(0)
            } else {
                None
            }),
            shops,
            open_shop_page: None,
        }
    }
}

impl RenderablePage for ShopsPage {
    fn title(&self) -> String {
        "Shops".into()
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let table = Table::new(
            self.shops.iter().map(|shop| {
                let s = shop.borrow().name.clone();
                Row::new(vec!["Generic".to_string(), s])
            }),
            [1, 50],
        )
        .header(Row::new(vec!["Category", "Name"]).style(Style::default().bg(GREEN.c600)))
        .block(Block::bordered())
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
                    KeyCode::Up => {
                        self.shop_table_state.scroll_up_by(1);
                        None
                    }
                    KeyCode::Down => {
                        self.shop_table_state.scroll_down_by(1);
                        None
                    }
                    KeyCode::Enter => {
                        let shop = Rc::clone(&self.shops[self.shop_table_state.selected()?]);
                        self.open_shop_page = Some(ShopPage::new(shop));
                        None
                    }
                    _ => Some(event),
                }
            }
            _ => Some(event),
        }
    }
}
