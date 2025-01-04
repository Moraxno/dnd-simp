use std::{cell::RefCell, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind}, layout::Rect, style::{palette::material::GREEN, Style, Stylize}, widgets::{Block, Row, Table, TableState}
};

use crate::data::shop::Shop;

use crate::ui::page::RenderablePage;

use super::{shop::ShopPage, translator::I18ner};

pub struct ShopsPage<'a> {
    shops: Vec<Rc<RefCell<Shop<'a>>>>,
    shop_table_state: TableState,

    open_shop_page: Option<ShopPage<'a>>,
}

impl<'a> ShopsPage<'a> {
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

    fn draw_self(&mut self, frame: &mut ratatui::Frame, area: Rect, i18n: &dyn I18ner) {
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
        // .row_highlight_style(Style::new().white().on_green())
        .highlight_symbol(">> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        frame.render_stateful_widget(table, area, &mut self.shop_table_state);
    }

    fn handle_shopspage_event(&mut self, event: &Event) {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up => {
                        self.shop_table_state.scroll_up_by(1);
                    }
                    KeyCode::Down => {
                        self.shop_table_state.scroll_down_by(1);
                    }
                    KeyCode::Enter => {
                        let opt_idx = self.shop_table_state.selected();

                        if let Some(idx) = opt_idx {
                            let shop = Rc::clone(&self.shops[idx]);
                            self.open_shop_page = Some(ShopPage::new(shop));
                        }
                    }
                    KeyCode::Esc => {
                        self.open_shop_page = None;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

impl<'a> RenderablePage for ShopsPage<'a> {
    fn title(&self) -> String {
        "Shops".into()
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, i18n: &dyn I18ner) {
        if let Some(ref mut page) = self.open_shop_page {
            page.draw(frame, area, i18n);
        } else {
            self.draw_self(frame, area, i18n);
        }
    }

    fn handle_and_transact(&mut self, event: &Event) {
        if let Some(ref mut page) = self.open_shop_page {
            page.handle_and_transact(event);
        } else {
            self.handle_shopspage_event(event);
        }
    }
}
