use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_enhanced_input::prelude::Actions;

use crate::player::{Player, PlayerEntity};
use crate::animation_config::{AnimationConfig, AnimationType};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn camera
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    // Initial animation config for player
    let initial_texture = asset_server.load("animations/player/posadas_idle_front.png");
    let initial_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(200, 240),
        3,
        1,
        None,
        None,
    ));
    let initial_size = Vec2::new(54.0, 72.0);

    // Spawn player entity
    commands.spawn((
        PlayerEntity {
            direction: Vec2 { x: 0.0, y: -1.0 },
            speed: 0.0,
            current_animation: AnimationType::PlayerIdleForward,
            animation_config: AnimationConfig {
                layout_handle: initial_layout,
                texture_handle: initial_texture.clone(),
                frame_count: 3,
                frame_time: 0.5,
                size: initial_size,
            },
            animation_timer: 0.0,
            current_animation_frame: 0,
            atlas_index: 0,
        },
        Sprite {
            image: initial_texture,
            custom_size: Some(initial_size),
            anchor: Anchor::Center,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Actions::<Player>::default()
    ));
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}