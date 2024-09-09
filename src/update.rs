use bevy::prelude::*;

use crate::{components::Player, systems::player::move_player};

pub fn updates(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    move_player(keyboard_input, query, time);
}
