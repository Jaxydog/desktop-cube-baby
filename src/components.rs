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

use bevy::prelude::*;

/// Marker component for values that belong to the cube baby.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Component)]
pub struct CubeBaby;

/// Represents a delay in seconds for when the cube baby may be pushed.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Component, Deref, DerefMut)]
pub struct PushDelay(pub f64);

impl PushDelay {
    /// A delay of zero seconds.
    pub const ZERO: Self = Self(0.0);
}

/// Represents the distance traveled since the cube baby last had its sprite updated.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Component, Deref, DerefMut)]
pub struct Distance(pub f32);

impl Distance {
    /// A distance of zero pixels.
    pub const ZERO: Self = Self(0.0);
}

/// Represents a persistent position.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Component, Deref, DerefMut)]
pub struct Position(pub Vec2);

impl Position {
    /// A position of `(0, 0)`.
    pub const ZERO: Self = Self(Vec2::ZERO);

    /// Creates a new [`Position`].
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

/// Represents a persistent velocity.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    /// A velocity of zero.
    pub const ZERO: Self = Self(Vec2::ZERO);

    /// Creates a new [`Velocity`].
    #[inline]
    pub const fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}
