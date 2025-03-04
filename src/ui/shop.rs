use std::{cell::RefCell, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::Stylize,
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Paragraph, Row, Scrollbar, ScrollbarState, Table, TableState, Wrap,
    }, Frame,
};

use crate::data::{item::ItemType, shop::{Shop, StockedItem}};

use super::{offer::OfferPage, page::RenderablePage, translator::I18ner};

use strum::IntoEnumIterator;
use tui_markdown;

use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum FocusedArea {
    Inventory,
    Details,
}

impl FocusedArea {
    fn next(&self) -> Self {
        match self {
            Self::Inventory => Self::Details,
            Self::Details => Self::Inventory,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Self::Inventory => Self::Details,
            Self::Details => Self::Inventory,
        }
    }
}

#[derive(Debug)]
enum Transaction {
    ScrollUp,
    ScrollDown,
    PageUp,
    PageDown,
    ShiftFocusForward,
    ShiftFocusBackward,
    CreateOffer,
}

#[derive(Debug)]
pub struct ShopPage<'a> {
    shop: &'a Shop<'a>,
    inventory_table_state: TableState,
    focus: FocusedArea,

    // refactor this into a f'ing useful abstraction for scrolling text content
    detail_scroll_state: ScrollbarState,
    detail_scroll: u16,
    detail_height: u16,

    overlay_page: Option<OfferPage<'a>>,

    transactions: VecDeque<Transaction>,
}

impl<'a> ShopPage<'a> {
    pub fn new(shop: &'a Shop<'a>) -> Self {
        Self {
            inventory_table_state: TableState::default().with_selected(
                if !shop.get_inventory().is_empty() {
                    Some(0)
                } else {
                    None
                },
            ),
            shop,
            focus: FocusedArea::Inventory,
            transactions: VecDeque::new(),
            detail_scroll: 0,
            detail_height: 10, // questionable initializer
            overlay_page: None,
            detail_scroll_state: ScrollbarState::default(),
        }
    }

    pub fn selected_item(&self) -> Option<StockedItem> {
        let idx = self.inventory_table_state.selected()?;
        let item = self.shop.get_inventory()[idx].to_owned();
        Some(item)
    }

    fn perform_transactions(&mut self) {
        while let Some(transaction) = self.transactions.pop_front() {
            let maybe_transaction = self.transact(transaction);
            if let Some(new_transaction) = maybe_transaction {
                self.transactions.push_back(new_transaction);
            }
        }
    }

    fn transact(&mut self, transaction: Transaction) -> Option<Transaction> {
        match transaction {
            Transaction::ScrollUp => match self.focus {
                FocusedArea::Details => self.detail_scroll = self.detail_scroll.saturating_sub(1),
                FocusedArea::Inventory => {
                    self.inventory_table_state.scroll_up_by(1);
                    self.detail_scroll = 0;
                }
            },
            Transaction::ScrollDown => match self.focus {
                FocusedArea::Details => self.detail_scroll = self.detail_scroll.saturating_add(1),
                FocusedArea::Inventory => {
                    self.inventory_table_state.scroll_down_by(1);
                    self.detail_scroll = 0;
                }
            },
            Transaction::PageUp => match self.focus {
                FocusedArea::Details => {
                    self.detail_scroll = self.detail_scroll.saturating_sub(self.detail_height)
                }
                FocusedArea::Inventory => {
                    self.inventory_table_state.scroll_up_by(self.detail_height); // @todo wrong variable, should be height of inventory
                    self.detail_scroll = 0;
                }
            },
            Transaction::PageDown => match self.focus {
                FocusedArea::Details => {
                    self.detail_scroll = self.detail_scroll.saturating_add(self.detail_height)
                }
                FocusedArea::Inventory => {
                    self.inventory_table_state
                        .scroll_down_by(self.detail_height); // @todo wrong variable, should be height of inventory
                    self.detail_scroll = 0;
                }
            },
            Transaction::ShiftFocusForward => self.focus = self.focus.next(),
            Transaction::ShiftFocusBackward => self.focus = self.focus.previous(),
            Transaction::CreateOffer => {
                self.overlay_page = Some(OfferPage::new(self.shop));
            }
        }

        log::info!("Transaction in ShopPage. New State: {:?}", self);

        None
    }

    fn border_type_for_area(&self, content: FocusedArea) -> BorderType {
        if self.focus == content {
            BorderType::Thick
        } else {
            BorderType::Plain
        }
    }

    fn draw_self(&mut self, frame: &mut Frame, area: Rect, i18n: &dyn I18ner) {
        let [content_area, menu_area] = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .areas(area);

        let [inventory_area, details_area] = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .areas(content_area);

        let table = Table::new(
            self.shop
                .get_inventory()
                .iter()
                .map(|item| Row::new(vec![item.item_type.rarity.to_string(), item.item_type.name.clone()])),
            [1, 50],
        )
        .block(
            Block::bordered()
                .title(self.shop.name.clone())
                .border_type(self.border_type_for_area(FocusedArea::Inventory)),
        )
        //.row_highlight_style(Style::new().white().on_green())
        .highlight_symbol(">> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);
        frame.render_stateful_widget(table, inventory_area, &mut self.inventory_table_state);

        let s: String = if let Some(item) = self.selected_item() {
            item.item_type.details.clone()
        } else {
            "(no item selected)".into()
        };

        let text = tui_markdown::from_str(s.as_str());

        // @todo find a way to detemine the length of the content properly
        // self.detail_scroll = min(self.detail_scroll, lines as u16);
        self.detail_scroll_state.position(self.detail_scroll.into());

        let paragraph_block = Block::bordered()
            .title("Detailtext")
            .border_type(self.border_type_for_area(FocusedArea::Details));

        self.detail_height = paragraph_block.inner(area).height;

        let nice_paragraph = Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .scroll((self.detail_scroll, 0))
            .block(paragraph_block);

        frame.render_widget(nice_paragraph, details_area);

        let scroll_bar =
            Scrollbar::default().orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight);

        frame.render_stateful_widget(
            scroll_bar,
            details_area.inner(Margin {
                vertical: 1,
                horizontal: 0,
            }),
            &mut self.detail_scroll_state,
        );

        let menu_bar = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw("o").black().on_white(),
            Span::raw(" generate offer"),
        ])]))
        .centered();

        frame.render_widget(menu_bar, menu_area);
    }
}

impl<'a> RenderablePage for ShopPage<'a> {
    fn title(&self) -> String {
        self.shop.name.clone()
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, i18n: &dyn I18ner) {
        if let Some(ref mut page) = &mut self.overlay_page {
            page.draw(frame, area, i18n);
        } else {
            self.draw_self(frame, area, i18n);
        }
    }

    fn handle_and_transact(&mut self, event: &Event) {
        if let Event::Key(key_event) = &event {
            match key_event.code {
                KeyCode::Esc if key_event.kind == KeyEventKind::Press => {
                    self.overlay_page = None;
                    return;
                },
                _ => {},
            }
        }

        if let Some(ref mut page) = &mut self.overlay_page {
            page.handle_and_transact(event);
        } else if let Event::Key(key_event) = &event {
            log::debug!("ShopPage handled event {:?}", key_event);

            match key_event.code {
                KeyCode::Up if key_event.kind == KeyEventKind::Press => {
                    self.transactions.push_back(Transaction::ScrollUp)
                }
                KeyCode::Down if key_event.kind == KeyEventKind::Press => {
                    self.transactions.push_back(Transaction::ScrollDown)
                }
                KeyCode::PageUp if key_event.kind == KeyEventKind::Press => {
                    self.transactions.push_back(Transaction::PageUp)
                }
                KeyCode::PageDown if key_event.kind == KeyEventKind::Press => {
                    self.transactions.push_back(Transaction::PageDown)
                }
                KeyCode::Right if key_event.kind == KeyEventKind::Press => {
                    self.transactions.push_back(Transaction::ShiftFocusForward)
                }
                KeyCode::Left if key_event.kind == KeyEventKind::Press => {
                    self.transactions.push_back(Transaction::ShiftFocusBackward)
                }
                KeyCode::Char('o') if key_event.kind == KeyEventKind::Press => {
                    self.transactions.push_back(Transaction::CreateOffer)
                }
                _ => {}
            };
        }
        self.perform_transactions();
    }
}
