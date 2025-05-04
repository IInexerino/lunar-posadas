use bevy::prelude::*;

mod animation_config;
mod window;
mod player;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                primary_window: Some(Window{
                    title: String::from("Lunar Posadas"),
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    resolution: Vec2::new(1920.0, 1080.0).into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugins(window::WindowPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(animation_config::AnimationPlugin)
        .add_plugins(setup::SetupPlugin)
        .run();
}