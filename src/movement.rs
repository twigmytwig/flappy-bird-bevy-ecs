use bevy::prelude::*;

use crate::state::GameState;

pub struct MovementPlugin;

/// Pixels per second (x = right, y = up). Anything with this and a `Transform`
/// is moved by `apply_velocity`; no other movement code needed.
#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, apply_velocity.run_if(in_state(GameState::Playing)));
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>){
    for (mut transform, velocity) in &mut query{
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}
