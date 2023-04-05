use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_tweening::*;
use rand::prelude::*;
pub const PLAYER_SIZE: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 300.0;
pub const ENEMY_SPEED: f32 = 700.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemies)
        .add_system(player_movement)
        .add_system(enemy_movement)
        .add_system(confine_player_movement)
        .add_system(confine_enemy_movement)
        .run();
}
#[derive(Component)]
pub struct Player {}
#[derive(Component)]
pub struct Enemy {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        },
        Player {},
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, _)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::H) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::L) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::J) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::K) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    let mut tween = Tween::new();
    let mut start_position = Vec3::ZERO;
    let mut end_position = Vec3::ZERO;
    let duration = time::Duration::from_secs(1);

    for (mut transform, _) in enemy_query.iter_mut() {
        let random_number = random::<f32>();
        if random_number < 0.25 {
            start_position = transform.translation;
            end_position = start_position + Vec3::new(-1.0, 0.0, 0.0);
            tween.tween_to(start_position, end_position, duration);
        } else if random_number < 0.5 {
            start_position = transform.translation;
            end_position = start_position + Vec3::new(1.0, 0.0, 0.0);
            tween.tween_to(start_position, end_position, duration);
        } else if random_number < 0.75 {
            start_position = transform.translation;
            end_position = start_position + Vec3::new(0.0, -1.0, 0.0);
            tween.tween_to(start_position, end_position, duration);
        } else {
            start_position = transform.translation;
            end_position = start_position + Vec3::new(0.0, 1.0, 0.0);
            tween.tween_to(start_position, end_position, duration);
        }

        transform.translation = tween.value(time.delta_seconds());
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {},
        ));
    }
}

pub fn confine_player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, With<Player>)>,
) {
    if let Ok((mut player_transform, _)) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        player_transform.translation = translation;
    }
}
pub fn confine_enemy_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut enemy_query: Query<(&mut Transform, With<Enemy>)>,
) {
    for (mut enemy_transform, _) in enemy_query.iter_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = enemy_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }
        enemy_transform.translation = translation;
    }
}
