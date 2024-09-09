use bevy::prelude::*;

use crate::{
    components::Player,
    constants::{PLAYER_MOVEMENT_SPEED, PLAYER_ROTATION_SPEED, PLAYER_SCALE_SPEED},
};

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
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    query: &mut Query<&mut Transform, With<Player>>,
    time: &Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut x_direction = 0.0;
    let mut y_direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) | keyboard_input.pressed(KeyCode::ArrowLeft) {
        x_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) | keyboard_input.pressed(KeyCode::ArrowRight) {
        x_direction += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) | keyboard_input.pressed(KeyCode::ArrowDown) {
        y_direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) | keyboard_input.pressed(KeyCode::ArrowUp) {
        y_direction += 1.0;
    }

    let new_player_x_position =
        player_transform.translation.x + x_direction * PLAYER_MOVEMENT_SPEED * time.delta_seconds();
    let new_player_y_position =
        player_transform.translation.y + y_direction * PLAYER_MOVEMENT_SPEED * time.delta_seconds();

    player_transform.translation.x = new_player_x_position;
    player_transform.translation.y = new_player_y_position;
}

pub fn scale_player(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    query: &mut Query<&mut Transform, With<Player>>,
    time: &Res<Time>,
) {
    let mut player_transform = query.single_mut();

    let mut size = 0.0;

    if keyboard_input.pressed(KeyCode::Minus) {
        size -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Equal) {
        size += 1.0;
    }

    let new_player_size = player_transform.scale + size * PLAYER_SCALE_SPEED * time.delta_seconds();

    player_transform.scale = new_player_size;
}

pub fn rotate_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();

    let mut rotate = 0.0;

    if keyboard_input.pressed(KeyCode::BracketRight) {
        rotate -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::BracketLeft) {
        rotate += 1.0;
    }

    player_transform.rotate_z(rotate * PLAYER_ROTATION_SPEED * time.delta_seconds());
}
