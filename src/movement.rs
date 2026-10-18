use bevy::prelude::*;

use crate::state::GameState;

pub struct MovementPlugin;

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Plugin for MovementPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, apply_velocity.run_if(in_state(GameState::Playing)));
    }
}

// Moves EVERY entity that has both a Transform and a Velocity.
// No With<Bird> filter, so pipes will get moved by this too later.
fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>){
    for (mut transform, velocity) in &mut query{
        // translation is a Vec3 (x, y, z) and velocity is a Vec2 (x, y),
        // so update x and y separately and leave z alone.
        transform.translation.x += velocity.0.x * time.delta_secs();
        transform.translation.y += velocity.0.y * time.delta_secs();
    }
}
