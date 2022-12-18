use crate::{assets::GameAssets, prelude::*};

pub struct CollisionPlugin;
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(GameSystemLabel::Core)
                .with_system(collision_system),
        );
    }
}
pub fn collision_system(
    mut commands: Commands,
    shuriken_query: Query<(Entity, &Transform, &HitBox), (With<Shuriken>, Without<MarkDespawn>)>,
    mut enemy_query: Query<
        (Entity, &Transform, &HitBox, &mut Enemy),
        (With<Enemy>, Without<MarkDespawn>),
    >,
    mut sfx_events: EventWriter<SFXEvents>,
    game_assets: Res<GameAssets>,
) {
    for (shuriken_entity, shuriken_transform, shuriken_hitbox) in shuriken_query.iter() {
        let shurkien_scale = shuriken_transform.scale.xy();

        for (enemy_entity, enemy_transform, enemy_hitbox, mut enemy) in enemy_query.iter_mut() {
            let enemy_scale = enemy_transform.scale.xy();

            let collision = collide(
                shuriken_transform.translation,
                shuriken_hitbox.0 * shurkien_scale,
                enemy_transform.translation,
                enemy_hitbox.0 * enemy_scale,
            );

            if collision.is_some() {
                sfx_events.send(SFXEvents::CollisionSound);

                commands.entity(enemy_entity).insert(MarkDespawn);
           
                commands.entity(shuriken_entity).insert(MarkDespawn);
           

                commands.spawn(DeathEffectBundle::new(
                    &game_assets,
                    enemy_transform.translation,
                ));

                sfx_events.send(SFXEvents::DeathSound);

                enemy.0 = EnemyState::Dead;
            }
        }
    }
}
