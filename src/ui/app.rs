use std::cmp::min;

use layout::Flex;
use rand::Fill;
use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Context, Line, Map, MapResolution, Shape};
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        style::Color,
    },
    style::Style,
    widgets::{Block, Clear, Paragraph, Row, TableState},
    DefaultTerminal, Frame,
};
use symbols::Marker;

use crate::{campaign::Campaign, data::shop::Shop};

enum AppPopup {
    WhatToDoWithShop { index: usize },
}

struct App<'a> {
    // registry: ItemRegistry,
    campaign: &'a mut Campaign,
    registry_state: TableState,
    is_running: bool,

    overlay: Option<Box<dyn AppScreen>>,

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

impl<'a> App<'a> {
    pub fn new(campaign: &'a mut Campaign) -> anyhow::Result<Self> {
        Ok(Self {
            campaign,
            registry_state: TableState::default().with_selected(Some(0)),
            is_running: true,
            overlay: None,
            selected_tab: 1,
            messages: vec![],
        })
    }

    pub fn exit(&mut self) {
        self.is_running = false;
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
        let [title_area, subline_area, content_area] = Layout::default()
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
                Constraint::Length(self.campaign.name.len() as u16 + 2),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .areas(title_area);

        let app_name = ratatui::widgets::Paragraph::new(APP_TITLE)
            .alignment(Alignment::Center)
            .bg(Color::Blue);
        frame.render_widget(app_name, app_name_area);

        let campaign_name = ratatui::widgets::Paragraph::new(self.campaign.name.as_str())
            .alignment(Alignment::Center)
            .bold()
            .bg(Color::Yellow);
        frame.render_widget(campaign_name, campaign_name_area);

        let object_ident = ratatui::widgets::Paragraph::new(self.campaign.name.as_str())
            .alignment(Alignment::Center)
            .bg(Color::Grey);
        frame.render_widget(object_ident, object_ident_area);

        let t = ratatui::widgets::Tabs::new(["Shops", "Weather"])
            .select(self.selected_tab)
            .divider(symbols::DOT);

        frame.render_widget(t, subline_area);

        let l = ratatui::widgets::Table::new(
            self.campaign
                .get_shops()
                .iter()
                .map(|shop| Row::new(vec!["S", shop.name.as_str()])),
            [1, 50],
        )
        .block(Block::bordered().title(self.campaign.name.clone()))
        .style(Style::new().white())
        .row_highlight_style(Style::new().white().on_green())
        // .header(Row::new(vec!["  ", "Shop"]))
        .highlight_symbol(">> ")
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        frame.render_stateful_widget(l, content_area, &mut self.registry_state);

        if let Some(over) = &self.overlay {
            over.draw(frame, popup_area(content_area, 50, 50));
        }

        let w = WelcomeScreen {};
        w.draw(frame, content_area);

    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
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
            let idx = self.registry_state.selected();

            let i = idx.unwrap();

            let shop = &self.campaign.get_shops()[i];

            match key_event.code {
                KeyCode::Enter => {
                    self.overlay = Some(Box::new(ShopSelectMenuPopup::new(
                        shop.name.clone(),
                        shop.clone(),
                    )))
                }

                KeyCode::Char('q') => self.exit(),
                KeyCode::Esc => self.overlay = None,
                KeyCode::Up => self.registry_state.scroll_up_by(1),
                KeyCode::Down => self.registry_state.scroll_down_by(1),
                KeyCode::Right => self.messages.push(AppMessage::NextCategory),
                KeyCode::Left => self.messages.push(AppMessage::PreviousCategory),
                _ => {}
            }
        }

        // Always allow to quit
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
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

struct WelcomeScreen {}

/// Renders a D20 to the given ctx at (0, 0)
/// Thanks to https://www.reddit.com/r/DnD/comments/go75gv/oc_flat_d20_sides_and_angles_for_home_projects/
/// for all the angles and lengths
pub fn render_d20(ctx: &mut Context, radius: f64) {
    let A = radius * 1.0_f64;
    let B = radius * 0.925_f64;
    let C = radius * 0.809_f64;
    let D = radius * 0.347_f64;

    let a = 98.182_f64.to_radians();
    let b = 76.364_f64.to_radians();
    let c = 60.0_f64.to_radians();
    let d = 51.818_f64.to_radians();
    let e = 21.818_f64.to_radians();

    let H = 3.0_f64.sqrt() / 2.0 * A;

    let top = (0.0_f64, H/2.0 + D);
    let left_upper = (top.0 - B * c.sin(), top.1 - B * c.cos());
    let right_upper = (top.0 + B * c.sin(), top.1 - B * c.cos());
    let left_lower = (left_upper.0, left_upper.1 - B);
    let right_lower = (right_upper.0, right_upper.1 - B);
    let bottom = (0.0, left_lower.1 + B * (2.0 * c).cos());

    let tri_top = (0.0, H/2.0);
    let tri_left = (-A/2.0, -H/2.0);
    let tri_right = (A/2.0, -H/2.0);

    let edges = [
        // outer hex
        (top, left_upper),
        (left_upper, left_lower),
        (left_lower, bottom),
        (bottom, right_lower),
        (right_lower, right_upper),
        (right_upper, top),

        // top connections
        (tri_top, left_upper),
        (tri_top, right_upper),
        (tri_top, top),

        // left connections
        (tri_left, left_lower),
        (tri_left, bottom),
        (tri_left, left_upper),

        // right connections
        (tri_right, right_lower),
        (tri_right, bottom),
        (tri_right, right_upper),

        // inner tri
        (tri_top, tri_left),
        (tri_left, tri_right),
        (tri_right, tri_top),
    ];

    for e in &edges {
        let line = Line::new(e.0.0, e.0.1, e.1.0, e.1.1, Color::Blue.into());
        ctx.draw(&line);
    }


    // for l in &lines {q
    //     ctx.draw(l);
    // }
}

impl AppScreen for WelcomeScreen {
    fn draw(&self, frame: &mut Frame, area: Rect) {
        let shortest_side = min(area.width, 2 * area.height);
        let [inner_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(shortest_side)
            ])
            .flex(Flex::Center)
            .areas(area);
        let [draw_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2 * shortest_side)
            ])
            .flex(Flex::Center)
            .areas(inner_area);

        let c = Canvas::default()
            .block(Block::bordered())
            .marker(Marker::Braille)
            .paint(|ctx| {
                // let side_angle = 60.0_f64;
                // let offset = 30.0_f64;
                // let r = 20.0_f64;
                // for step in 0..6 {
                //     let angle = (step as f64 * side_angle + offset).to_radians();
                //     let next_angle = ((step + 1) as f64 * side_angle + offset).to_radians();
                //     ctx.draw(&Line::new(r * angle.cos(), r * angle.sin(), r * next_angle.cos(), r * next_angle.sin(), Color::White.into()));
                // }
                render_d20(ctx, 10.0);
            })
            .x_bounds([-20.0, 20.0])
            .y_bounds([-20.0, 20.0]);
        
        frame.render_widget(c, draw_area);
    }

    fn handle_key_event(&mut self, _key_event: KeyEvent) -> FlowControl {
        FlowControl::NoOperation
    }
}

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
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(2),
                ])
                .areas(*op_area);

            let par = Paragraph::new(offer_name.as_str())
                .centered()
                .bold();

            let rare_string = offer[i].rarity.as_string();

            let par2 = Paragraph::new(rare_string.as_str())
                .centered()
                .italic();
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

pub fn run_app(campaign: &mut Campaign) -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(campaign)?;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
