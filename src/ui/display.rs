pub trait AsRatatuiSpan {
    fn as_span(&self) -> ratatui::text::Span;
}


// @todo maybe move this into its own file?
impl AsRatatuiSpan for Rarity {
    fn as_span(&self) -> Span {
        let base_span = Span::raw(self.to_string());
        match self {
            Rarity::Common => base_span.style(Style::default().gray().italic()),
            Rarity::Uncommon => base_span.style(Style::default().white().italic()),
            Rarity::Rare => base_span.style(Style::default().green().italic()),
            Rarity::VeryRare => base_span.style(Style::default().magenta().italic()),
            Rarity::Legendary => base_span.style(Style::default().red().italic()),
            Rarity::Artifact => base_span.style(Style::default().red().underlined().italic()),
        }
    }
}