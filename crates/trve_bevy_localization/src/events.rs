use bevy_ecs::{event::Event, message::Message};

/// Event triggered whenever the currently selected [`Locale`] changes.
///
/// It derives both [`Event`] and [`Message`] so it can be listened from
/// observers or read from systems.
#[derive(Event, Message, Default)]
pub struct LocaleChanged;

#[derive(Message, Default)]
pub(crate) struct LocalizationAssetsFinishedLoading;
