use bevy::{ecs::world::Command, prelude::*};
use bevy_rapier2d::prelude::*;

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

    // Add a square with a physics body that can be collided with
    world.spawn((
        RigidBody::Fixed,
        Collider::cuboid(50.0, 50.0),
        Transform::from_xyz(1.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}
