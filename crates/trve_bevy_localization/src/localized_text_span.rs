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
use bevy_text::TextSpan;

use crate::prelude::{LocaleChanged, LocalizationManager};

pub(crate) struct LocalizedTextSpanPlugin;

impl Plugin for LocalizedTextSpanPlugin {
    fn build(&self, app: &mut App) {
        type Filter = (With<LocalizedTextSpan>, Changed<LocalizedTextSpan>);

        app.add_systems(
            PostUpdate,
            (
                update_localized_text_spans::<()>.run_if(on_message::<LocaleChanged>),
                update_localized_text_spans::<Filter>.run_if(not(on_message::<LocaleChanged>)),
            )
                .run_if(any_with_component::<LocalizedTextSpan>)
                .chain(),
        );
        app.add_observer(on_insert_localized_text_span);
    }
}

/// Localizes text spans by synchronizing a [`fluent_content::Request`]
/// message and a bevy [`TextSpan`] component.
#[derive(Component, Default, Clone)]
#[component(on_remove = on_remove_localized_text_span)]
#[require(TextSpan)]
pub struct LocalizedTextSpan(pub String);

impl LocalizedTextSpan {
    pub fn new(text: impl Into<String>) -> Self {
        Self(text.into())
    }
}

fn update_localized_text_spans<F: QueryFilter>(
    mut q: Query<(&mut TextSpan, &LocalizedTextSpan), F>,
    localization_manager: LocalizationManager,
) {
    for (mut text, localized_text_span) in &mut q {
        text.0 = localization_manager.content(&localized_text_span.0);
    }
}

fn on_insert_localized_text_span(
    on: On<Insert, LocalizedTextSpan>,
    mut q: Query<(&mut TextSpan, &LocalizedTextSpan)>,
    localization_manager: LocalizationManager,
) {
    if let Ok((mut text, localized_text_span)) = q.get_mut(on.entity) {
        text.0 = localization_manager.content(&localized_text_span.0);
    }
}

fn on_remove_localized_text_span(mut world: DeferredWorld, context: HookContext) {
    world
        .commands()
        .entity(context.entity)
        .try_remove::<TextSpan>();
}
