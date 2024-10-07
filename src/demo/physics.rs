use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default());
    app.add_startup_system(spawn_physics_entity);
}

fn spawn_physics_entity(mut commands: Commands) {
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(50.0, 50.0),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}
