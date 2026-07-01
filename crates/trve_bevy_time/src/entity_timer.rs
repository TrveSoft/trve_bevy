use core::{marker::PhantomData, time::Duration};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    component::Component,
    query::QueryFilter,
    system::{Query, Res},
};
use bevy_time::{Time, Timer, TimerMode};

/// A generic [`Component`] wrapping a [`Timer`].
/// This allows multiple timers to be attached to
/// the same entity.
#[derive(Component, Default, Clone, Deref, DerefMut)]
pub struct EntityTimer<T: Send + Sync + 'static> {
    #[deref]
    timer: Timer,
    _marker: PhantomData<T>,
}

/// Creates a repeating [`EntityTimer`] from a seconds value.
impl<T: Send + Sync + 'static> EntityTimer<T> {
    pub fn from_seconds(secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Repeating),
            _marker: PhantomData,
        }
    }

    /// Creates an [`EntityTimer`] from a seconds value that runs once.
    pub fn from_seconds_once(secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Once),
            _marker: PhantomData,
        }
    }

    /// Sets the duration of the timer from a seconds value.
    pub fn set_duration_from_seconds(&mut self, secs: f32) {
        self.timer.set_duration(Duration::from_secs_f32(secs));
    }

    /// Acts as a system that you can run on a schedule of your choosing
    /// to tick all [`EntityTimer`]s of type `T`.
    ///
    /// If you need to filter which entities should get their timers updated,
    /// use [`Self::tick_system_with_filter()`].
    pub fn tick_system(q: Query<&mut Self>, time: Res<Time>) {
        Self::tick_system_with_filter::<()>(q, time);
    }

    /// Acts as a system that you can run on a schedule of your choosing
    /// to tick all timers of type `T` that match the provided [`QueryFilter`].
    pub fn tick_system_with_filter<F: QueryFilter>(mut q: Query<&mut Self, F>, time: Res<Time>) {
        for mut timer in &mut q {
            timer.tick(time.delta());
        }
    }
}
