use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::image::ImageSampler;

// --- Components ---

#[derive(Component)]
struct Player {
    last_action: LastAction,
}

#[derive(Default, PartialEq)]
enum LastAction {
    #[default]
    None,
    WalkUp,
    WalkDown,
    WalkSideForward,
    WalkSideBack,
}

#[derive(Component)]
struct IdleState {
    current: IdleAnimation,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy)]
enum IdleAnimation {
    #[default]
    FacingUp,
    FacingDown,
    FacingSideFront,
    FacingSideBack,
}

#[derive(Component)]
struct AnimationPlayer {
    config: AnimationConfig,
    current_frame: u32,
    timer: f32,
    flip_x: bool,
}

#[derive(Component)]
struct AtlasIndex(u32);

#[derive(Resource)]
struct AnimationRegistry {
    animations: std::collections::HashMap<IdleAnimation, AnimationConfig>,
}

#[derive(Clone)]
struct AnimationConfig {
    layout_handle: Handle<TextureAtlasLayout>,
    texture_handle: Handle<Image>,
    frame_count: u32,
    frame_time: f32,
    size: Vec2,
}

// --- Systems ---

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut images: ResMut<Assets<Image>>,
) {

    let configs = [
        (
            IdleAnimation::FacingUp,
            "animations/player/posadas_idle_back1.png",
            UVec2::new(200, 240),
            3,
            Vec2::new(54.0, 72.0),
        ),
        (
            IdleAnimation::FacingDown,
            "animations/player/posadas_idle_front1.png",
            UVec2::new(200, 250),
            3,
            Vec2::new(54.0, 75.0),
        ),
        (
            IdleAnimation::FacingSideFront,
            "animations/player/posadas_idle_side1.png",
            UVec2::new(190, 250),
            4,
            Vec2::new(51.0, 75.0),
        ),
        (
            IdleAnimation::FacingSideBack,
            "animations/player/posadas_idle_back_side1.png",
            UVec2::new(200, 230),
            4,
            Vec2::new(54.0, 69.0),
        ),
    ];

    let mut registry = AnimationRegistry {
        animations: std::collections::HashMap::new(),
    };

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


    
    let initial_config = configs[0].4;
    let initial_texture = asset_server.load("animations/player/posadas_idle_back1.png");
    let initial_layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(200, 240),
        3,
        1,
        None,
        None,
    ));

    commands.insert_resource(registry);

    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    commands.spawn((
        Player {
            last_action: LastAction::None,
        },
        Sprite {
            image: initial_texture.clone(),
            custom_size: Some(initial_config),
            anchor: Anchor::Center,
            ..default()
        },
        IdleState {
            current: IdleAnimation::FacingUp,
        },
        AnimationPlayer {
            config: AnimationConfig {
                layout_handle: initial_layout.clone(),
                texture_handle: initial_texture,
                frame_count: 3,
                frame_time: 0.5,
                size: initial_config,
            },
            current_frame: 0,
            timer: 0.0,
            flip_x: false,
        },
        AtlasIndex(0),
    ));
}

fn handle_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    const SPEED: f32 = 200.0;

    for (mut player, mut transform) in query.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if keyboard.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }
        if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation += (direction * SPEED * time.delta_secs()).extend(0.0);

            player.last_action = if direction.y.abs() > direction.x.abs() {
                if direction.y > 0.0 { LastAction::WalkUp }
                else { LastAction::WalkDown }
            } else if direction.x.abs() > 0.0 {
                if direction.y > 0.0 { LastAction::WalkSideBack }
                else { LastAction::WalkSideForward }
            } else {
                LastAction::None
            };
        } else {
            player.last_action = LastAction::None;
        }
    }
}

fn update_animation_state(
    mut query: Query<(
        &Player,
        &mut IdleState,
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
            LastAction::WalkUp => IdleAnimation::FacingUp,
            LastAction::WalkDown => IdleAnimation::FacingDown,
            LastAction::WalkSideForward => IdleAnimation::FacingSideFront,
            LastAction::WalkSideBack => IdleAnimation::FacingSideBack,
            LastAction::None => idle_state.current,
        };

        if player.last_action == LastAction::WalkSideForward || player.last_action == LastAction::WalkSideBack {
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

        // Update the sprite rect based on the current atlas frame
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (handle_movement, update_animation_state, play_animations).chain(),
        )
        .run();
}