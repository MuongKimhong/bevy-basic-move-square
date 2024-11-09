use bevy::prelude::*;
use bevy::window::{Window, WindowResolution};

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

fn set_window_resolution() -> WindowResolution {
    WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT).with_scale_factor_override(1.0)
}

fn set_window_size() -> Window {
    Window {
        resolution: set_window_resolution(),
        ..default()
    }
}

pub fn set_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(set_window_size()),
        ..default()
    }
}
