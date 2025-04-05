use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::player::{Player, LastAction};
use crate::animation::{CurrentAnimationState, AnimationPlayer, AtlasIndex, AnimationConfig};

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
    let initial_texture = asset_server.load("animations/player/posadas_idle_back1.png");
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
        Sprite {
            image: initial_texture.clone(),
            custom_size: Some(initial_size),
            anchor: Anchor::Center,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player {
            last_action: LastAction::None,
        },
        CurrentAnimationState {
            current: crate::animation::IdleAnimation::Forward,
        },
        AnimationPlayer {
            config: AnimationConfig {
                layout_handle: initial_layout.clone(),
                texture_handle: initial_texture,
                frame_count: 3,
                frame_time: 0.5,
                size: initial_size,
            },
            current_frame: 0,
            timer: 0.0,
            flip_x: false,
        },
        AtlasIndex(0),
    ));
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}