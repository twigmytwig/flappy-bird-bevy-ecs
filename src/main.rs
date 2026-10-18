use bevy::prelude::*;

mod animation;
mod background;
mod bird;
mod pipe;
mod movement;
mod scoring;
mod state;

fn main() {
    App::new()
        // Nearest-neighbor scaling keeps pixel art sharp instead of blurry.
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(state::StatePlugin)
        .add_plugins(bird::BirdPlugin)
        .add_plugins(pipe::PipePlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(background::BackgroundPlugin)
        .add_plugins(scoring::ScoringPlugin)
        .add_systems(Startup, spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2d);
}
