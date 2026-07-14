use bevy_app::{App, Plugin, Startup, Update};
use bevy_ecs::{
    message::MessageWriter,
    schedule::{
        common_conditions::{resource_exists, resource_exists_and_changed},
        {IntoScheduleConfigs, SystemCondition},
    },
    system::{Commands, Res, ResMut},
};
use bevy_fluent::prelude::*;

use crate::{
    DEFAULT_LANG_ID,
    loading::LocalizationAssetLoadingPlugin,
    prelude::{DefaultLocale, LocaleChanged, SupportedLocales},
    resources::LocalizationFolderHandle,
};

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LocalizationAssetLoadingPlugin, FluentPlugin));

        #[cfg(feature = "text")]
        app.add_plugins(crate::localized_text::LocalizedTextPlugin);

        #[cfg(feature = "text_2d")]
        app.add_plugins(crate::localized_text_2d::LocalizedText2dPlugin);

        #[cfg(feature = "text_span")]
        app.add_plugins(crate::localized_text_span::LocalizedTextSpanPlugin);

        app.add_message::<LocaleChanged>();

        app.init_resource::<DefaultLocale>();
        app.init_resource::<SupportedLocales>();

        app.add_systems(Startup, set_default_locale);

        app.add_systems(
            Update,
            on_locale_changed.run_if(
                resource_exists::<Localization>.and_then(resource_exists_and_changed::<Locale>),
            ),
        );
    }
}

fn set_default_locale(
    mut commands: Commands,
    mut supported_locales: ResMut<SupportedLocales>,
    default_locale: Res<DefaultLocale>,
) {
    if !supported_locales.contains(&DEFAULT_LANG_ID) {
        supported_locales.0.push(DEFAULT_LANG_ID);
    }

    if !supported_locales.contains(&default_locale) {
        supported_locales.0.push(default_locale.clone());
    }

    commands.insert_resource(Locale::new(default_locale.clone()).with_default(DEFAULT_LANG_ID));
}

fn on_locale_changed(
    mut commands: Commands,
    mut message_writer: MessageWriter<LocaleChanged>,
    localization_builder: LocalizationBuilder,
    folder_handle: Res<LocalizationFolderHandle>,
) {
    commands.insert_resource(localization_builder.build(&folder_handle));
    commands.trigger(LocaleChanged);
    message_writer.write_default();
}
