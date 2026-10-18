use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::state::{GameOverSet, GameState};

pub struct ScoringPlugin;

// Relative to wherever the game is run from. Listed in .gitignore.
const SAVE_FILE: &str = "save.ron";

/// Points this round. Don't change it directly; send `PointScored`.
#[derive(Resource)]
pub struct GameScore(pub u32);

#[derive(Resource)]
pub struct HighScore(pub u32);

/// Everything persisted between runs. To save something new, add a field.
#[derive(Serialize, Deserialize, Default)]
struct SaveData {
    high_score: u32,
}

/// Send one per point from anything that awards points (pipes, future coins).
#[derive(Message)]
pub struct PointScored;

#[derive(Component)]
struct ScoreText;

impl Plugin for ScoringPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(GameScore(0));
        // build() runs once at startup, so this loads the save exactly once.
        let save = load_save();
        app.insert_resource(HighScore(save.high_score));
        app.add_systems(OnEnter(GameState::GameOver), update_high_score.in_set(GameOverSet::UpdateScores));
        app.add_message::<PointScored>();
        app.add_systems(OnEnter(GameState::PreGame), (reset_score, spawn_score_text));
        app.add_systems(Update, (
            increase_score,
            update_score_text.run_if(resource_changed::<GameScore>).run_if(in_state(GameState::Playing)),
        ));
    }
}

fn spawn_score_text(mut commands: Commands, window: Single<&Window>){
    let center_x = window.width() / 2.0;

    commands.spawn((
        ScoreText,
        Text::new("0"),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(center_x),
            ..default()
        },
        DespawnOnExit(GameState::Playing),
    ));
}

fn reset_score(mut score: ResMut<GameScore>){
    score.0 = 0;
}

// Resource first, then disk: the game over screen is right even if saving fails.
fn update_high_score(score: Res<GameScore>, mut high_score: ResMut<HighScore>){
    if score.0 > high_score.0 {
        high_score.0 = score.0;
        write_save(&SaveData { high_score: high_score.0 });
    }
}

// Any problem falls back to an empty save instead of crashing.
fn load_save() -> SaveData {
    let text = match std::fs::read_to_string(SAVE_FILE) {
        Ok(text) => text,
        // Match guard: a missing file just means first time playing, so no warning.
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return SaveData::default(),
        Err(e) => {
            warn!("Couldn't read {SAVE_FILE}: {e}. Starting with no high score.");
            return SaveData::default();
        }
    };

    match ron::from_str(&text) {
        Ok(save) => save,
        Err(e) => {
            warn!("{SAVE_FILE} is invalid: {e}. Starting with no high score.");
            SaveData::default()
        }
    }
}

// Failing to save shouldn't crash the game, so errors only warn.
fn write_save(save: &SaveData) {
    let text = match ron::ser::to_string_pretty(save, ron::ser::PrettyConfig::default()) {
        Ok(text) => text,
        Err(e) => {
            warn!("Couldn't convert save data to RON: {e}");
            return;
        }
    };

    if let Err(e) = std::fs::write(SAVE_FILE, text) {
        warn!("Couldn't write {SAVE_FILE}: {e}");
    }
}

fn update_score_text(score: Res<GameScore>, mut text: Single<&mut Text, With<ScoreText>>){
    text.0 = score.0.to_string();
}

fn increase_score(mut points: MessageReader<PointScored>, mut score: ResMut<GameScore>){
    for _ in points.read() {
        score.0 += 1;
    }
}
