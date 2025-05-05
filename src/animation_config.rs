use bevy::prelude::*;
use bevy::image::ImageSampler;
use std::collections::HashMap;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AnimationType {
    #[default]
    PlayerIdleForward,
    PlayerIdleBack,
    PlayerIdleRight,
    PlayerIdleRightBack,
    PlayerWalkForward,
    PlayerWalkBack,
    PlayerWalkRight,
    PlayerWalkRightBack,
}

#[derive(Resource)]
pub struct AnimationRegistry {
    pub animations: HashMap<AnimationType, AnimationConfig>,
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
            AnimationType::PlayerIdleBack,
            "animations/player/posadas_idle_back.png",
            UVec2::new(200, 240), //images size
            3,
            Vec2::new(54.0, 72.0), //desired in game size
            0.5,
        ),
        (
            AnimationType::PlayerIdleForward,
            "animations/player/posadas_idle_front.png",
            UVec2::new(200, 250),
            3,
            Vec2::new(54.0, 75.0),
            0.5,
        ),
        (
            AnimationType::PlayerIdleRight,
            "animations/player/posadas_idle_side.png",
            UVec2::new(190, 250),
            4,
            Vec2::new(51.0, 75.0),
            0.5,
        ),
        (
            AnimationType::PlayerIdleRightBack,
            "animations/player/posadas_idle_back_side.png",
            UVec2::new(200, 230),
            4,
            Vec2::new(54.0, 69.0),
            0.5,
        ),
        (
            AnimationType::PlayerWalkForward,
            "animations/player/posadas_walk_front.png",
            UVec2::new(220, 250),
            4,
            Vec2::new(66.0, 75.0),
            0.35,
        ),
        (
            AnimationType::PlayerWalkRight,
            "animations/player/posadas_walk_right.png",
            UVec2::new(180, 270),
            4,
            Vec2::new(54.0, 81.0),
            0.35,
        ),
    ];

    let mut registry = AnimationRegistry {animations: HashMap::new(),};

    for (state, path, frame_size, frame_count, size, frame_time) in configs {
        let texture_handle = asset_server.load(path);
        let layout = TextureAtlasLayout::from_grid(frame_size, frame_count, 1, None, None);
        let layout_handle = texture_atlases.add(layout);

        if let Some(image)=images.get_mut(&texture_handle)  {image.sampler = ImageSampler::nearest();} else { 
            eprintln!("Error: texture {:?} not found", texture_handle) 
        };
        registry.animations.insert(
            state,
            AnimationConfig {
                layout_handle,
                texture_handle: texture_handle.clone(),
                frame_count,
                frame_time,
                size,
            },
        );
    }

    commands.insert_resource(registry);
}


pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_animation_registry);
    }
}