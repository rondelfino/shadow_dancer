use rand::Rng;

use crate::{assets::GameAssets, prelude::*};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::InGame).with_system(collision_system));
    }
}

pub fn collision_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    shuriken_query: Query<
        (Entity, &Transform, &HitBox),
        (With<Shuriken>, Without<MarkDespawn>, Without<Reflected>),
    >,
    mut enemy_query: Query<
        (Entity, &Transform, &HitBox, &mut Enemy, &ReflectChance),
        (With<Enemy>, Without<MarkDespawn>),
    >,
    mut sfx_events: EventWriter<SFXEvents>,

    game_assets: Res<GameAssets>,
) {
    let player_transform = player_query.single();

    for (shuriken_entity, shuriken_transform, shuriken_hitbox) in shuriken_query.iter() {
        let shurkien_scale = shuriken_transform.scale.xy();

        for (enemy_entity, enemy_transform, enemy_hitbox, mut enemy, reflect_chance) in
            enemy_query.iter_mut()
        {
            let random_number = rand::thread_rng().gen_range(0.0..=1.0);

            let enemy_scale = enemy_transform.scale.xy();

            let collision = collide(
                shuriken_transform.translation,
                shuriken_hitbox.0 * shurkien_scale,
                enemy_transform.translation,
                enemy_hitbox.0 * enemy_scale,
            );

            if collision.is_some() {
                let angle = f32::atan2(
                    player_transform.translation.y - shuriken_transform.translation.y,
                    player_transform.translation.x - shuriken_transform.translation.x,
                );

                if random_number < reflect_chance.0 && enemy.0 == EnemyState::Airborne {
                    commands.entity(shuriken_entity).insert(Reflected(angle));
                } else if enemy.0 == EnemyState::WallHanging {
                    commands.entity(shuriken_entity).insert(Reflected(angle));
                } else if enemy.0 == EnemyState::Airborne && random_number > reflect_chance.0 {
                    sfx_events.send(SFXEvents::CollisionSound);

                    commands.entity(enemy_entity).insert(MarkDespawn);

                    commands.entity(shuriken_entity).insert(MarkDespawn);

                    commands.spawn(DeathEffectBundle::new(
                        &game_assets,
                        enemy_transform.translation,
                    ));
                    println!("Entity {:?} died.", enemy_entity);
                    sfx_events.send(SFXEvents::DeathSound);

                    enemy.0 = EnemyState::Dead;
                }
            }
        }
    }
}
