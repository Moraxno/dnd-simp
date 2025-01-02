use layout::Flex;
use ratatui::prelude::*;
use ratatui::widgets::canvas::Shape;
use ratatui::widgets::{Borders, Padding};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        style::Color,
    },
    style::Style,
    widgets::{Block, Clear, Paragraph, TableState},
    DefaultTerminal, Frame,
};
use style::palette::material::{GRAY as SLATE, RED};

use crate::campaign::WorkCampaign;
use crate::data::shop::Shop;

use super::home::HomePage;
use super::page::RenderablePage;
use super::shops::ShopsPage;

enum AppPopup {
    WhatToDoWithShop { index: usize },
}

struct App {
    // registry: ItemRegistry,
    name: String,
    registry_state: TableState,
    is_running: bool,

    overlay: Option<Box<dyn AppScreen>>,

    pages: Vec<Box<dyn RenderablePage>>,

    selected_tab: usize,

    messages: Vec<AppMessage>,
}

#[derive(Debug, Clone)]
pub enum AppCategory {
    Shops,
    Weather,
}

const TABS: [AppCategory; 2] = [AppCategory::Shops, AppCategory::Weather];

const APP_TITLE: &str = "DnD Simp";

#[derive(Debug, Clone)]
pub enum AppMessage {
    SwitchCategory(AppCategory),
    PreviousCategory,
    NextCategory,
}

impl App {
    pub fn new(campaign: &'static mut WorkCampaign) -> anyhow::Result<Self> {
        Ok(Self {
            name: campaign.name.clone(),
            registry_state: TableState::default().with_selected(Some(0)),
            is_running: true,
            overlay: None,
            pages: vec![
                Box::new(HomePage::new()),
                Box::new(ShopsPage::new(campaign.shops.clone())),
            ],
            selected_tab: 0,
            messages: vec![],
        })
    }

    pub fn exit(&mut self) {
        self.is_running = false;
    }

    pub fn current_overlay(&mut self) -> &mut Box<dyn RenderablePage> {
        &mut self.pages[self.selected_tab]
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.is_running {
            self.update();
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn update(&mut self) {
        while let Some(msg) = self.messages.pop() {
            match &msg {
                AppMessage::NextCategory => {
                    if self.selected_tab < TABS.len() - 1 {
                        self.selected_tab += 1;
                    }
                }
                AppMessage::PreviousCategory => {
                    if self.selected_tab > 0 {
                        self.selected_tab -= 1;
                    }
                }
                AppMessage::SwitchCategory(cat) => match cat {
                    AppCategory::Shops => self.selected_tab = 0,
                    AppCategory::Weather => self.selected_tab = 1,
                },
                _ => { /* do nothing, otherwise */ }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [title_area, subline_area, border_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .areas(frame.area());

        let [app_name_area, _, campaign_name_area, _, object_ident_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(APP_TITLE.len() as u16 + 2),
                Constraint::Length(1),
                Constraint::Length(self.name.len() as u16 + 2),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .areas(title_area);

        let app_name = ratatui::widgets::Paragraph::new(APP_TITLE)
            .alignment(Alignment::Center)
            .bg(Color::Blue);
        frame.render_widget(app_name, app_name_area);

        let campaign_name = ratatui::widgets::Paragraph::new(self.name.as_str())
            .alignment(Alignment::Center)
            .bold()
            .bg(Color::Yellow);
        frame.render_widget(campaign_name, campaign_name_area);

        let object_ident = ratatui::widgets::Paragraph::new(self.name.as_str())
            .alignment(Alignment::Center)
            .bg(Color::Grey);
        frame.render_widget(object_ident, object_ident_area);

        let page_tabs = ratatui::widgets::Tabs::new(
            self.pages
                .iter()
                .map(|page| ratatui::text::Line::raw(format!("  {}  ", page.title()))),
        )
        .select(self.selected_tab)
        .highlight_style(Style::new().fg(SLATE.c300).bg(RED.a100))
        .padding("", "")
        .divider("");

        let [tab_area] = Layout::default()
            .flex(Flex::Center)
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Min(10)])
            .areas(subline_area);

        let block = Block::new()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(3))
            .border_style(RED.a100);

        let content_area = block.inner(border_area);

        frame.render_widget(page_tabs, tab_area);
        frame.render_widget(block, border_area);

        self.pages[self.selected_tab].draw(frame, content_area);

        // let l = ratatui::widgets::Table::new(
        //     self.campaign
        //         .get_shops()
        //         .iter()
        //         .map(|shop| Row::new(vec!["S", shop.name.as_str()])),
        //     [1, 50],
        // )
        // .block(Block::bordered().title(self.campaign.name.clone()))
        // .style(Style::new().white())
        // .row_highlight_style(Style::new().white().on_green())
        // // .header(Row::new(vec!["  ", "Shop"]))
        // .highlight_symbol(">> ")
        // .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        // frame.render_stateful_widget(l, content_area, &mut self.registry_state);

        if let Some(over) = &self.overlay {
            over.draw(frame, popup_area(border_area, 50, 50));
        }
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        let ev = event::read()?;
        self.current_overlay().handle_and_transact(&ev);

        match ev {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let Some(over) = &mut self.overlay {
            let ctrl = over.handle_key_event(key_event);

            match ctrl {
                FlowControl::ClosePopup => self.overlay = None,
                FlowControl::NoOperation => {}
            }
        } else {
            //     let idx = self.registry_state.selected();

            //     let i = idx.unwrap();

            //     let shop = &self.campaign.get_shops()[i];

            match key_event.code {
                //         KeyCode::Enter => {
                //             self.overlay = Some(Box::new(ShopSelectMenuPopup::new(
                //                 shop.name.clone(),
                //                 shop.clone(),
                //             )))
                //         }

                //         KeyCode::Char('q') => self.exit(),
                //         KeyCode::Esc => self.overlay = None,
                // KeyCode::Up => self.registry_state.scroll_up_by(1),
                // KeyCode::Down => self.registry_state.scroll_down_by(1),
                KeyCode::Tab => self.messages.push(AppMessage::NextCategory),
                KeyCode::BackTab => self.messages.push(AppMessage::PreviousCategory),
                _ => {}
            }
        }

        // Always allow to quit
        if let KeyCode::Char('q') = key_event.code {
            self.exit()
        }
    }
}

struct ShopSelectMenuPopup {
    shop_name: String,
    shop: Shop,
}

enum FlowControl {
    ClosePopup,
    NoOperation,
}

trait AppScreen {
    fn draw(&self, frame: &mut Frame, area: Rect);
    fn handle_key_event(&mut self, key_event: KeyEvent) -> FlowControl;
}

// // struct WelcomeScreen {}

// // impl AppScreen for WelcomeScreen {
// //     fn draw(&self, frame: &mut Frame, area: Rect) {
// //         let shortest_side = min(area.width, 2 * area.height);
// //         let [inner_area] = Layout::default()
// //             .direction(Direction::Horizontal)
// //             .constraints(vec![
// //                 Constraint::Length(shortest_side)
// //             ])
// //             .flex(Flex::Center)
// //             .areas(area);
// //         let [draw_area] = Layout::default()
// //             .direction(Direction::Vertical)
// //             .constraints(vec![
// //                 Constraint::Length(2 * shortest_side)
// //             ])
// //             .flex(Flex::Center)
// //             .areas(inner_area);

// //         let c = Canvas::default()
// //             .block(Block::bordered())
// //             .marker(Marker::Braille)
// //             .paint(|ctx| {
// //                 // let side_angle = 60.0_f64;
// //                 // let offset = 30.0_f64;
// //                 // let r = 20.0_f64;
// //                 // for step in 0..6 {
// //                 //     let angle = (step as f64 * side_angle + offset).to_radians();
// //                 //     let next_angle = ((step + 1) as f64 * side_angle + offset).to_radians();
// //                 //     ctx.draw(&Line::new(r * angle.cos(), r * angle.sin(), r * next_angle.cos(), r * next_angle.sin(), Color::White.into()));
// //                 // }
// //                 render_d20(ctx, 10.0);
// //             })
// //             .x_bounds([-20.0, 20.0])
// //             .y_bounds([-20.0, 20.0]);

// //         frame.render_widget(c, draw_area);
// //     }

// //     fn handle_key_event(&mut self, _key_event: KeyEvent) -> FlowControl {
// //         FlowControl::NoOperation
// //     }
// // }

impl ShopSelectMenuPopup {
    pub fn new(shop_name: String, shop: Shop) -> Self {
        Self { shop_name, shop }
    }
}
impl AppScreen for ShopSelectMenuPopup {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(format!("Selecting {}", self.shop_name));

        let options: [Rect; 3] = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ])
            .areas(area);

        let offer = self.shop.produce_offer(3);

        frame.render_widget(Clear, area); //this clears out the background

        for (i, op_area) in options.iter().enumerate() {
            if offer.len() <= i {
                break;
            }

            let offer_name = &offer[i].name;

            let [title_area, desc_area] = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Fill(1), Constraint::Fill(2)])
                .areas(*op_area);

            let par = Paragraph::new(offer_name.as_str()).centered().bold();

            let rare_string = offer[i].rarity.as_string();

            let par2 = Paragraph::new(rare_string.as_str()).centered().italic();
            frame.render_widget(par, title_area);
            frame.render_widget(par2, desc_area);
        }

        frame.render_widget(block, area);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> FlowControl {
        match key_event.code {
            KeyCode::Esc => FlowControl::ClosePopup,
            // KeyCode::Up => self.registry_state.scroll_up_by(1),
            // KeyCode::Down => self.registry_state.scroll_down_by(1),
            _ => FlowControl::NoOperation,
        }
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub fn run_app(campaign: &'static mut WorkCampaign) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(campaign)?;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
