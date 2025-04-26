use bevy::prelude::*;

use crate::mods::constants::HEIGHT;
use crate::mods::constants::WIDTH;
use crate::mods::resources::*;
use crate::mods::components::*;


pub struct MissilePlugin;

impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_missiles);
    }
}

fn despawn_missile(
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity).despawn();
}

fn move_missiles (
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &CVelocity) , With<CMissile>>,
) {
    for (entity, mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
        
        if transform.translation.y > HEIGHT /2.0 || transform.translation.y < (HEIGHT * -0.5) {
            despawn_missile(&mut commands, entity);
        }
        if transform.translation.x > WIDTH / 2.0 || transform.translation.x < (WIDTH * -0.5) {
            despawn_missile(&mut commands, entity);
        }
    }
}