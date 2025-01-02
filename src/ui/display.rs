pub trait AsRatatuiSpan {
    fn as_span(&self) -> ratatui::text::Span;
}