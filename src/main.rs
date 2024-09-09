use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_stuff::{startup::setup, update::updates};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, updates)
        .run();
}
