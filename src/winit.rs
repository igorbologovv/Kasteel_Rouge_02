use crate::resources::WinSize;
use bevy::prelude::*;
use bevy::window::WindowResolution;

pub fn winit(width: f32, height: f32) -> WindowPlugin {
    let winsize = WinSize {
        w: width,
        h: height,
    };

    let windowplugin = WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(winsize.w, winsize.h).with_scale_factor_override(1.0),
            title: "Resizing".into(),
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    };

    windowplugin
}

