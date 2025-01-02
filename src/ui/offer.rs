use std::{cell::RefCell, cmp::min, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::{data::shop::Shop, registry::ItemType};

use super::{display::AsRatatuiSpan, page::RenderablePage};

#[derive(Debug)]
pub struct OfferPage {
    shop: Rc<RefCell<Shop>>,

    current_offer: Vec<ItemType>,

    offer_idx: usize,
}

impl OfferPage {
    pub fn new(shop: Rc<RefCell<Shop>>) -> Self {
        let current_offer = shop
            .borrow()
            .produce_offer(3)
            .into_iter().cloned()
            .collect();
        Self {
            shop,
            current_offer,
            offer_idx: 0,
        }
    }
}

impl RenderablePage for OfferPage {
    fn title(&self) -> String {
        format!("Offer for {}", self.shop.borrow().name)
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let offer_areas: [Rect; 3] = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Fill(1); 3])
            .areas(area);

        for (idx, (offer_area, item)) in offer_areas
            .into_iter()
            .zip(self.current_offer.iter())
            .enumerate()
        {
            let block = if idx == self.offer_idx {
                Block::bordered().border_type(ratatui::widgets::BorderType::Thick)
            } else {
                Block::bordered().border_type(ratatui::widgets::BorderType::Plain)
            };

            let par = Paragraph::new(vec![
                Line::raw(item.name.clone()),
                Line::from(item.rarity.as_span()),
                Line::raw(item.details.clone()),
            ])
            .block(block);

            frame.render_widget(par, offer_area);
        }
    }

    fn handle_and_transact(&mut self, event: &ratatui::crossterm::event::Event) {
        if let Event::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return;
            }

            match key_event.code {
                KeyCode::Right => self.offer_idx = min(2, self.offer_idx + 1),
                KeyCode::Left => self.offer_idx = self.offer_idx.saturating_sub(1),
                _ => {}
            }
        }
    }
}
