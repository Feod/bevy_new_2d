use bevy::prelude::*;

mod animation;
pub mod level;
mod movement;
pub mod player;
mod physics;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        movement::plugin,
        player::plugin,
        level::plugin,
        physics::plugin,
    ));
}
