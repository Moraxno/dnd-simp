use std::{fs, io::Read};

use ratatui::{crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::Style, widgets::{Block, ListState, Row, TableState}, DefaultTerminal, Frame};
use ratatui::prelude::Stylize;

use crate::registry::ItemRegistry;

struct App {
    registry: ItemRegistry,

    registry_state: TableState,

    is_running: bool,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        let f = fs::File::open("registry.yaml")?;
        Ok(Self {
            registry: ItemRegistry::from_reader(f)?,
            registry_state: TableState::default().with_selected(Some(0)),
            is_running: true,
        })
    }

    pub fn exit(&mut self) {
        self.is_running = false;
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.is_running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let l = ratatui::widgets::Table::new(
            self.registry
                .items()
                .into_iter()
                .map(|item| Row::new(vec![item.rarity.as_symbol(), item.name.clone()])), [1, 50])
            .block(Block::bordered().title("List"))
            .style(Style::new().white())
            .row_highlight_style(Style::new().white().on_green())
            .header(Row::new(vec!["  ", "Seltenheit"]))
            .highlight_symbol(">> ")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always);

        frame.render_stateful_widget(l, frame.area(), &mut self.registry_state);
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.registry_state.scroll_up_by(1),
            KeyCode::Down => self.registry_state.scroll_down_by(1),
            _ => {}
        }
    }
}
pub fn run_app() -> anyhow::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new()?;
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}