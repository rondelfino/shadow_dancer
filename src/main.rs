use bevy::{input::keyboard::KeyboardInput, math::const_vec2, prelude::*, transform};
use rand::{
    distributions::Standard,
    prelude::{random, Distribution},
    Rng,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct EnemyCount(u32);

struct SpawnTimer(Timer);

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;

const PLAYER_SPEED: f32 = 600.0;
const ENEMY_SPEED: f32 = 1000.0;

const GRAVITY: f32 = 4.5;
const ENEMY_REBOUND_FORCE: f32 = 700.0;

fn setup(mut commands: Commands) {
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //player
    spawn_player(&mut commands);
    spawn_wall(&mut commands);
}

fn spawn_wall(commands: &mut Commands) {
    commands.spawn().insert(Wall).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(WINDOW_HEIGHT, 300.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-800.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_player(commands: &mut Commands) {
    commands.spawn().insert(Player).insert_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 350.0, 1.0),
            ..default()
        },
        ..default()
    });
}

fn spawn_enemy(commands: &mut Commands) {
    let direction = match random::<bool>() {
        true => -1.,
        false => 1.,
    };
    commands
        .spawn()
        .insert(Enemy)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(direction * WINDOW_WIDTH / 2.0, -350.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Velocity(Vec2::new(1.0, 0.25) * ENEMY_SPEED));
}

fn enemy_spawner(time: Res<Time>, mut timer: ResMut<SpawnTimer>, mut commands: Commands) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn_enemy(&mut commands);
    }
}

fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &Sprite), With<Enemy>>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut velocity, sprite) in query.iter_mut() {
        let (left_bound, right_bound) = calculate_bounds(&transform, sprite);

        transform.translation.y += velocity.y * time.delta().as_secs_f32();
        transform.translation.x += velocity.x * time.delta().as_secs_f32();

        let is_touching_left_bound = left_bound < -WINDOW_WIDTH / 2.0;
        let is_touching_right_bound = right_bound > WINDOW_WIDTH / 2.0;

        if (velocity.x < 0.0 && is_touching_left_bound)
            || (velocity.x > 0.0 && is_touching_right_bound)
        {
            velocity.x = velocity.x * -1.0;
            velocity.y = ENEMY_REBOUND_FORCE;
        }

        if transform.translation.y > WINDOW_HEIGHT {
            commands.entity(entity).despawn();
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Sprite), With<Player>>,
) {
    let (mut player_translation, sprite) = query.single_mut();
    let (left_bound, right_bound) = calculate_bounds(&player_translation, sprite);

    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A])
        && left_bound > -WINDOW_WIDTH / 2.0
    {
        player_translation.translation.x -= PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D])
        && right_bound < WINDOW_WIDTH / 2.0
    {
        player_translation.translation.x += PLAYER_SPEED * time.delta().as_secs_f32();
    }
}

fn calculate_bounds(transform: &Transform, sprite: &Sprite) -> (f32, f32) {
    let left_bound = transform.translation.x - sprite.custom_size.unwrap().x / 2.;
    let right_bound = transform.translation.x + sprite.custom_size.unwrap().x / 2.;
    (left_bound, right_bound)
}

fn gravity_system(mut query: Query<&mut Velocity, With<Enemy>>) {
    for mut velocity in query.iter_mut() {
        velocity.y -= GRAVITY;
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "kajsfioawjer".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, true)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(enemy_spawner)
        .add_system(enemy_movement)
        .add_system(gravity_system)
        .run();
}
