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

use bevy::prelude::*;

use crate::{ATLAS_FRAMES, WINDOW_SIZE};

/// Contains metadata relating to an atlased texture.
#[derive(Clone, Debug, PartialEq, Eq, Resource)]
pub struct TextureMetadata {
    /// The handle to the texture's image.
    pub image_handle: Handle<Image>,
    /// The handle to the texture's atlas layout.
    pub layout_handle: Handle<TextureAtlasLayout>,
    /// The size of the image.
    pub size: UVec2,
}

impl TextureMetadata {
    /// Returns the size of a single frame.
    pub const fn frame_size(&self) -> UVec2 {
        UVec2::new(self.size.x / ATLAS_FRAMES, self.size.y)
    }

    /// Returns the calculated sprite scale.
    #[inline]
    pub fn sprite_scale(&self) -> Vec2 {
        Vec2::splat(WINDOW_SIZE) / self.frame_size().as_vec2()
    }
}

/// Contains the properties of the current display.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Resource)]
pub struct DisplayProperties {
    /// The display's position.
    pub position: IVec2,
    /// The display's resolution.
    pub resolution: UVec2,
}

impl DisplayProperties {
    /// Returns the smallest possible position that is contained within this display.
    #[inline]
    pub const fn minimum_position(&self) -> IVec2 {
        self.position
    }

    /// Returns the largest possible position that is contained within this display.
    #[inline]
    pub const fn maximum_position(&self) -> IVec2 {
        self.minimum_position().saturating_add_unsigned(self.resolution)
    }

    /// Returns the position at the center of this display.
    #[inline]
    pub const fn center_position(&self) -> IVec2 {
        self.minimum_position().saturating_add_unsigned(self.resolution.saturating_div(UVec2::splat(2)))
    }

    /// Returns `true` if this display contains the given position.
    pub const fn contains(&self, position: IVec2) -> bool {
        self.minimum_position().x < position.x
            && self.maximum_position().x > position.x
            && self.minimum_position().y < position.y
            && self.maximum_position().y > position.y
    }
}
