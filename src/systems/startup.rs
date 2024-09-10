use bevy::prelude::*;

use crate::components::Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/image.png"),
            ..default()
        },
        Player,
    ));
}
