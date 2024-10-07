use bevy::{ecs::world::Command, prelude::*};
use bevy_rapier2d::prelude::*; // P5793

use crate::demo::player::SpawnPlayer;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    SpawnPlayer { max_speed: 400.0 }.apply(world);
    spawn_physics_square(world);
}

fn spawn_physics_square(world: &mut World) {
    let texture_handle = {
        let asset_server = world.resource::<AssetServer>();
        asset_server.load("images/WhiteSquare.png")
    };

    let mut commands = world.spawn();
    commands.insert_bundle(SpriteBundle {
        texture: texture_handle,
        transform: Transform::from_scale(Vec3::splat(50.0)),
        ..Default::default()
    });
    commands.insert(RigidBody::Dynamic); // P5793
    commands.insert(Collider::cuboid(25.0, 25.0)); // P6080
}
