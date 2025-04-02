use bevy::prelude::*;
use bevy::image::ImageSampler;
use bevy::sprite::Anchor;

// Driving concept component
#[derive(Component)]
struct Player {
    last_action: LastAction,
}

// Tracks the last movement direction
#[derive(Default, PartialEq)]
enum LastAction {
    #[default]
    None,
    WalkUp,
    WalkDown,
    WalkSideForward,
    WalkSideBack,
}

// Manages the current idle animation state
#[derive(Component)]
struct IdleState {
    current: IdleAnimation,
}

#[derive(Default, PartialEq, Clone, Copy)]
enum IdleAnimation {
    #[default]
    FacingUp,
    FacingDown,
    FacingSideFront,
    FacingSideBack,
}

// Stores animation data for the current state
#[derive(Component)]
struct AnimationPlayer {
    layout_handle: Handle<TextureAtlasLayout>,
    texture_handle: Handle<Image>,
    frame_count: usize,
    current_frame: usize,
    frame_time: f32,
    timer: f32,
    flip_x: bool,
}

#[derive(Component)]
struct AtlasIndex(usize);

#[derive(Resource)]
struct AnimationHandles {
    back_layout: Handle<TextureAtlasLayout>,
    back_texture: Handle<Image>,
    front_layout: Handle<TextureAtlasLayout>,
    front_texture: Handle<Image>,
    side_front_layout: Handle<TextureAtlasLayout>,
    side_front_texture: Handle<Image>,
    side_back_layout: Handle<TextureAtlasLayout>,
    side_back_texture: Handle<Image>,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        Camera {
            order: 0,
            ..default()
        },
        Camera2d {},
        Transform::from_xyz(0.0, 0.0, 1000.0),
        GlobalTransform::default(),
        Visibility::Visible,
    ));

    // Load textures and create atlas layouts (10x upscale, 1px transparent border on each side, no padding between frames)
    let back_texture = asset_server.load("animations/player/posadas_idle_back1.png");
    let back_layout = TextureAtlasLayout::from_grid(UVec2::new(200, 240), 3, 1, None, None); // No padding between frames
    let back_layout_handle = texture_atlas_layouts.add(back_layout);

    let front_texture = asset_server.load("animations/player/posadas_idle_front1.png");
    let front_layout = TextureAtlasLayout::from_grid(UVec2::new(200, 250), 3, 1, None, None);
    let front_layout_handle = texture_atlas_layouts.add(front_layout);

    let side_front_texture = asset_server.load("animations/player/posadas_idle_side1.png");
    let side_front_layout = TextureAtlasLayout::from_grid(UVec2::new(190, 250), 4, 1, None, None);
    let side_front_layout_handle = texture_atlas_layouts.add(side_front_layout);

    let side_back_texture = asset_server.load("animations/player/posadas_idle_back_side1.png");
    let side_back_layout = TextureAtlasLayout::from_grid(UVec2::new(200, 230), 4, 1, None, None);
    let side_back_layout_handle = texture_atlas_layouts.add(side_back_layout);

    // Set Nearest filtering
    if let Some(image) = images.get_mut(&back_texture) {
        image.sampler = ImageSampler::nearest();
    }
    if let Some(image) = images.get_mut(&front_texture) {
        image.sampler = ImageSampler::nearest();
    }
    if let Some(image) = images.get_mut(&side_front_texture) {
        image.sampler = ImageSampler::nearest();
    }
    if let Some(image) = images.get_mut(&side_back_texture) {
        image.sampler = ImageSampler::nearest();
    }

    commands.insert_resource(AnimationHandles {
        back_layout: back_layout_handle.clone(),
        back_texture: back_texture.clone(),
        front_layout: front_layout_handle.clone(),
        front_texture: front_texture.clone(),
        side_front_layout: side_front_layout_handle.clone(),
        side_front_texture: side_front_texture.clone(),
        side_back_layout: side_back_layout_handle.clone(),
        side_back_texture: side_back_texture.clone(),
    });

    commands.spawn((
        Player { last_action: LastAction::None },
        IdleState { current: IdleAnimation::FacingUp },
        AnimationPlayer {
            layout_handle: back_layout_handle.clone(),
            texture_handle: back_texture.clone(),
            frame_count: 3,
            current_frame: 0,
            frame_time: 0.5,
            timer: 0.0,
            flip_x: false,
        },
        Sprite {
            custom_size: Some(Vec2::new(54.0, 72.0)),
            flip_x: false,
            anchor: Anchor::Center,
            image: back_texture,
            rect: Some(Rect::new(0.0, 0.0, 200.0, 240.0)),
            ..default()
        },
        AtlasIndex(0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::Visible,
    ));
}

fn handle_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        let speed = 200.0;
        let mut direction = Vec2::ZERO;

        if keyboard.pressed(KeyCode::KeyW) { direction.y += 1.0; }
        if keyboard.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
        if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }
        if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }

        if direction != Vec2::ZERO {
            direction = direction.normalize();
            transform.translation += (direction * speed * time.delta_secs()).extend(0.0);

            if direction.y > 0.5 {
                player.last_action = LastAction::WalkUp;
            } else if direction.y < -0.5 {
                if direction.x.abs() > 0.5 { player.last_action = LastAction::WalkSideBack; }
                else { player.last_action = LastAction::WalkDown; }
            } else if direction.x.abs() > 0.5 {
                player.last_action = LastAction::WalkSideForward;
            }
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
        &Transform,
    )>,
    keyboard: Res<ButtonInput<KeyCode>>,
    handles: Res<AnimationHandles>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
) {
    for (player, mut idle_state, mut animation_player, mut sprite, mut atlas_index, transform) in query.iter_mut() {
        let new_state = match player.last_action {
            LastAction::WalkUp => IdleAnimation::FacingUp,
            LastAction::WalkDown => IdleAnimation::FacingDown,
            LastAction::WalkSideForward => IdleAnimation::FacingSideFront,
            LastAction::WalkSideBack => IdleAnimation::FacingSideBack,
            LastAction::None => idle_state.current,
        };

        if new_state != idle_state.current {
            idle_state.current = new_state;
            animation_player.current_frame = 0;
            animation_player.timer = 0.0;

            match new_state {
                IdleAnimation::FacingUp => {
                    animation_player.layout_handle = handles.back_layout.clone();
                    sprite.image = handles.back_texture.clone();
                    animation_player.texture_handle = handles.back_texture.clone();
                    animation_player.frame_count = 3;
                    animation_player.flip_x = false;
                    sprite.custom_size = Some(Vec2::new(54.0, 72.0));
                    sprite.anchor = Anchor::Center;
                }
                IdleAnimation::FacingDown => {
                    animation_player.layout_handle = handles.front_layout.clone();
                    sprite.image = handles.front_texture.clone();
                    animation_player.texture_handle = handles.front_texture.clone();
                    animation_player.frame_count = 3;
                    animation_player.flip_x = false;
                    sprite.custom_size = Some(Vec2::new(54.0, 75.0));
                    sprite.anchor = Anchor::Center;
                }
                IdleAnimation::FacingSideFront => {
                    animation_player.layout_handle = handles.side_front_layout.clone();
                    sprite.image = handles.side_front_texture.clone();
                    animation_player.texture_handle = handles.side_front_texture.clone();
                    animation_player.frame_count = 4;
                    animation_player.flip_x = keyboard.pressed(KeyCode::KeyA) && !keyboard.pressed(KeyCode::KeyD);
                    sprite.custom_size = Some(Vec2::new(51.0, 75.0));
                    sprite.anchor = Anchor::Center;
                }
                IdleAnimation::FacingSideBack => {
                    animation_player.layout_handle = handles.side_back_layout.clone();
                    sprite.image = handles.side_back_texture.clone();
                    animation_player.texture_handle = handles.side_back_texture.clone();
                    animation_player.frame_count = 4;
                    animation_player.flip_x = keyboard.pressed(KeyCode::KeyA) && !keyboard.pressed(KeyCode::KeyD);
                    sprite.custom_size = Some(Vec2::new(54.0, 69.0));
                    sprite.anchor = Anchor::Center;
                }
            }
            atlas_index.0 = 0;
            sprite.flip_x = animation_player.flip_x;

            if let Some(atlas) = texture_atlases.get(&animation_player.layout_handle) {
                if let Some(urect) = atlas.textures.get(atlas_index.0) {
                    sprite.rect = Some(Rect::new(
                        urect.min.x as f32,
                        urect.min.y as f32,
                        urect.max.x as f32,
                        urect.max.y as f32,
                    ));
                    println!("Update Animation - Frame {} Rect: {:?}", atlas_index.0, sprite.rect);
                }
            }
        }
    }
}

fn play_animations(
    mut query: Query<(&mut AnimationPlayer, &mut Sprite, &mut AtlasIndex)>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
) {
    for (mut animation_player, mut sprite, mut atlas_index) in query.iter_mut() {
        animation_player.timer += time.delta_secs();
        if animation_player.timer >= animation_player.frame_time {
            animation_player.timer -= animation_player.frame_time;
            animation_player.current_frame = (animation_player.current_frame + 1) % animation_player.frame_count;
            atlas_index.0 = animation_player.current_frame;

            if let Some(atlas) = texture_atlases.get(&animation_player.layout_handle) {
                if let Some(urect) = atlas.textures.get(atlas_index.0) {
                    sprite.rect = Some(Rect::new(
                        urect.min.x as f32,
                        urect.min.y as f32,
                        urect.max.x as f32,
                        urect.max.y as f32,
                    ));
                    println!("Play Animations - Frame {} Rect: {:?}", atlas_index.0, sprite.rect);
                }
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_movement, update_animation_state, play_animations).chain())
        .run();
}