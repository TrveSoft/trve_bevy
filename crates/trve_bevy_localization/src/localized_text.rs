use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    component::Component,
    lifecycle::{HookContext, Insert},
    observer::On,
    query::{Changed, QueryFilter, With},
    schedule::{
        IntoScheduleConfigs,
        common_conditions::{not, on_message},
    },
    system::Query,
    world::DeferredWorld,
};
use bevy_ui::widget::Text;

use crate::prelude::{LocaleChanged, LocalizationManager};

pub(crate) struct LocalizedTextPlugin;

impl Plugin for LocalizedTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                update_localized_text::<()>.run_if(on_message::<LocaleChanged>),
                update_localized_text::<(With<LocalizedText>, Changed<LocalizedText>)>
                    .run_if(not(on_message::<LocaleChanged>)),
            )
                .chain(),
        );
        app.add_observer(on_insert_localized_text);
    }
}

/// Localizes UI text by synchronizing a [`fluent_content::Request`]
/// message and a bevy [`Text`] component.
#[derive(Component, Default, Clone)]
#[component(on_remove = on_remove_localized_text)]
#[require(Text)]
pub struct LocalizedText(pub String);

impl LocalizedText {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

fn update_localized_text<F: QueryFilter>(
    mut q: Query<(&mut Text, &LocalizedText), F>,
    localization_manager: LocalizationManager,
) {
    for (mut text, localized_text) in &mut q {
        text.0 = localization_manager.content(&localized_text.0);
    }
}

fn on_insert_localized_text(
    on: On<Insert, LocalizedText>,
    mut q: Query<(&mut Text, &LocalizedText)>,
    localization_manager: LocalizationManager,
) {
    if let Ok((mut text, localized_text)) = q.get_mut(on.entity) {
        text.0 = localization_manager.content(&localized_text.0);
    }
}

fn on_remove_localized_text(mut world: DeferredWorld, context: HookContext) {
    world.commands().entity(context.entity).try_remove::<Text>();
}
