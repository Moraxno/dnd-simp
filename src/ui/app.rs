use std::{fs, io::Read, iter::Enumerate, str::FromStr};

use layout::Flex;
use ratatui::{crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, MouseEventKind}, execute, style::Color}, style::Style, widgets::{block::title, Block, Clear, List, ListState, Paragraph, Row, Table, TableState}, DefaultTerminal, Frame};
use ratatui::prelude::*;

use chrono::Utc;

use crate::{campaign::{self, Campaign}, registry::ItemRegistry, shop::Shop};

enum AppPopup {
    WhatToDoWithShop { index: usize },

}

struct App<'a> {
    // registry: ItemRegistry,
    campaign: &'a mut Campaign,
    registry_state: TableState,
    is_running: bool,

    overlay: Option<Box<dyn AppOverlay>>,

    selected_tab: usize,

    messages: Vec<AppMessage>,
}



#[derive(Debug, Clone)]
pub enum AppCategory {
    Shops,
    Weather
}

const TABS: [AppCategory; 2] = [AppCategory::Shops, AppCategory::Weather];

const APP_TITLE: &str = "DnD Simp";

#[derive(Debug, Clone)]
pub enum AppMessage {
    SwitchCategory(AppCategory),
    PreviousCategory,
    NextCategory
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
                AppMessage::NextCategory => if self.selected_tab < TABS.len() - 1 {
                    self.selected_tab += 1;
                },
                AppMessage::PreviousCategory => if self.selected_tab > 0 {
                    self.selected_tab -= 1;
                },
                AppMessage::SwitchCategory(cat) => match cat {
                    AppCategory::Shops => self.selected_tab = 0,
                    AppCategory::Weather => self.selected_tab = 1,
                },
                _ => { /* do nothing, otherwise */ },   
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
                .into_iter()
                .map(|shop| Row::new(vec!["S", shop.name.as_str()])), [1, 50])
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
    }

    fn handle_events(&mut self) -> anyhow::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let Some(over) = &mut self.overlay {
            let ctrl = over.handle_key_event(key_event);

            match ctrl {
                FlowControl::ClosePopup => { self.overlay = None },
                FlowControl::NoOperation => {},
            }
        } else {
            let idx = self.registry_state.selected();
        
            let i = idx.unwrap();

            let shop = &self.campaign.get_shops()[i];

            match key_event.code {

                KeyCode::Enter => {self.overlay = Some(Box::new(ShopSelectMenuPopup::new(shop.name.clone(), shop.clone())))},
                
                KeyCode::Char('q') => self.exit(),
                KeyCode::Esc => {self.overlay = None},
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
    shop: Shop
}

enum FlowControl {
    ClosePopup,
    NoOperation
}

trait AppOverlay {
    fn draw(&self, frame: &mut Frame, area: Rect);
    fn handle_key_event(&mut self, key_event: KeyEvent) -> FlowControl;
}

impl ShopSelectMenuPopup {
    pub fn new(shop_name: String, shop: Shop) -> Self {
        Self {
            shop_name,
            shop
        }
    }
}
impl AppOverlay for ShopSelectMenuPopup {
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

        let offer = self.shop.produce_offer();

        frame.render_widget(Clear, area); //this clears out the background

        for (i, op_area) in options.iter().enumerate() {
            if offer.len() <= i {
                break;
            }

            let offer_name = &offer[i as usize].name;

            let par = Paragraph::new(offer_name.as_str())
                .block(Block::bordered())
                .centered();
            frame.render_widget(par, *op_area);
        }

        frame.render_widget(block, area);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> FlowControl {
        match key_event.code {
            KeyCode::Esc => FlowControl::ClosePopup,
            // KeyCode::Up => self.registry_state.scroll_up_by(1),
            // KeyCode::Down => self.registry_state.scroll_down_by(1),
            _ => FlowControl::NoOperation
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