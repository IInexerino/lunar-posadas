use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub last_action: LastAction,
}        

#[derive(Default, PartialEq)]
pub enum LastAction {
    #[default]
    None,
    WalkBack,
    WalkForward,
    WalkSideOrForward,
    WalkSideBack,
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
                if direction.y > 0.0 { LastAction::WalkBack }
                else { LastAction::WalkForward }
            } else if direction.x.abs() > 0.0 {
                if direction.y > 0.0 { LastAction::WalkSideBack }
                else { LastAction::WalkSideOrForward }
            } else {
                LastAction::None
            };
        } else {
            player.last_action = LastAction::None;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_movement);
    }
}