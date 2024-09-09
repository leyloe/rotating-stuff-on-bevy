use bevy::prelude::*;

use crate::{
    components::Player,
    systems::player::{move_player, rotate_player, scale_player},
};

pub fn updates(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    move_player(&keyboard_input, &mut query, &time);
    scale_player(&keyboard_input, &mut query, &time);
    rotate_player(keyboard_input, query, time)
}
