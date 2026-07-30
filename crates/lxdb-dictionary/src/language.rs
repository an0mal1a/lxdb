/// Static provider identifiers for one language pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LanguageProfile {
    pub iso_639_1: &'static str,
    pub iso_639_3: &'static str,
    pub display_name: &'static str,
    pub kaikki_language_name: &'static str,
    pub wiktionary_code: &'static str,
    pub wordnet_code: Option<&'static str>,
    pub frequency_code: Option<&'static str>,
    pub hunspell_locale: Option<&'static str>,
}

pub const LANGUAGES: &[LanguageProfile] = &[
    LanguageProfile {
        iso_639_1: "es",
        iso_639_3: "spa",
        display_name: "Español",
        kaikki_language_name: "Spanish",
        wiktionary_code: "es",
        wordnet_code: Some("spa"),
        frequency_code: Some("es"),
        hunspell_locale: Some("es_ES"),
    },
    LanguageProfile {
        iso_639_1: "en",
        iso_639_3: "eng",
        display_name: "English",
        kaikki_language_name: "English",
        wiktionary_code: "en",
        wordnet_code: Some("eng"),
        frequency_code: Some("en"),
        hunspell_locale: Some("en_US"),
    },
];

pub fn find_language(code: &str) -> Option<&'static LanguageProfile> {
    LANGUAGES.iter().find(|language| language.iso_639_1 == code)
}
