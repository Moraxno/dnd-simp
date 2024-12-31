use std::cmp::min;

use ratatui::{
    crossterm::event::Event,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{palette::material::WHITE, Color},
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Context, Line},
        Block,
    },
    Frame,
};

use super::page::RenderablePage;

pub struct HomePage {
    color: Color,
}

impl HomePage {
    pub fn new() -> Self {
        Self { color: WHITE }
    }
}

impl<'a> RenderablePage for HomePage {
    fn title(&self) -> String {
        "Home".into()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let shortest_side = min(area.width, 2 * area.height);
        let [inner_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(shortest_side)])
            .flex(Flex::Center)
            .areas(area);
        let [draw_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(2 * shortest_side)])
            .flex(Flex::Center)
            .areas(inner_area);

        let c = Canvas::default()
            .block(Block::bordered())
            .marker(Marker::Braille)
            .paint(|ctx| {
                render_d20(ctx, 10.0, self.color);
            })
            .x_bounds([-20.0, 20.0])
            .y_bounds([-20.0, 20.0]);

        frame.render_widget(c, draw_area);
    }

    fn handle_and_transact(&mut self, event: Event) -> Option<Event> {
        Some(event)
    }
}

/// Renders a D20 to the given ctx at (0, 0)
/// Thanks to https://www.reddit.com/r/DnD/comments/go75gv/oc_flat_d20_sides_and_angles_for_home_projects/
/// for all the angles and lengths
pub fn render_d20(ctx: &mut Context, radius: f64, color: Color) {
    let A = radius * 1.0_f64;
    let B = radius * 0.93418_f64;
    let C = radius * 0.8165_f64;
    let D = radius * 0.35683_f64;

    let a = 97.761_f64.to_radians();
    let b = 75.522_f64.to_radians();
    let c = 60.0_f64.to_radians();
    let d = 52.239_f64.to_radians();
    let e = 22.239_f64.to_radians();

    let H = 3.0_f64.sqrt() / 2.0 * A;

    let top = (0.0_f64, H / 2.0 + D);
    let left_upper = (top.0 - B * c.sin(), top.1 - B * c.cos());
    let right_upper = (top.0 + B * c.sin(), top.1 - B * c.cos());
    let left_lower = (left_upper.0, left_upper.1 - B);
    let right_lower = (right_upper.0, right_upper.1 - B);
    let bottom = (0.0, left_lower.1 + B * (2.0 * c).cos());

    let tri_top = (0.0, H / 2.0);
    let tri_left = (-A / 2.0, -H / 2.0);
    let tri_right = (A / 2.0, -H / 2.0);

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
        let line = Line::new(e.0 .0, e.0 .1, e.1 .0, e.1 .1, color);
        ctx.draw(&line);
    }
}
