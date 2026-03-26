use bevy_derive::Deref;
use bevy_ecs::resource::Resource;
use unic_langid::LanguageIdentifier;

use crate::DEFAULT_LANG_ID;

/// Holds all the supported locales as [`LanguageIdentifier`]s.
/// The `en-US` locale will always be included by default.
#[derive(Resource, Deref)]
pub struct SupportedLocales(pub(crate) Vec<LanguageIdentifier>);

impl SupportedLocales {
    pub fn new(locales: impl Into<Vec<LanguageIdentifier>>) -> Self {
        Self(locales.into())
    }

    pub fn index(&self, locale: &LanguageIdentifier) -> Option<usize> {
        self.iter().position(|id| id == locale)
    }
}

impl Default for SupportedLocales {
    fn default() -> Self {
        Self(vec![DEFAULT_LANG_ID])
    }
}
