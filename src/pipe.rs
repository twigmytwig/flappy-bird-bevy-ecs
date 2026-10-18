use bevy::prelude::*;
use crate::{movement::Velocity, state::GameState};

pub struct PipePlugin;

const PIPE_SPAWN_X: f32 = 1000.0;
const PIPE_GAP: f32 = 120.0;
// Bevy's default window is 1280x720. (0, 0) is the center, so the top
// of the screen is y = +360 and the bottom is y = -360.
const SCREEN_HEIGHT: f32 = 720.0;
// Shortest a pipe is allowed to be, so the gap never touches the screen edge.
const MIN_PIPE_HEIGHT: f32 = 50.0;
const PIPE_WIDTH: f32 = 50.0;

const PIPE_SPEED: f32 = -200.0;
const PIPE_SPAWN_TIMER: f32 = 2.0;

// Everything spawn_pipe needs to place one top/bottom pair.
// `y` values are sprite CENTERS, because that's what Transform uses.
pub struct PipeLayout {
    pub top_height: f32,
    pub top_y: f32,
    pub bottom_height: f32,
    pub bottom_y: f32,
}

// Given where the middle of the gap should be, work out how tall each pipe
// is and where its center goes so the pipes reach exactly to the screen edges.
fn pipe_layout(gap_center_y: f32) -> PipeLayout {
    let screen_top = SCREEN_HEIGHT / 2.0;
    let screen_bottom = -SCREEN_HEIGHT / 2.0;

    // Keep the gap far enough from the edges that both pipes stay at least
    // MIN_PIPE_HEIGHT tall, even if someone passes a silly value.
    let limit = screen_top - PIPE_GAP / 2.0 - MIN_PIPE_HEIGHT;
    let gap_center_y = gap_center_y.clamp(-limit, limit);

    let gap_top = gap_center_y + PIPE_GAP / 2.0;
    let gap_bottom = gap_center_y - PIPE_GAP / 2.0;

    // Top pipe: from the top of the gap up to the top of the screen.
    let top_height = screen_top - gap_top;
    let top_y = gap_top + top_height / 2.0;

    // Bottom pipe: from the bottom of the screen up to the bottom of the gap.
    let bottom_height = gap_bottom - screen_bottom;
    let bottom_y = screen_bottom + bottom_height / 2.0;

    PipeLayout { top_height, top_y, bottom_height, bottom_y }
}

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct TopPipe;

#[derive(Component)]
pub struct ScoredAlready;

impl Plugin for PipePlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(PipeSpawnTimer(Timer::from_seconds(PIPE_SPAWN_TIMER, TimerMode::Repeating)));
        app.add_systems(Update, (spawn_pipe, despawn_pipe).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_pipe(time: Res<Time>, mut timer: ResMut<PipeSpawnTimer>, mut commands: Commands){
    if timer.0.tick(time.delta()).just_finished(){
        //This is unique because pipes spawn in sets of two with a space.
        //i think the best way to calculate this is probably to randomly make a height of the pipes so long as they
        //are always the smae amount of units apart.
        // Pick a random spot for the middle of the gap. The gap SIZE never
        // changes (PIPE_GAP); moving its center up/down is what makes the top
        // and bottom pipe heights vary. pipe_layout clamps anything too close
        // to the screen edges.
        let gap_center_y = rand::random_range(-250.0..250.0);
        let layout = pipe_layout(gap_center_y);
        // Both pipes in a pair share the same x so they line up vertically.

        // Bottom pipe
        commands.spawn((
            Pipe,
            Velocity(Vec2::new(PIPE_SPEED, 0.0)),
            Transform::from_xyz(PIPE_SPAWN_X, layout.bottom_y, 0.0),
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(PIPE_WIDTH, layout.bottom_height)),
            DespawnOnExit(GameState::Playing),
        ));

        // Top pipe
        commands.spawn((
            Pipe,
            TopPipe,
            Velocity(Vec2::new(PIPE_SPEED, 0.0)),
            Transform::from_xyz(PIPE_SPAWN_X, layout.top_y, 0.0),
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(PIPE_WIDTH, layout.top_height)),
            DespawnOnExit(GameState::Playing),
        ));
    }
}

fn despawn_pipe(mut commands: Commands, window: Single<&Window>, query: Query<(Entity, &Transform), With<Pipe>>){
    //loop through positions. If positions are out of bounds, despawn.
    let left_edge = -window.width() / 2.0; //TODO: have a resource that stores/updates this value to save time
    for (entity, transform) in query{
        if transform.translation.x < (left_edge - PIPE_WIDTH / 2.0) {
            commands.entity(entity).despawn();
        }
    }
}