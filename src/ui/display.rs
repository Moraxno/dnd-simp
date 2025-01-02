pub trait AsRatatuiSpan {
    fn as_span(&self) -> ratatui::text::Span;
}

pub trait HasCostExpression {
    fn price_expr(&self) -> String;
}