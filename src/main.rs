use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_stuff::systems::{
    player::{move_player, rotate_player, scale_player},
    startup::setup,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, scale_player, rotate_player))
        .run();
}
