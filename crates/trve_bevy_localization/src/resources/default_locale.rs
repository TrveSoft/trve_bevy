use core::str::FromStr;

use bevy_derive::Deref;
use bevy_ecs::resource::Resource;
use unic_langid::LanguageIdentifier;

use crate::DEFAULT_LANG_ID;

/// User-provided default locale.
/// By default, `en-US` will be used if no [`DefaultLocale`] is set by the user.
#[derive(Resource, Deref)]
pub struct DefaultLocale(LanguageIdentifier);

impl DefaultLocale {
    pub fn new(lang_id: LanguageIdentifier) -> Self {
        Self(lang_id)
    }
}

impl Default for DefaultLocale {
    fn default() -> Self {
        let sys_locale = sys_locale::get_locale().unwrap_or_else(|| DEFAULT_LANG_ID.to_string());
        let lang_id = LanguageIdentifier::from_str(&sys_locale).unwrap_or(DEFAULT_LANG_ID);

        Self(lang_id)
    }
}
