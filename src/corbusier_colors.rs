/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Louis Mayencourt
*/

use bevy::prelude::*;
use rand::seq::SliceRandom;

pub const COLOR_BLUE: Color = Color::srgb(132.0 / 255.0, 166.0 / 255.0, 199.0 / 255.0);
pub const COLOR_LIGHT_BLUE: Color = Color::srgb(175.0 / 255.0, 188.0 / 255.0, 198.0 / 255.0);
pub const COLOR_GREEN: Color = Color::srgb(175.0 / 255.0, 192.0 / 255.0, 130.0 / 255.0);
pub const COLOR_RED: Color = Color::srgb(159.0 / 255.0, 75.0 / 255.0, 63.0 / 255.0);
pub const COLOR_WHITE: Color = Color::srgb(233.0 / 255.0, 228.0 / 255.0, 217.0 / 255.0);
pub const COLOR_BLACK: Color = Color::srgb(58.0 / 255.0, 59.0 / 255.0, 59.0 / 255.0);

pub fn random() -> Color {
    let val = [COLOR_BLUE,
    COLOR_LIGHT_BLUE,
    COLOR_GREEN,
    COLOR_RED,
    COLOR_WHITE,
    COLOR_BLACK].choose(&mut rand::thread_rng());

    *val.unwrap()
}