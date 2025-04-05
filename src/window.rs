use bevy::prelude::*;
use bevy::window::WindowMode;

fn change_window_mode(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    if keyboard.just_pressed(KeyCode::F11) {
        window.mode = match window.mode {
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            WindowMode::BorderlessFullscreen(MonitorSelection::Current) => WindowMode::Windowed,
            _ => WindowMode::Windowed,
        };
    }
}

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, change_window_mode);
    }
}