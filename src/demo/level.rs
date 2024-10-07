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
    spawn_physics_square(world);
}

fn spawn_physics_square(world: &mut World) {
    let mut commands = world.get_resource_mut::<Commands>().unwrap();
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 50.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        SpriteBundle {
            texture: world.resource::<AssetServer>().load("images/physics_square.png"),
            ..Default::default()
        },
    ));
}
