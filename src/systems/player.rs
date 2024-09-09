use bevy::prelude::*;

use crate::{components::Player, constants::PLAYER_SPEED};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/image.png"),
            ..default()
        },
        Player,
    ));
}

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        x_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        x_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        y_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        y_direction += 1.0;
    }

    let new_player_x_position =
        player_transform.translation.x + x_direction * PLAYER_SPEED * time.delta_seconds();
    let new_player_y_position =
        player_transform.translation.y + y_direction * PLAYER_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_x_position;
    player_transform.translation.y = new_player_y_position;
}
