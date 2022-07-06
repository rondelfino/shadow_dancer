use bevy::{input::keyboard::KeyboardInput, prelude::*, transform};
use rand::{
    distributions::Standard,
    prelude::{random, Distribution},
    Rng,
};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy {
    direction: Direction,
}

#[derive(Component)]
struct Velocity(f32);

#[derive(Component)]
struct EnemyCount(u32);

struct SpawnTimer(Timer);

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 800.0;

fn setup(mut commands: Commands) {
    //camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //player
    spawn_player(&mut commands);
    //enemy
    spawn_enemy(&mut commands);
}

fn spawn_player(commands: &mut Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Velocity(5.0))
        .insert_bundle(SpriteBundle {
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
    commands
        .spawn()
        .insert(Enemy {
            direction: random::<Direction>(),
        })
        .insert(Velocity(2.0))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -350.0, 1.0),
                ..default()
            },
            ..default()
        });
}

fn enemy_spawner(time: Res<Time>, mut timer: ResMut<SpawnTimer>, mut commands: Commands) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn_enemy(&mut commands);
    }
}

fn enemy_movement(mut query: Query<(&mut Transform, &Velocity, &Sprite, &mut Enemy), With<Enemy>>) {
    for (mut transform, speed, sprite, mut enemy) in query.iter_mut() {
        let (left_bound, right_bound) = calculate_bounds(&transform, sprite);
        transform.translation.y += speed.0;

        let is_touching_left_bound = left_bound < -WINDOW_WIDTH / 2.0;
        let is_touching_right_bound = right_bound > WINDOW_WIDTH / 2.0;

        if enemy.direction == Direction::Left {
            if is_touching_left_bound {
                enemy.direction = Direction::Right;
            }
            transform.translation.x -= speed.0;
        } else if enemy.direction == Direction::Right {
            if is_touching_right_bound {
                enemy.direction = Direction::Left;
            }
            transform.translation.x += speed.0;
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Velocity, &Sprite), With<Player>>,
) {
    let (mut player_translation, player_speed, sprite) = query.single_mut();
    let (left_bound, right_bound) = calculate_bounds(&player_translation, sprite);

    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A])
        && left_bound > -WINDOW_WIDTH / 2.0
    {
        player_translation.translation.x -= player_speed.0;
    }

    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D])
        && right_bound < WINDOW_WIDTH / 2.0
    {
        player_translation.translation.x += player_speed.0;
    }
}

fn calculate_bounds(transform: &Transform, sprite: &Sprite) -> (f32, f32) {
    let left_bound = transform.translation.x - sprite.custom_size.unwrap().x / 2.;
    let right_bound = transform.translation.x + sprite.custom_size.unwrap().x / 2.;
    (left_bound, right_bound)
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
        .run();
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        if rng.gen_bool(0.5) {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}
