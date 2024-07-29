use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

use crate::{GameState, SNAKE_SIZE, STARTING_SPEED};

#[derive(Component)]
pub struct SnakeHead;

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Speed(pub f32);

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_snake).add_systems(
            Update,
            (
                direction_control,
                movement.run_if(in_state(GameState::Running)),
                tail_colision,
                wrap_around,
            )
                .chain(),
        );
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(SNAKE_SIZE, SNAKE_SIZE)).into(),
            material: materials.add(Color::PURPLE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Direction { x: 1.0, y: 0.0 },
        Speed(STARTING_SPEED),
        SnakeHead,
    ));
}

fn movement(
    mut head: Query<(&Direction, &mut Transform, &Speed), With<SnakeHead>>,
    mut tail: Query<&mut Transform, (With<SnakeTail>, Without<SnakeHead>)>,
    time: Res<Time>,
) {
    let (d, mut t, Speed(s)) = head.single_mut();

    let mut h_x = t.translation.x;
    let mut h_y = t.translation.y;

    t.translation.x += d.x * time.delta_seconds() * s;
    t.translation.y += d.y * time.delta_seconds() * s;

    for mut d in tail.iter_mut() {
        let t_x = d.translation.x;
        let t_y = d.translation.y;
        d.translation.x = h_x;
        d.translation.y = h_y;
        h_x = t_x;
        h_y = t_y;
    }
}

fn direction_control(
    mut query: Query<&mut Direction, With<SnakeHead>>,
    kb: Res<ButtonInput<KeyCode>>,
) {
    let mut d = query.single_mut();
    if kb.pressed(KeyCode::KeyQ) && d.x != 1.0 {
        d.x = -1.0;
        d.y = 0.0;
        return;
    }
    if kb.pressed(KeyCode::KeyW) && d.y != -1.0 {
        d.x = 0.0;
        d.y = 1.0;
        return;
    }
    if kb.pressed(KeyCode::KeyF) && d.x != -1.0 {
        d.x = 1.0;
        d.y = 0.0;
        return;
    }
    if kb.pressed(KeyCode::KeyR) && d.y != 1.0 {
        d.x = 0.0;
        d.y = -1.0;
    }
}

// fn wall_colision(
//     window: Query<&Window, With<PrimaryWindow>>,
//     mut snake: Query<&mut Transform, With<SnakeHead>>,
//     mut next_state: ResMut<NextState<GameState>>,
// ) {
//     let window = &window.single().resolution;
//     let w = window.width() / 2.0;
//     let h = window.height() / 2.0;

//     let snake_t = snake.single_mut().translation;
//     let x = snake_t.x;
//     let y = snake_t.y;
//     if x > w || x < -w || y > h || y < -h {
//         next_state.set(GameState::GameOver);
//     }
// }

fn tail_colision(
    head: Query<(&Transform, &Direction), (With<SnakeHead>, Without<SnakeTail>)>,
    tail: Query<&Transform, With<SnakeTail>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let (head_t, head_d) = head.single();
    let head_t = head_t.translation;
    for tail in tail.iter() {
        let tail_t = tail.translation;
        if head_t.distance(tail_t) <= SNAKE_SIZE
            && (head_d.x < 0.0 && tail_t.x < head_t.x
                || head_d.x > 0.0 && tail_t.x > head_t.x
                || head_d.y < 0.0 && tail_t.y < head_t.y
                || head_d.y > 0.0 && tail_t.y > head_t.y)
        {
            next_state.set(GameState::GameOver);
        }
    }
}

fn wrap_around(
    window: Query<&Window, With<PrimaryWindow>>,
    mut snake: Query<&mut Transform, With<SnakeHead>>,
) {
    let window = &window.single().resolution;
    let mut snake = snake.single_mut();
    let w = window.width() / 2.0;
    if snake.translation.x > w {
        snake.translation.x = -w;
    }
    if snake.translation.x < -w {
        snake.translation.x = w;
    }

    let h = window.height() / 2.0;
    if snake.translation.y > h {
        snake.translation.y = -h;
    }
    if snake.translation.y < -h {
        snake.translation.y = h;
    }
}
