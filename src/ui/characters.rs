use ratatui::{layout::{Constraint, Layout}, widgets::{Row, Table, TableState}};

use crate::data::{character::Character, item::Item};

use super::{flow::KeyHandler, key::KeyMenu, page::RenderablePage};

pub struct CharactersPage<'a> {
    pub characters: Vec<&'a Character<'a>>,
    character_table_state: TableState,
}

impl<'a> CharactersPage<'a> {
    pub fn new(characters: Vec<&'a Character>) -> Self {
        Self {
            character_table_state: TableState::default().with_selected(if characters.len() > 0 { Some(1) } else { None }),
            characters,
        }
    }

    pub fn selected_character(&self) -> Option<&'a Character> {
        Some(self.characters[self.character_table_state.selected()?])
    }
}

impl<'a> RenderablePage for CharactersPage<'a> {
    fn title(&self) -> String {
        "Characters".into()
    }

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect, i18n: &dyn super::translator::I18ner) {
        let [content_area, menu_area] = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(3)])
            .areas(area);

        let [char_list_area, char_info_area] = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([Constraint::Fill(1); 2])
            .areas(content_area);

        let table = Table::new(
            self.characters
                .iter()
                .map(|ch| Row::new(vec![ch.name().as_str()]))
                .collect::<Vec<_>>(),
                [Constraint::Fill(1)]
        );

        let wish_list = match self.selected_character() {
            Some(char) => { char.wish_list()
                .iter()
                .filter_map(|item|
                    if let Item::Concrete(item_type) = item {
                        Some(Row::new(vec![item_type.name.clone()]))
                    } else {
                        None
                    })
                .collect() },
            None => vec![],
        };

        let wish_list_table = Table::new(
            wish_list, [Constraint::Fill(1)]
        );

        frame.render_stateful_widget(table, char_list_area, &mut self.character_table_state);
        frame.render_widget(wish_list_table, char_info_area);
        frame.render_widget(KeyMenu { handles: vec![] }, menu_area);
    }

    fn handle_and_transact(&mut self, event: &ratatui::crossterm::event::Event) {
        
    }
}

