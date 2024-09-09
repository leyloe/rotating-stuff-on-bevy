use bevy::prelude::*;

use crate::systems::player::spawn_player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn_player(commands, asset_server);
}
