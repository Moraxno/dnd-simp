use std::{cell::RefCell, cmp::min, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

use crate::ui::display::HasCostExpression;
use crate::{
    data::{gold::GoldAmount, shop::Shop},
    registry::ItemType,
};

use super::{display::AsRatatuiSpan, page::RenderablePage};
use tyche::Expr;

#[derive(Debug)]
struct Offer {
    pub item: ItemType,
    pub price: Option<GoldAmount>,
}

#[derive(Debug)]
pub struct OfferPage {
    shop: Rc<RefCell<Shop>>,

    current_offer: Vec<Offer>,

    offer_idx: usize,
}

impl OfferPage {
    pub fn new(shop: Rc<RefCell<Shop>>) -> Self {
        let current_offer = shop
            .borrow()
            .produce_offer(3)
            .into_iter()
            .cloned()
            .map(|i| Offer {
                item: i,
                price: None,
            })
            .collect();
        Self {
            shop,
            current_offer,
            offer_idx: 0,
        }
    }

    pub fn realize_prices(&mut self) {
        self.current_offer = self
            .current_offer
            .iter()
            .map(|offer| {
                let opt_d_expr: Result<Expr, _> = offer.item.price_expr().as_str().parse();

                let roll = if let Ok(d_expr) = opt_d_expr {
                    d_expr
                        .eval(&mut tyche::dice::roller::FastRand::default())
                        .ok()
                        .map(|ev| ev.calc())
                } else {
                    None
                };

                let price = if let Some(real_roll) = roll {
                    if let Ok(val) = real_roll {
                        Some(GoldAmount::new(val as isize, 0, 0))
                    } else {
                        None
                    }
                } else {
                    None
                };

                Offer {
                    item: offer.item.clone(),
                    price,
                }
            })
            .collect();
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

        for (idx, (offer_area, offer)) in offer_areas
            .into_iter()
            .zip(self.current_offer.iter())
            .enumerate()
        {
            let [upper_area, lower_area] = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(3)])
                .areas(offer_area);

            let block = if idx == self.offer_idx {
                Block::bordered().border_type(ratatui::widgets::BorderType::Thick)
            } else {
                Block::bordered().border_type(ratatui::widgets::BorderType::Plain)
            };

            let par = Paragraph::new(vec![
                Line::raw(offer.item.name.clone()),
                Line::from(offer.item.rarity.as_span()),
                Line::raw(offer.item.details.clone()),
            ])
            .block(block);

            frame.render_widget(par, upper_area);

            let l = if let Some(price) = &offer.price {
                Line::from(vec![Span::raw("Price: "), Span::raw(price.to_string())]).centered()
            } else {
                Line::from(vec![
                    Span::raw("Price: "),
                    Span::raw(offer.item.price_expr()),
                    Span::raw(" gp"),
                ])
                .centered()
            };

            frame.render_widget(l, lower_area);
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
                KeyCode::Char('p') => self.realize_prices(),
                _ => {}
            }
        }
    }
}
