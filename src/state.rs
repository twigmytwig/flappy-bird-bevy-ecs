use bevy::prelude::*;

use crate::scoring::{GameScore, HighScore};

pub struct StatePlugin;

/// Phases that run, in order, when entering `GameState::GameOver`.
///
/// Join one with `.in_set(GameOverSet::...)` on an `OnEnter(GameState::GameOver)`
/// system. The order is set once in `StatePlugin`, so plugins never need to
/// name each other's functions.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameOverSet {
    /// The final score is known: update high score, save data, stats.
    UpdateScores,
    /// Build the game over screen. Can rely on `UpdateScores` being done.
    ShowUi,
}

/// Flow: `PreGame` → (Space) → `Playing` → (hit) → `GameOver` → (R) → `PreGame`.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    PreGame,
    Playing,
    GameOver,
}

impl Plugin for StatePlugin{
    fn build(&self, app: &mut App){
        app.init_state::<GameState>();

        app.add_systems(OnEnter(GameState::PreGame), pregame_text);
        app.configure_sets(
            OnEnter(GameState::GameOver),
            (GameOverSet::UpdateScores, GameOverSet::ShowUi).chain(),
        );
        app.add_systems(OnEnter(GameState::GameOver), game_over_text.in_set(GameOverSet::ShowUi));

        app.add_systems(Update, start_on_space.run_if(in_state(GameState::PreGame)));
        app.add_systems(Update, restart_on_r.run_if(in_state(GameState::GameOver)));
    }
}

fn pregame_text(mut commands: Commands, window: Single<&Window>){
    let center_x = window.width() / 2.0;

    commands.spawn((
        Text::new("Press Space to start"),
        TextFont {
            font_size: FontSize::Px(36.0),
            ..default()
        },
        // UI is measured from the window's top-left corner, not the world center.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(center_x - 170.0), // roughly centered
            ..default()
        },
        DespawnOnExit(GameState::PreGame),
    ));
}

fn start_on_space(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>){
    if keys.just_pressed(KeyCode::Space){
        next.set(GameState::Playing);
    }
}

fn game_over_text(
    mut commands: Commands,
    window: Single<&Window>,
    score: Res<GameScore>,
    high_score: Res<HighScore>,
){
    let center_x = window.width() / 2.0;

    commands.spawn((
        Text::new(format!(
            "GAME OVER\nScore: {}\nBest: {}\nPress R to restart",
            score.0, high_score.0
        )),
        TextFont {
            font_size: FontSize::Px(48.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(center_x - 120.0), // roughly centered
            ..default()
        },
        DespawnOnExit(GameState::GameOver),
    ));
}

// R, not Space, so a panicked flap while dying can't skip this screen.
fn restart_on_r(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>){
    if keys.just_pressed(KeyCode::KeyR){
        next.set(GameState::PreGame);
    }
}
