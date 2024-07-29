use crate::{
    snake::{SnakeHead, SnakeTail, Speed},
    Scoreboard, APPLE_SIZE, SNAKE_SIZE, SPEED_INCREASE, TAIL_INCREASE,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use rand::Rng;

#[derive(Component)]
pub struct Apple;

#[derive(Component)]
pub struct ApplesEaten(pub usize);

pub struct ApplesPlugin;

impl Plugin for ApplesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_first_apple)
            .add_systems(Update, being_eaten);
    }
}

fn spawn_first_apple(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle { radius: APPLE_SIZE }).into(),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(50.0, 100.0, 0.0),
            ..default()
        },
        Apple,
    ));
    command.spawn(ApplesEaten(0));
}

fn being_eaten(
    mut commands: Commands,
    mut apples: Query<(Entity, &GlobalTransform), With<Apple>>,
    mut snake: Query<(&GlobalTransform, &mut Speed), With<SnakeHead>>,
    mut sb: Query<&mut Text, With<Scoreboard>>,
    mut apples_eaten: Query<&mut ApplesEaten>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let (snake, mut snake_speed) = snake.single_mut();
        for (entity, apple) in apples.iter_mut() {
            let distance = snake.translation().distance(apple.translation());
            if distance < 30.0 {
                commands.entity(entity).despawn_recursive();
                snake_speed.0 += SPEED_INCREASE;
                let mut rng = rand::thread_rng();
                let window = &window.single().resolution;
                let w = window.width() / 2.0;
                let h = window.height() / 2.0;
                let x: f32 = rng.gen_range(-w * 0.95..=w * 0.95);
                let y: f32 = rng.gen_range(-h * 0.95..=h * 0.95);
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Circle { radius: APPLE_SIZE }).into(),
                        material: materials.add(Color::RED),
                        transform: Transform::from_xyz(x, y, 0.0),
                        ..default()
                    },
                    Apple,
                ));
                for _ in 0..TAIL_INCREASE {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::new(SNAKE_SIZE, SNAKE_SIZE)).into(),
                            material: materials.add(Color::PURPLE),
                            transform: Transform::from_xyz(
                                snake.translation().x,
                                snake.translation().y,
                                0.0,
                            ),
                            ..default()
                        },
                        SnakeTail,
                    ));
                }
                let mut apples_eaten = apples_eaten.single_mut();
                apples_eaten.0 += 1;
                sb.single_mut().sections[1].value = format!("{}", apples_eaten.0);
            }
        }
}
