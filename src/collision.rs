use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::collide};

use crate::components::{Enemy, HitBox, Shuriken};

pub fn collision_system(
    mut commands: Commands,
    shuriken_query: Query<(Entity, &Transform, &HitBox), With<Shuriken>>,
    enemy_query: Query<(Entity, &Transform, &HitBox), With<Enemy>>,
) {
    for (shuriken_entity, shuriken_transform, shuriken_hitbox) in shuriken_query.iter() {
        let shurkien_scale = Vec2::from(shuriken_transform.scale.xy());

        for (enemy_entity, enemy_transform, enemy_hitbox) in enemy_query.iter() {
            let enemy_scale = Vec2::from(enemy_transform.scale.xy());

            let collision = collide(
                shuriken_transform.translation,
                shuriken_hitbox.0 * shurkien_scale,
                enemy_transform.translation,
                enemy_hitbox.0 * enemy_scale,
            );

            if let Some(_) = collision {
                commands.entity(enemy_entity).despawn();

                commands.entity(shuriken_entity).despawn();
            }
        }
    }
}
