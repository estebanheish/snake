mod apples;
mod snake;

use crate::apples::ApplesPlugin;
use crate::snake::SnakePlugin;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;

pub const APPLE_SIZE: f32 = 15.0;
pub const SPEED_INCREASE: f32 = 50.0;
pub const TAIL_INCREASE: usize = 10;
pub const STARTING_SPEED: f32 = 200.0;
pub const SNAKE_SIZE: f32 = 25.0;

#[derive(Component)]
struct Scoreboard;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Running,
    GameOver,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings::default(),
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Apples: ",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: 30.0,
                    ..default()
                },
            ),
        ])
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        Scoreboard,
    ));
}

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(Update, restart.run_if(in_state(GameState::GameOver)))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((SnakePlugin, ApplesPlugin))
        .run();
}
