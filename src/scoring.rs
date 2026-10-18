use bevy::prelude::*;

use crate::state::GameState;

pub struct ScoringPlugin;

#[derive(Resource)]
pub struct GameScore(pub u32);

// A message is a one-off "this happened" note. Any system can send one,
// and any system can read them. It carries no data here; the fact that one
// was sent is the whole message. (pub so pipe.rs, coins, etc. can send it.)
#[derive(Message)]
pub struct PointScored;

// Marker so update_score_text can find the score text among all text entities.
#[derive(Component)]
struct ScoreText;

impl Plugin for ScoringPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(GameScore(0));
        // Tell Bevy this message type exists so it can store them each frame.
        app.add_message::<PointScored>();
        // Each new round: zero the score and spawn fresh score text.
        app.add_systems(OnEnter(GameState::PreGame), (reset_score, spawn_score_text));
        app.add_systems(Update, (
            increase_score,
            // run_if: only run this system on frames where GameScore changed.
            update_score_text.run_if(resource_changed::<GameScore>).run_if(in_state(GameState::Playing)),
        ));
    }
}

// Spawns a UI text entity in the top-left corner showing the score.
// UI is laid out in SCREEN space (pixels from the window's corner), not the
// game world, so it stays put no matter what the camera or sprites do.
fn spawn_score_text(mut commands: Commands, window: Single<&Window>){
    let left_edge = window.width() / 2.0;
    
    commands.spawn((
        ScoreText,
        Text::new("0"),
        TextFont {
            font_size: FontSize::Px(48.0),
            // `..default()` = "every other field gets its default value".
            ..default()
        },
        // Node controls where UI goes. Absolute = place it exactly here,
        // measured from the window's top-left corner.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(left_edge),
            ..default()
        },
        // Removed on game over (the game over screen shows the score instead).
        DespawnOnExit(GameState::Playing),
    ));
}

fn reset_score(mut score: ResMut<GameScore>){
    score.0 = 0;
}

// Copies the number from the GameScore resource into the text on screen.
fn update_score_text(score: Res<GameScore>, mut text: Single<&mut Text, With<ScoreText>>){
    // Text is a tuple struct holding a String; .0 is that String.
    text.0 = score.0.to_string();
}

// Reads every PointScored sent since it last ran and adds 1 for each.
// It doesn't know or care WHO sent them (pipe, coin, ...).
fn increase_score(mut points: MessageReader<PointScored>, mut score: ResMut<GameScore>){
    // `_` because the message itself has no data we need, only that it exists.
    for _ in points.read() {
        score.0 += 1;
        println!("Score: {}", score.0);
    }
}
