mod events;
mod loading;
mod localization_manager;
#[cfg(feature = "text")]
mod localized_text;
#[cfg(feature = "text_2d")]
mod localized_text_2d;
#[cfg(feature = "text_span")]
mod localized_text_span;
mod plugin;
mod resources;
mod run_conditions;

const DEFAULT_LANG_ID: unic_langid::LanguageIdentifier = unic_langid::langid!("en-US");

pub mod prelude {
    use super::*;

    pub use events::LocaleChanged;
    pub use localization_manager::LocalizationManager;
    #[cfg(feature = "text")]
    pub use localized_text::LocalizedText;
    #[cfg(feature = "text_2d")]
    pub use localized_text_2d::LocalizedText2d;
    #[cfg(feature = "text_span")]
    pub use localized_text_span::LocalizedTextSpan;
    pub use plugin::LocalizationPlugin;
    pub use resources::{
        DefaultLocale, LocalizationAssetFolder, LocalizationAssetsLoadState, SupportedLocales,
    };
    pub use run_conditions::{
        localization_assets_loaded, localization_assets_loading, localization_assets_loading_failed,
    };
}

pub use unic_langid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_lang_id_to_string() {
        assert_eq!(DEFAULT_LANG_ID.to_string(), String::from("en-US"));
    }
}
