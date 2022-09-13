use bevy::{
    prelude::*,
    render::{
        camera::{DepthCalculation, ScalingMode},
        texture::ImageSettings,
    },
};
use components::*;
use enemy::EnemyBundle;
use player::PlayerBundle;

struct EnemyCount(u32);

struct SpawnTimer(Timer);

enum GameState {
    Splash,
    MainMenu,
    InGame,
    Paused,
}

mod components;
mod enemy;
mod player;
// mod settings;
// mod systems;

const WORLD_WIDTH: f32 = 550.0;
const WINDOW_HEIGHT: f32 = 800.0;

const PLAYER_SPEED: f32 = 600.0;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection = OrthographicProjection {
        far: 1000.0,
        depth_calculation: DepthCalculation::ZDifference,
        scaling_mode: ScalingMode::None,
        scale: 1.0,
        left: -400.0,
        right: 400.0,
        top: 225.0,
        bottom: -225.0,
        ..Default::default()
    };
    //camera
    commands.spawn_bundle(camera_bundle);
    //player
    spawn_player(&mut commands, asset_server, texture_atlases);
}

fn spawn_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn()
        .insert(Player)
        .insert_bundle(PlayerBundle::new(asset_server, texture_atlases));
}

fn spawn_enemy(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn().insert_bundle(EnemyBundle::pawn(
        WORLD_WIDTH,
        asset_server,
        texture_atlases,
    ));
}

fn enemy_spawner(
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut count: ResMut<EnemyCount>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn_enemy(&mut commands, asset_server, texture_atlases);
        count.0 += 1;
    }
}

fn enemy_movement(
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &mut Velocity,
            &mut TextureAtlasSprite,
            &InitialEnemySpeed,
        ),
        With<Enemy>,
    >,
    mut commands: Commands,
) {
    for (entity, mut transform, mut velocity, mut sprite, initial_enemy_speed) in query.iter_mut() {
        let (left_bound, right_bound) = calculate_bounds(&transform, sprite.custom_size);

        transform.translation.y += velocity.y * time.delta().as_secs_f32();
        transform.translation.x += velocity.x * time.delta().as_secs_f32();

        let is_touching_left_bound = left_bound < -WORLD_WIDTH / 2.0;
        let is_touching_right_bound = right_bound > WORLD_WIDTH / 2.0;

        if (velocity.x < 0.0 && is_touching_left_bound)
            || (velocity.x > 0.0 && is_touching_right_bound)
        {
            velocity.x = velocity.x * -1.0;
            velocity.y = initial_enemy_speed.0;
        }

        if velocity.x < 0.0 {
            sprite.index = 1;
        } else {
            sprite.index = 0;
        }

        if transform.translation.y > WINDOW_HEIGHT {
            commands.entity(entity).despawn();
        }
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &TextureAtlasSprite), With<Player>>,
) {
    let (mut player_translation, sprite) = query.single_mut();
    let (left_bound, right_bound) = calculate_bounds(&player_translation, sprite.custom_size);

    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A])
        && left_bound > -WORLD_WIDTH / 2.0
    {
        player_translation.translation.x -= PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D])
        && right_bound < WORLD_WIDTH / 2.0
    {
        player_translation.translation.x += PLAYER_SPEED * time.delta().as_secs_f32();
    }
}

fn calculate_bounds(transform: &Transform, size: Option<Vec2>) -> (f32, f32) {
    let left_bound = transform.translation.x - size.unwrap_or_default().x / 2.;
    let right_bound = transform.translation.x + size.unwrap_or_default().x / 2.;
    (left_bound, right_bound)
}

fn gravity_system(mut query: Query<(&mut Velocity, &mut Gravity), With<Enemy>>) {
    for (mut velocity, gravity) in query.iter_mut() {
        velocity.y -= gravity.0;
    }
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Shadow Dancer".to_string(),
            width: 1600.0,
            height: 900.0,
            ..default()
        })
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, true)))
        .insert_resource(EnemyCount(0))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(enemy_spawner)
        .add_system(enemy_movement)
        .add_system(gravity_system)
        .run();
}
