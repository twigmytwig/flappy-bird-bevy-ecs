use bevy::prelude::*;

use crate::movement::Velocity;
use crate::pipe::{Pipe, ScoredAlready, TopPipe};
use crate::scoring::PointScored;
use crate::state::GameState;

pub struct BirdPlugin;

// Upward speed (pixels per second) the bird gets from one flap.
const FLAP_STRENGTH: f32 = 350.0;
// How fast gravity pulls velocity down (pixels per second, per second).
const GRAVITY: f32 = 1000.0;

#[derive(Component)]
pub struct Bird;

impl Plugin for BirdPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(OnEnter(GameState::PreGame), spawn_bird);
        // A tuple registers several systems at once. Update = every frame.
        app.add_systems(Update, (apply_gravity, check_collision, check_pipe_passed).run_if(in_state(GameState::Playing)));
        // Flap also runs in PreGame, so the Space press that starts the round
        // gives the bird its first jump. or_else = "either condition is true".
        app.add_systems(Update, flap.run_if(in_state(GameState::PreGame).or_else(in_state(GameState::Playing))));
    }
}

fn spawn_bird(mut commands: Commands){
    commands.spawn((
        Bird,
        Velocity(Vec2::ZERO),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        Sprite::from_color(Color::srgb(1.0, 0.8, 0.0), Vec2::new(30.0, 30.0)),
        // Removed automatically when we leave Playing (game over).
        DespawnOnExit(GameState::Playing),
    ));
}

// Runs every frame, but only does something on the frame Space goes down.
// It SETS the upward speed once; apply_velocity does the actual moving.
fn flap(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Bird>>){
    if keys.just_pressed(KeyCode::Space){
        for mut velocity in &mut query{
            velocity.0.y = FLAP_STRENGTH;
        }
    }
}

// Runs every frame: pulls the bird's velocity down a little each time.
// Multiplying by delta time makes it the same per second at any frame rate.
fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Bird>>){
    for mut velocity in &mut query{
        velocity.0.y -= GRAVITY * time.delta_secs();
    }
}

//instead of scoringwhen you pass a pipe, im gonna grant it when the transforms.x are >= the pipes
fn check_pipe_passed(
    mut commands: Commands,
    mut message: MessageWriter<PointScored>,
    bird: Single<&Transform, With<Bird>>,
    pipes: Query<(Entity, &Transform), (With<Pipe>, Without<TopPipe>, Without<ScoredAlready>)>,
){
    let bird_transform = *bird;
    // Destructure the tuple into names instead of using pipe.0 / pipe.1.
    for (pipe_entity, pipe_transform) in &pipes {
        if bird_transform.translation.x >= pipe_transform.translation.x {
            // Tell the scoring system a point happened...
            message.write(PointScored);
            // ...and mark this pipe so the Without<ScoredAlready> filter
            // skips it from the next frame on.
            commands.entity(pipe_entity).insert(ScoredAlready);
        }
    }
}

fn check_collision(
    bird: Single<(&Transform, &Sprite), With<Bird>>,
    pipes: Query<(&Transform, &Sprite), With<Pipe>>,
    mut next: ResMut<NextState<GameState>>,
    window: Single<&Window>
){
    
    // Single gives us the bird's (Transform, Sprite) tuple directly; `*`
    // unwraps the Single so we can destructure it into two variables.
    let (bird_transform, bird_sprite) = *bird;

    // custom_size is an Option<Vec2>. `let ... else` says: if it's Some, pull
    // the size out; otherwise run the else block (here: stop the system).
    let Some(bird_size) = bird_sprite.custom_size else { return; };

    // (0, 0) is the screen CENTER, so each edge is HALF the height away.
    let screen_top = window.height() / 2.0;
    let screen_bottom = -window.height() / 2.0;
    // Use the bird's edges, not its center, so it dies the moment it touches.
    let bird_top = bird_transform.translation.y + bird_size.y / 2.0;
    let bird_bottom = bird_transform.translation.y - bird_size.y / 2.0;

    if bird_bottom <= screen_bottom || bird_top >= screen_top {
        next.set(GameState::GameOver);
        return; // already dead, no need to check pipes
    }
    // translation is a Vec3 (x, y, z); truncate() drops z, giving a Vec2 (x, y).
    let bird_center = bird_transform.translation.truncate();

    for (pipe_transform, pipe_sprite) in &pipes {
        // Same idea, but `continue` skips just this pipe instead of stopping.
        let Some(pipe_size) = pipe_sprite.custom_size else { continue; };
        let pipe_center = pipe_transform.translation.truncate();

        if rects_overlap(bird_center, bird_size, pipe_center, pipe_size) {
            //Change to game over satate.
            next.set(GameState::GameOver);
        }
    }
}

//TODO: move this out of bird, it is a general fucntio that could be reused eventually
// AABB check: two rectangles touch only if they overlap on x AND on y.
// Each one is described by its center and its full size (width, height).
fn rects_overlap(a_center: Vec2, a_size: Vec2, b_center: Vec2, b_size: Vec2) -> bool {
    let a_left = a_center.x - a_size.x / 2.0;
    let a_right = a_center.x + a_size.x / 2.0;
    let a_bottom = a_center.y - a_size.y / 2.0;
    let a_top = a_center.y + a_size.y / 2.0;

    let b_left = b_center.x - b_size.x / 2.0;
    let b_right = b_center.x + b_size.x / 2.0;
    let b_bottom = b_center.y - b_size.y / 2.0;
    let b_top = b_center.y + b_size.y / 2.0;

    let overlap_x = a_left < b_right && a_right > b_left;
    let overlap_y = a_bottom < b_top && a_top > b_bottom;

    overlap_x && overlap_y
}
