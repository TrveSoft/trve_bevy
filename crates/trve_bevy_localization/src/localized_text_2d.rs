use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::{
    component::Component,
    lifecycle::{HookContext, Insert},
    observer::On,
    query::{Changed, QueryFilter, With},
    schedule::{
        IntoScheduleConfigs,
        common_conditions::{any_with_component, not, on_message},
    },
    system::Query,
    world::DeferredWorld,
};
use bevy_sprite::Text2d;

use crate::prelude::{LocaleChanged, LocalizationManager};

pub(crate) struct LocalizedText2dPlugin;

impl Plugin for LocalizedText2dPlugin {
    fn build(&self, app: &mut App) {
        type Filter = (With<LocalizedText2d>, Changed<LocalizedText2d>);

        app.add_systems(
            PostUpdate,
            (
                update_localized_text_2d::<()>.run_if(on_message::<LocaleChanged>),
                update_localized_text_2d::<Filter>.run_if(not(on_message::<LocaleChanged>)),
            )
                .run_if(any_with_component::<LocalizedText2d>)
                .chain(),
        );
        app.add_observer(on_insert_localized_text_2d);
    }
}

/// Localizes 2D text by synchronizing a [`fluent_content::Request`]
/// message and a bevy [`Text2d`] component.
#[derive(Component, Default, Clone)]
#[component(on_remove = on_remove_localized_text_2d)]
#[require(Text2d)]
pub struct LocalizedText2d(pub String);

impl LocalizedText2d {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

fn update_localized_text_2d<F: QueryFilter>(
    mut q: Query<(&mut Text2d, &LocalizedText2d), F>,
    localization_manager: LocalizationManager,
) {
    for (mut text, localized_text_2d) in &mut q {
        text.0 = localization_manager.content(&localized_text_2d.0);
    }
}

fn on_insert_localized_text_2d(
    on: On<Insert, LocalizedText2d>,
    mut q: Query<(&mut Text2d, &LocalizedText2d)>,
    localization_manager: LocalizationManager,
) {
    if let Ok((mut text, localized_text_2d)) = q.get_mut(on.entity) {
        text.0 = localization_manager.content(&localized_text_2d.0);
    }
}

fn on_remove_localized_text_2d(mut world: DeferredWorld, context: HookContext) {
    world
        .commands()
        .entity(context.entity)
        .try_remove::<Text2d>();
}
