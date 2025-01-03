pub enum I18nPhrase {
    Roll,
    Price
}

pub trait I18ner {
    fn i18n(&self, phrase: I18nPhrase) -> String;
}


pub struct EngNerdI18n {}

impl I18ner for EngNerdI18n {
    fn i18n(&self, phrase: I18nPhrase) -> String {
        match phrase {
            I18nPhrase::Price => "\u{ede8}".into(),
            I18nPhrase::Roll => "\u{e270}".into(),
        }
    }
}