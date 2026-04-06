use core::borrow::Borrow;

use bevy_ecs::system::{Res, ResMut, SystemParam};
use bevy_fluent::{Locale, Localization};
use fluent::FluentArgs;
use fluent_content::{Content, Request};

use crate::prelude::SupportedLocales;

#[derive(SystemParam)]
pub struct LocalizationManager<'w> {
    locale: ResMut<'w, Locale>,
    locales: Res<'w, SupportedLocales>,
    localization: Option<Res<'w, Localization>>,
}

impl LocalizationManager<'_> {
    pub fn content<'a, T, U>(&self, request: T) -> String
    where
        T: Copy + Into<Request<'a, U>>,
        U: Borrow<FluentArgs<'a>>,
    {
        match &self.localization {
            Some(localization) => localization.content(request).unwrap_or_else(|| {
                bevy_log::warn!(
                    "Unable to retrieve the requested localized content. Returning an empty String."
                );
                String::default()
            }),
            None => {
                bevy_log::warn!("Localization assets were not loaded. Returning an empty String.");
                String::default()
            }
        }
    }

    pub fn current_locale_string(&self) -> String {
        self.locale.requested.to_string()
    }

    pub fn locale(&self) -> &Locale {
        &self.locale
    }

    pub fn next(&mut self) {
        if let Some(index) = self.locales.index(&self.locale.requested) {
            let next_index = index.saturating_add(1).min(self.locales.len() - 1);
            self.set_requested_locale(next_index);
        }
    }

    pub fn previous(&mut self) {
        if let Some(index) = self.locales.index(&self.locale.requested) {
            let prev_index = index.saturating_sub(1);
            self.set_requested_locale(prev_index);
        }
    }

    pub fn supported_locales(&self) -> &SupportedLocales {
        &self.locales
    }

    fn set_requested_locale(&mut self, index: usize) {
        if self.locale.requested != self.locales[index] {
            self.locale.requested = self.locales[index].clone();
        }
    }
}
