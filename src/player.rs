use std::cmp::Ordering;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use crate::animation_config::{AnimationType, AnimationConfig, AnimationRegistry};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EnhancedInputPlugin)
            .add_input_context::<Player>()
            .add_observer(binding)
            .add_systems(Update, handle_movement)
            .add_systems(Update, (set_player_animation, play_animations).chain());
    }
}








// ============================ PLAYER / PLAYER-INPUTACTION DATA CONFIGURATION ============================

#[derive(Component)]
pub struct PlayerEntity {
    pub direction: Vec2,
    pub speed: f32,
    pub current_animation: AnimationType,
    pub current_animation_frame: u32,
    pub animation_timer: f32,
    pub atlas_index: u32,
    pub animation_config: AnimationConfig,
}        

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Move;

#[derive(InputContext)]
pub struct Player;

fn binding(trigger: Trigger<Binding<Player>>, mut actions: Query<&mut Actions<Player>>) {
    let mut actions = actions.get_mut(trigger.target()).unwrap();
    actions
        .bind::<Move>()
        .to((Cardinal::wasd_keys(), Axial::left_stick()))
        .with_modifiers(DeadZone::default());
}








// ======================================== PLAYER MOVEMENT =========================================

fn handle_movement(
    mut query: Query<(&mut PlayerEntity, &mut Transform, &Actions<Player>)>,
    time: Res<Time>,
) {
    for (mut player, mut transform, actions) in query.iter_mut() {
        // Get the current value of the Move action
        let move_input = actions.action::<Move>().value().as_axis2d();

        // if there is a WASD triggered Move action, set player direction to the Vec2 output of the WASD binding()
        if move_input != Vec2::ZERO {
            player.direction = move_input;
            player.speed = 200.0;
        } // if there is no WASD triggered Move action, do not reset player.direction, set the speed to 0
        else { player.speed = 0.0; }

        // Apply movement if speed is non-zero
        if player.speed > 0.0 {
            // normalize the vector to cancel out diagonal movement inequality
            let normalized_direction = player.direction.normalize();
            /* scale speed by time elapsed, scales the transform vector by the result of the former, extends the Vec2 into a Vec3 
            - changes the translation - moves the character by the product of the previous calculation */
            transform.translation += (normalized_direction * player.speed * time.delta_secs()).extend(0.0);
        }
    }
}








// ======================================== PLAYER ANIMATION =========================================


fn set_player_animation(
    mut query: Query<(&mut PlayerEntity, &mut Sprite)>,
    animation_registry: Res<AnimationRegistry>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
) {
    for (mut player, mut sprite) in query.iter_mut() {
        let (x, y) = (player.direction.x.partial_cmp(&0.0).unwrap(), player.direction.y.partial_cmp(&0.0).unwrap());

        let (anim_idle, anim_walk) = match (x, y) {
            // Up (x ≈ 0, y > 0)
            (Ordering::Equal, Ordering::Greater) => {
                sprite.flip_x = false;
                (AnimationType::PlayerIdleBack, AnimationType::PlayerWalkBack) 
            }
            // Down (x ≈ 0, y < 0)
            (Ordering::Equal, Ordering::Less) => {
                sprite.flip_x = false;
                (AnimationType::PlayerIdleForward, AnimationType::PlayerWalkForward)
            }
            // Right (x > 0, y ≈ 0) - Down-Right (x > 0, y < 0)
            (Ordering::Greater, Ordering::Equal) | (Ordering::Greater, Ordering::Less) => {
                sprite.flip_x = false;
                (AnimationType::PlayerIdleRight, AnimationType::PlayerWalkRight)
            }
            // Left (x < 0, y ≈ 0)
            (Ordering::Less, Ordering::Equal) | (Ordering::Less, Ordering::Less) => {
                sprite.flip_x = true;
                (AnimationType::PlayerIdleRight, AnimationType::PlayerWalkRight)
            }
            // Up-Right (x > 0, y > 0)
            (Ordering::Greater, Ordering::Greater) => {
                sprite.flip_x = false;
                (AnimationType::PlayerIdleRightBack, AnimationType::PlayerWalkRightBack)
            }
            // Up-Left (x < 0, y > 0)
            (Ordering::Less, Ordering::Greater) => {
                sprite.flip_x = true;
                (AnimationType::PlayerIdleRightBack, AnimationType::PlayerWalkRightBack)
            }
            (Ordering::Equal, Ordering::Equal) => panic!("invalid direction the player is facing"), // Fallback to current state
        };
        let new_animation = 
            if player.speed > 0.0 { anim_walk } else 
            if player.speed == 0.0 { anim_idle } else { 
                panic!("invalid negative speed") 
        };

        if player.current_animation == new_animation { } else {
            player.current_animation = new_animation;
            player.current_animation_frame = 0;
            player.animation_timer = 0.0;

            if let Some(config) = animation_registry.animations.get(&player.current_animation) {
                player.animation_config = config.clone();
                sprite.image = config.texture_handle.clone();
                sprite.custom_size = Some(config.size);
                player.atlas_index = 0;
            }
        }

        if let Some(atlas) = texture_atlases.get(&player.animation_config.layout_handle) {
            if let Some(urect) = atlas.textures.get(player.atlas_index as usize) {
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
    mut query: Query<&mut PlayerEntity>,
    time: Res<Time>,
) {
    for mut player in query.iter_mut() {
        // this is a system, so it runs once every frame
        // here we add time corresponding to the time it takes to go through one frame to our player.animation timer
        player.animation_timer += time.delta_secs();
        // check if the accumulated time on the timer exceeds the amount of time after which a new frame needs to play as configured in animation_configs.frame_time
        if player.animation_timer >= player.animation_config.frame_time {
            // reset the timer -- but not fully -- only subtract the 'necessary frame time', because we are trying to carefully deal with the passage of frames and time without conflict
            player.animation_timer -= player.animation_config.frame_time;
            // changes the frame to the next frame, and loops if it reaches the end
            player.current_animation_frame = (player.current_animation_frame + 1) % player.animation_config.frame_count;
            // set the atlas_index to the current_animation_frame
            player.atlas_index = player.current_animation_frame;
        } // does nothing else and returns if the timer hasnt hit the necessary amount to play a following frame  
    }
}