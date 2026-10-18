use bevy::prelude::*;

use crate::scoring::GameScore;

pub struct StatePlugin;

// Every "mode" the game can be in. Bevy tracks which one is current.
// The derives let Bevy copy, compare, hash and print states.
// Add MainMenu / Paused here later.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default] // the state the game starts in
    PreGame,
    Playing,
    GameOver,
}

impl Plugin for StatePlugin{
    fn build(&self, app: &mut App){
        // Registers GameState and starts it at the #[default] variant.
        app.init_state::<GameState>();

        // OnEnter schedules run ONCE each time the game enters that state.
        app.add_systems(OnEnter(GameState::PreGame), pregame_text);
        app.add_systems(OnEnter(GameState::GameOver), game_over_text);

        // Each input system only listens in the state it belongs to.
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
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(center_x - 170.0), // roughly centered
            ..default()
        },
        // Gone the moment the round starts.
        DespawnOnExit(GameState::PreGame),
    ));
}

// While in PreGame: Space starts the round. (The bird's flap system also
// runs in PreGame, so this same press makes the bird jump as it starts.)
fn start_on_space(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>){
    if keys.just_pressed(KeyCode::Space){
        next.set(GameState::Playing);
    }
}

fn game_over_text(mut commands: Commands, window: Single<&Window>, score: Res<GameScore>){
    let center_x = window.width() / 2.0;

    commands.spawn((
        // format! builds a String the same way println! prints one.
        // (You can't `+` a "string literal" and a String in that order.)
        Text::new(format!("GAME OVER\nScore: {}\nPress R to restart", score.0)),
        TextFont {
            font_size: FontSize::Px(48.0),
            // `..default()` = "every other field gets its default value".
            ..default()
        },
        // Node controls where UI goes. Absolute = place it exactly here,
        // measured from the window's top-left corner.
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(200.0),
            left: Val::Px(center_x - 120.0), // roughly centered
            ..default()
        },
        // Removed automatically when we leave GameOver (i.e. on restart).
        DespawnOnExit(GameState::GameOver),
    ));
}

// While in GameOver: R goes back to PreGame. A different key from flap, so
// a panicked Space press while dying can't skip the game over screen.
fn restart_on_r(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>){
    if keys.just_pressed(KeyCode::KeyR){
        next.set(GameState::PreGame);
    }
}
