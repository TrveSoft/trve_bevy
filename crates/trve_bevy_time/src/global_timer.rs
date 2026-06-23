use core::{marker::PhantomData, time::Duration};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    resource::Resource,
    system::{Res, ResMut},
};
use bevy_time::{Time, Timer, TimerMode};

/// A generic [`Resource`] wrapping a [`Timer`].
#[derive(Resource, Default, Deref, DerefMut)]
pub struct GlobalTimer<T: Send + Sync + 'static> {
    #[deref]
    timer: Timer,
    _marker: PhantomData<T>,
}

/// Creates a repeating [`GlobalTimer`] from a seconds value.
impl<T: Send + Sync + 'static> GlobalTimer<T> {
    pub fn from_seconds(secs: f32) -> Self {
        Self {
            timer: Timer::from_seconds(secs, TimerMode::Repeating),
            _marker: PhantomData,
        }
    }

    /// Creates an [`GlobalTimer`] from a seconds value that runs once.
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
    /// to tick all [`GlobalTimer`]s of type `T`.
    pub fn tick_system(mut t: ResMut<Self>, time: Res<Time>) {
        t.tick(time.delta());
    }
}
