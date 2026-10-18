use bevy::prelude::*;
use crate::{movement::Velocity, state::GameState};

pub struct PipePlugin;

const PIPE_SPAWN_X: f32 = 1000.0;
const PIPE_GAP: f32 = 120.0;
// Bevy's default window height. World (0, 0) is the center, so the screen
// spans y = -360 to +360.
const SCREEN_HEIGHT: f32 = 720.0;
const MIN_PIPE_HEIGHT: f32 = 50.0;
const PIPE_WIDTH: f32 = 50.0;

const PIPE_SPEED: f32 = -200.0;
const PIPE_SPAWN_TIMER: f32 = 2.0;

// `y` values are sprite centers, because that's what Transform positions.
pub struct PipeLayout {
    pub top_height: f32,
    pub top_y: f32,
    pub bottom_height: f32,
    pub bottom_y: f32,
}

// Sizes both pipes to reach exactly from the gap to the screen edges.
fn pipe_layout(gap_center_y: f32) -> PipeLayout {
    let screen_top = SCREEN_HEIGHT / 2.0;
    let screen_bottom = -SCREEN_HEIGHT / 2.0;

    // Keep both pipes at least MIN_PIPE_HEIGHT tall, whatever is passed in.
    let limit = screen_top - PIPE_GAP / 2.0 - MIN_PIPE_HEIGHT;
    let gap_center_y = gap_center_y.clamp(-limit, limit);

    let gap_top = gap_center_y + PIPE_GAP / 2.0;
    let gap_bottom = gap_center_y - PIPE_GAP / 2.0;

    let top_height = screen_top - gap_top;
    let top_y = gap_top + top_height / 2.0;

    let bottom_height = gap_bottom - screen_bottom;
    let bottom_y = screen_bottom + bottom_height / 2.0;

    PipeLayout { top_height, top_y, bottom_height, bottom_y }
}

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

#[derive(Component)]
pub struct Pipe;

/// Top pipes are skipped when scoring, so each pair is worth one point.
#[derive(Component)]
pub struct TopPipe;

/// Added once a pipe has awarded its point, so it can't score again.
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
        // The gap size is fixed; moving its center is what varies the pipe heights.
        let gap_center_y = rand::random_range(-250.0..250.0);
        let layout = pipe_layout(gap_center_y);

        commands.spawn((
            Pipe,
            Velocity(Vec2::new(PIPE_SPEED, 0.0)),
            Transform::from_xyz(PIPE_SPAWN_X, layout.bottom_y, 0.0),
            Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(PIPE_WIDTH, layout.bottom_height)),
            DespawnOnExit(GameState::Playing),
        ));

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
    let left_edge = -window.width() / 2.0;
    for (entity, transform) in query{
        // x is the pipe's center, so wait until its right side is past the edge.
        if transform.translation.x < (left_edge - PIPE_WIDTH / 2.0) {
            commands.entity(entity).despawn();
        }
    }
}
