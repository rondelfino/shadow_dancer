use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};

use crate::{
    audio::SFXEvents,
    components::{Enemy, EnemyState, HitBox, MarkDespawn, Shuriken},
    death_effect::DeathEffectBundle,
};

pub fn collision_system(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    shuriken_query: Query<(Entity, &Transform, &HitBox), (With<Shuriken>, Without<MarkDespawn>)>,
    mut enemy_query: Query<
        (Entity, &Transform, &HitBox, &mut Enemy),
        (With<Enemy>, Without<MarkDespawn>),
    >,
    mut sfx_events: EventWriter<SFXEvents>,
) {
    let texture_handle = asset_server.load("effects/death.png");

    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(40.0, 95.0), 4, 1, None, None);
    let death_effect_atlas_handle = texture_atlases.add(texture_atlas);

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

                commands.spawn_empty().insert(DeathEffectBundle::new(
                    death_effect_atlas_handle.clone(),
                    enemy_transform.translation,
                ));

                sfx_events.send(SFXEvents::DeathSound);

                enemy.0 = EnemyState::Dead;
            }
        }
    }
}
