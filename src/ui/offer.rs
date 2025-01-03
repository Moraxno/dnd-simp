use std::{cell::RefCell, cmp::min, rc::Rc};

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers}, layout::{Constraint, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::{Block, Paragraph, Wrap}
};

use crate::{
    data::{gold::GoldAmount, shop::Shop},
    registry::ItemType,
};
use crate::{
    registry::{xanathar_magic_item_cost, CostExpressionFunction},
    ui::flow::KeyHandler,
};

use crate::ui::key::KeyMenu;
use super::{display::AsRatatuiSpan, flow::HandlesKeyEvents, page::RenderablePage, translator::{I18nPhrase, I18ner}};
use tyche::Expr;

use crate::data::gold::AsGoldCurrency;

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

    pub fn hide_prices(&mut self) {
        self.current_offer = self
            .current_offer
            .iter()
            .map(|offer| 
                Offer { item: offer.item.clone(), price: None })
            .collect();
    }

    pub fn realize_prices(&mut self, cost_expr: &CostExpressionFunction) {
        self.current_offer = self
            .current_offer
            .iter()
            .map(|offer| {
                let opt_d_expr: Result<Expr, _> = cost_expr(&offer.item).as_str().parse();

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
                        Some((val as isize).gold())
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

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, i18n: &dyn I18ner) {
        let [offers_area, menu_area] = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .areas(area);
        
        let offer_areas: [Rect; 3] = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Fill(1); 3])
            .areas(offers_area);

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
                Line::from(vec![
                    Span::raw(offer.item.category.to_string()).italic(),
                    Span::raw(", "),
                    offer.item.rarity.as_span(),
                ]),
                Line::raw(" "),
                Line::raw(offer.item.details.clone()),
            ])
            .block(block)
            .wrap(Wrap { trim: true });

            frame.render_widget(par, upper_area);

            let l = if let Some(price) = &offer.price {
                Line::from(vec![
                    Span::raw(i18n.i18n(I18nPhrase::Price)),
                    Span::raw(" "),
                    Span::raw(price.to_string())
                    ]).centered()
            } else {
                Line::from(vec![
                    Span::raw(i18n.i18n(I18nPhrase::Roll)),
                    Span::raw(" "),
                    Span::raw(xanathar_magic_item_cost(&offer.item)),
                    Span::raw(" gp"),
                ])
                .centered()
            };

            frame.render_widget(l, lower_area);

            frame.render_widget(KeyMenu { handles: self.get_handlers() }, menu_area);
        }
    }

    fn handle_and_transact(&mut self, event: &ratatui::crossterm::event::Event) {
        if let Event::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return;
            }

            log::debug!("KeyCode for Offer {:?}", key_event);

            match key_event.code {
                KeyCode::Right => self.offer_idx = min(2, self.offer_idx + 1),
                KeyCode::Left => self.offer_idx = self.offer_idx.saturating_sub(1),
                KeyCode::Char('P') => self.hide_prices(),
                KeyCode::Char('p') => self.realize_prices(&xanathar_magic_item_cost),
                _ => {}  
            }
        }
    }
}

impl HandlesKeyEvents for OfferPage {
    fn get_handlers(&self) -> Vec<super::flow::KeyHandler> {
        vec![
            KeyHandler {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                command: "Next Offer".into(),
            },
            KeyHandler {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                command: "Previous Offer".into(),
            },
            KeyHandler {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
                command: "(Re)roll prices".into(),
            },
            KeyHandler {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::SHIFT,
                command: "Show price formula".into(),
            },
        ]
    }
}
