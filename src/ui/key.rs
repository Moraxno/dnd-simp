use ratatui::crossterm::event::{KeyCode, KeyModifiers};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::prelude::*;
use super::flow::KeyHandler;

pub struct KeyMenu {
    pub handles: Vec<KeyHandler>,
}

fn key_span<'a>(handler: &KeyHandler) -> Vec<Span<'a>> {
    let mut spans = vec![];

    if handler.modifiers != KeyModifiers::NONE {
        spans.push(Span::from(handler.modifiers.to_string()).black().on_white());
        spans.push(Span::raw("+").black().on_white());
    }
    
    spans.append(&mut vec![
        Span::from(handler.code.to_string()).black().on_white(),
        Span::raw(" "),
        Span::raw(handler.command.clone()),
    ]);

    spans
}

impl Widget for KeyMenu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let b = Block::bordered()
            .border_set(symbols::border::DOUBLE);

        let inner = b.inner(area);
        b.render(area, buf);

        let spans: Vec<Vec<Span>> = self.handles
            .iter()
            .map(|handler| 
                key_span(handler))
            .collect();

        let true_spans = spans.join(&Span::from("    "));
        let l = Line::from(true_spans);

        let p = Paragraph::new(Text::from(l));
        p.render(inner, buf);
    }
}