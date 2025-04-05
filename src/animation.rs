use bevy::prelude::*;
use bevy::image::ImageSampler;
use std::collections::HashMap;

use crate::player::{Player, LastAction};

#[derive(Component)]
pub struct CurrentAnimationState {
    pub current: IdleAnimation,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IdleAnimation {
    #[default]
    Forward,
    Back,
    RSide,
    RSideBack,
}

#[derive(Component)]
pub struct AnimationPlayer {
    pub config: AnimationConfig,
    pub current_frame: u32,
    pub timer: f32,
    pub flip_x: bool,
}

#[derive(Component)]
pub struct AtlasIndex(pub u32);

#[derive(Resource)]
pub struct AnimationRegistry {
    animations: HashMap<IdleAnimation, AnimationConfig>,
}

#[derive(Clone)]
pub struct AnimationConfig {
    pub layout_handle: Handle<TextureAtlasLayout>,
    pub texture_handle: Handle<Image>,
    pub frame_count: u32,
    pub frame_time: f32,
    pub size: Vec2,
}

fn setup_animation_registry(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut images: ResMut<Assets<Image>>,
) {
    let configs = [
        (
            IdleAnimation::Back,
            "animations/player/posadas_idle_back1.png",
            UVec2::new(200, 240),
            3,
            Vec2::new(54.0, 72.0),
        ),
        (
            IdleAnimation::Forward,
            "animations/player/posadas_idle_front1.png",
            UVec2::new(200, 250),
            3,
            Vec2::new(54.0, 75.0),
        ),
        (
            IdleAnimation::RSide,
            "animations/player/posadas_idle_side1.png",
            UVec2::new(190, 250),
            4,
            Vec2::new(51.0, 75.0),
        ),
        (
            IdleAnimation::RSideBack,
            "animations/player/posadas_idle_back_side1.png",
            UVec2::new(200, 230),
            4,
            Vec2::new(54.0, 69.0),
        ),
    ];

    let mut registry = AnimationRegistry {animations: HashMap::new(),};

    for (state, path, frame_size, frame_count, size) in configs {
        let texture_handle = asset_server.load(path);
        let layout = TextureAtlasLayout::from_grid(frame_size, frame_count, 1, None, None);
        let layout_handle = texture_atlases.add(layout);

        if let Some(image) = images.get_mut(&texture_handle) {
            image.sampler = ImageSampler::nearest();
        }

        registry.animations.insert(
            state,
            AnimationConfig {
                layout_handle,
                texture_handle: texture_handle.clone(),
                frame_count,
                frame_time: 0.5,
                size,
            },
        );
    }

    commands.insert_resource(registry);
}

fn update_animation_state(
    mut query: Query<(
        &Player,
        &mut CurrentAnimationState,
        &mut AnimationPlayer,
        &mut Sprite,
        &mut AtlasIndex,
    )>,
    keyboard: Res<ButtonInput<KeyCode>>,
    registry: Res<AnimationRegistry>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
) {
    for (player, mut idle_state, mut animation_player, mut sprite, mut atlas_index) in query.iter_mut() {
        let new_state = match player.last_action {
            LastAction::WalkForward => IdleAnimation::Forward,
            LastAction::WalkBack => IdleAnimation::Back,
            LastAction::WalkSideOrForward => IdleAnimation::RSide,
            LastAction::WalkSideBack => IdleAnimation::RSideBack,
            LastAction::None => idle_state.current,
        };

        if player.last_action == LastAction::WalkSideOrForward || player.last_action == LastAction::WalkSideBack {
            sprite.flip_x = keyboard.pressed(KeyCode::KeyA) && !keyboard.pressed(KeyCode::KeyD);
            animation_player.flip_x = sprite.flip_x;
        }

        if new_state != idle_state.current {
            idle_state.current = new_state;
            animation_player.current_frame = 0;
            animation_player.timer = 0.0;

            if let Some(config) = registry.animations.get(&new_state) {
                animation_player.config = config.clone();
                sprite.image = config.texture_handle.clone();
                sprite.custom_size = Some(config.size);
                atlas_index.0 = 0;
            }
        }

        if let Some(atlas) = texture_atlases.get(&animation_player.config.layout_handle) {
            if let Some(urect) = atlas.textures.get(atlas_index.0 as usize) {
                sprite.rect = Some(Rect::new(
                    urect.min.x as f32,
                    urect.min.y as f32,
                    urect.max.x as f32,
                    urect.max.y as f32,
                ));
            }
        }
    }
}

fn play_animations(
    mut query: Query<(&mut AnimationPlayer, &mut AtlasIndex)>,
    time: Res<Time>,
) {
    for (mut animation_player, mut atlas_index) in query.iter_mut() {
        animation_player.timer += time.delta_secs();
        if animation_player.timer >= animation_player.config.frame_time {
            animation_player.timer -= animation_player.config.frame_time;
            animation_player.current_frame =
                (animation_player.current_frame + 1) % animation_player.config.frame_count;
            atlas_index.0 = animation_player.current_frame;
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_animation_registry)
            .add_systems(Update, (update_animation_state, play_animations).chain());
    }
}