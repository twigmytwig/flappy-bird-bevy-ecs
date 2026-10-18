use bevy::prelude::*;

mod bird;
mod pipe;
mod movement;
mod scoring;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(state::StatePlugin)
        .add_plugins(bird::BirdPlugin)
        .add_plugins(pipe::PipePlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(scoring::ScoringPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
