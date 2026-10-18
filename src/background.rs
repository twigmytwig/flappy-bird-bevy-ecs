use bevy::prelude::*;

pub struct BackgroundPlugin;

// The art is 256x256. 3x makes it 768 tall, slightly taller than the 720
// window (the extra is cut off), and whole-number scales keep pixels even.
const BACKGROUND_SCALE: f32 = 3.0;
const BACKGROUND_ART_SIZE: f32 = 256.0;
// Wide enough to cover the default 1280px window.
const BACKGROUND_WIDTH: f32 = 1280.0;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
    }
}

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(BACKGROUND_WIDTH, BACKGROUND_ART_SIZE * BACKGROUND_SCALE)),
            // Repeat the image sideways instead of stretching it to 1280 wide.
            // stretch_value: draw each copy at 3x before repeating.
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: BACKGROUND_SCALE,
            },
            ..Sprite::from_image(asset_server.load("Flappy Bird Assets/Background/Background4.png"))
        },
        // Lower z draws behind. Bird and pipes are at z = 0.
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));
}
