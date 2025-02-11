// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright © 2025 Jaxydog
//
// This file is part of Desktop Cube Baby.
//
// Desktop Cube Baby is free software: you can redistribute it and/or modify it under the terms of the GNU General
// Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Desktop Cube Baby is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the
// implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with Desktop Cube Baby. If not,
// see <https://www.gnu.org/licenses/>.

use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use bevy::state::state::{FreelyMutableState, States};

/// A trait marker for types intended to be used for loading states.
pub trait LoadingTypeMarker: Send + Sync + 'static {}

/// The type marker used for the texture loading state.
pub enum TextureLoadingMarker {}

impl LoadingTypeMarker for TextureLoadingMarker {}

/// The type marker used for the display loading state.
pub enum DisplayLoadingMarker {}

impl LoadingTypeMarker for DisplayLoadingMarker {}

/// The type marker used for the application loading state.
pub enum ApplicationLoadingMarker {}

impl LoadingTypeMarker for ApplicationLoadingMarker {}

/// A typed loading state.
#[repr(transparent)]
pub struct LoadingState<T: LoadingTypeMarker> {
    /// The inner loading state.
    inner: GenericLoadingState,
    /// The associated type marker.
    marker: PhantomData<T>,
}

impl<T: LoadingTypeMarker> LoadingState<T> {
    /// Creates a new [`LoadingState<T>`] that is set to [`Loading`].
    ///
    /// [`Loading`]: GenericLoadingState::Loading
    #[inline]
    pub const fn loading() -> Self {
        Self::new(GenericLoadingState::Loading)
    }

    /// Creates a new [`LoadingState<T>`] that is set to [`Finished`].
    ///
    ///
    /// [`Finished`]: GenericLoadingState::Finished
    #[inline]
    pub const fn finished() -> Self {
        Self::new(GenericLoadingState::Finished)
    }

    /// Creates a new [`LoadingState<T>`].
    #[inline]
    const fn new(inner: GenericLoadingState) -> Self {
        Self { inner, marker: PhantomData }
    }

    /// Returns `true` if the typed loading state is [`Loading`].
    ///
    /// [`Loading`]: GenericLoadingState::Loading
    #[inline]
    #[must_use]
    pub const fn is_loading(&self) -> bool {
        self.inner.is_loading()
    }

    /// Returns `true` if the typed loading state is [`Finished`].
    ///
    /// [`Finished`]: GenericLoadingState::Finished
    #[inline]
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        self.inner.is_finished()
    }
}

impl<T: LoadingTypeMarker> Clone for LoadingState<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: LoadingTypeMarker> Copy for LoadingState<T> {}

impl<T: LoadingTypeMarker> Debug for LoadingState<T> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LoadingState").field(&self.inner).finish()
    }
}
impl<T: LoadingTypeMarker> Default for LoadingState<T> {
    #[inline]
    fn default() -> Self {
        Self::new(GenericLoadingState::default())
    }
}

impl<T: LoadingTypeMarker> Hash for LoadingState<T> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
        self.marker.hash(state);
    }
}

impl<T: LoadingTypeMarker> PartialEq for LoadingState<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: LoadingTypeMarker> Eq for LoadingState<T> {}

impl<T: LoadingTypeMarker> States for LoadingState<T> {}

impl<T: LoadingTypeMarker> FreelyMutableState for LoadingState<T> {}

/// A generic loading state.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, States)]
pub enum GenericLoadingState {
    /// Currently loading.
    Loading,
    /// Finished loading.
    Finished,
}

impl GenericLoadingState {
    /// Returns `true` if the generic loading state is [`Loading`].
    ///
    /// [`Loading`]: GenericLoadingState::Loading
    #[inline]
    #[must_use]
    pub const fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    /// Returns `true` if the generic loading state is [`Finished`].
    ///
    /// [`Finished`]: GenericLoadingState::Finished
    #[inline]
    #[must_use]
    pub const fn is_finished(&self) -> bool {
        matches!(self, Self::Finished)
    }
}

impl Default for GenericLoadingState {
    #[inline]
    fn default() -> Self {
        Self::Loading
    }
}
