#![allow(clippy::type_complexity)]
use audio::{GameAudioPlugin, SFXEvents};
use background::*;
use bevy::{prelude::*, render::camera::ScalingMode};
use components::*;
use constants::*;
use death_effect::death_effect_animator;
use enemy::{enemy_animator, EnemyBundle};
use player::{player_attacking_system, PlayerBundle};
use shuriken::{shuriken_animator, shuriken_movement};
use walls::{spawn_walls, wall_animator};

#[derive(Resource)]
struct EnemyCount(u32);

#[derive(Resource)]
struct SpawnTimer(Timer);

struct Bounds {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
enum GameSystemLabel {
    Core,
    Cleanup,
}

// enum GameState {
//     Splash,
//     MainMenu,
//     InGame,
//     Paused,
// }

mod audio;
mod collision;
mod components;
mod constants;
mod death_effect;
mod enemy;
mod player;
mod shuriken;
mod walls;
mod background;
// mod settings;
// mod systems;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let half_height = WORLD_HEIGHT / 2.0;
    let half_width = WORLD_WIDTH / 2.0;

    let camera_bundle = Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.0,
            scaling_mode: ScalingMode::None,
            scale: 1.0,
            left: -half_width,
            right: half_width,
            top: half_height,
            bottom: -half_height,
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn(camera_bundle);

    spawn_player(&mut commands, asset_server, texture_atlases);
}

fn spawn_player(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_empty()
        .insert(PlayerBundle::new(asset_server, texture_atlases));
}

fn spawn_enemy(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_empty()
        .insert(EnemyBundle::pawn(asset_server, texture_atlases));
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
            &InitialEnemySpeed,
            &mut WallHangingTimer,
            &mut Enemy,
        ),
        With<Enemy>,
    >,
    mut commands: Commands,
) {
    for (
        entity,
        mut transform,
        mut velocity,
        initial_enemy_speed,
        mut wall_hanging_timer,
        mut enemy,
    ) in query.iter_mut()
    {
        let Bounds { right, left, .. } = calculate_bounds(&transform, None);

        let is_touching_left_bound = left < LEFT_WALL;
        let is_touching_right_bound = right > RIGHT_WALL;

        if (velocity.x < 0.0 && is_touching_left_bound)
            || (velocity.x > 0.0 && is_touching_right_bound)
        {
            if wall_hanging_timer.0.tick(time.delta()).just_finished() {
                velocity.x *= -1.0;
                velocity.y = initial_enemy_speed.0;
                enemy.0 = EnemyState::Airborne;
            } else {
                enemy.0 = EnemyState::WallHanging;
            }
        } else {
            transform.translation.y += velocity.y * time.delta().as_secs_f32();
            transform.translation.x += velocity.x * time.delta().as_secs_f32();
        }

        if transform.translation.y > (WORLD_HEIGHT / 2.0) + 100.0 && enemy.0 != EnemyState::Dead {
            commands.entity(entity).insert(MarkDespawn);
        }
    }
}

fn play_controls(
    keyboard_input: Res<Input<KeyCode>>,

    time: Res<Time>,
    mut query: Query<(&mut Player, &mut Transform, &TextureAtlasSprite), With<Player>>,
) {
    let (mut player, mut player_transform, sprite) = query.single_mut();
    let Bounds {
        top,
        right,
        bottom,
        left,
    } = calculate_bounds(&player_transform, sprite.custom_size);

    if keyboard_input.any_pressed(vec![KeyCode::Left, KeyCode::A]) && left > LEFT_WALL {
        player_transform.translation.x -= PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if keyboard_input.any_pressed(vec![KeyCode::Right, KeyCode::D]) && right < RIGHT_WALL {
        player_transform.translation.x += PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if keyboard_input.pressed(KeyCode::W) && top < TOP_WALL {
        player_transform.translation.y += PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if keyboard_input.pressed(KeyCode::S) && bottom > BOTTOM_WALL {
        player_transform.translation.y -= PLAYER_SPEED * time.delta().as_secs_f32();
    }

    if keyboard_input.just_pressed(KeyCode::Down) && player.0 == PlayerState::Falling {
        player.0 = PlayerState::Attacking;
    }
}

fn calculate_bounds(transform: &Transform, size: Option<Vec2>) -> Bounds {
    let left_bound = transform.translation.x - size.unwrap_or_default().x / 2.0;
    let right_bound = transform.translation.x + size.unwrap_or_default().x / 2.0;
    let top_bound = transform.translation.y + size.unwrap_or_default().y / 2.0;
    let bottom_bound = transform.translation.y - size.unwrap_or_default().y / 2.0;

    Bounds {
        top: top_bound,
        right: right_bound,
        bottom: bottom_bound,
        left: left_bound,
    }
}

fn gravity_system(mut query: Query<(&mut Velocity, &mut Gravity), With<Enemy>>) {
    for (mut velocity, gravity) in query.iter_mut() {
        velocity.y -= gravity.0;
    }
}

fn despawner(mut commands: Commands, query: Query<Entity, With<MarkDespawn>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1600.0,
                        height: 900.0,
                        position: WindowPosition::Centered,
                        monitor: MonitorSelection::Current,
                        title: "Shadow Dancer".to_string(),
                        mode: WindowMode::Windowed,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(GameAudioPlugin)
        .insert_resource(SpawnTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        .insert_resource(EnemyCount(0))
        .add_startup_system(setup)
        .add_event::<SFXEvents>()
        .add_startup_system(spawn_walls)
        .add_startup_system(spawn_day_background)
        .add_system(
            despawner
                .after(GameSystemLabel::Core)
                .label(GameSystemLabel::Cleanup),
        )
        .add_system(background_animator.label(GameSystemLabel::Core))
        .add_system(wall_animator.label(GameSystemLabel::Core))
        .add_system(play_controls.label(GameSystemLabel::Core))
        .add_system(enemy_spawner.label(GameSystemLabel::Core))
        .add_system(shuriken_movement.label(GameSystemLabel::Core))
        .add_system(shuriken_animator.label(GameSystemLabel::Core))
        .add_system(player_attacking_system.label(GameSystemLabel::Core))
        .add_system(enemy_animator.label(GameSystemLabel::Core))
        .add_system(enemy_movement.label(GameSystemLabel::Core))
        .add_system(gravity_system.label(GameSystemLabel::Core))
        .add_system(death_effect_animator.label(GameSystemLabel::Core))
        .run();
}
