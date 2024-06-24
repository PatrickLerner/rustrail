#[cfg(test)]
mod tests;

use super::Landscape;
use bevy::prelude::*;

#[cfg_attr(coverage, allow(unused_variables))]
pub fn system(
    mut commands: Commands,
    mut landscapes: Query<(Entity, &mut Landscape, &Transform)>,
    time: Res<Time>,
) {
    for (entity, mut landscape, transform) in landscapes.iter_mut() {
        landscape.ttl -= time.delta_seconds();
        if landscape.ttl <= 0.0 {
            commands.entity(entity).despawn_recursive();

            #[cfg(not(coverage))]
            log::debug!(
                "Despawning landscape at {:?}",
                (
                    transform.translation.x as i32,
                    transform.translation.z as i32,
                )
            );
        }
    }
}
