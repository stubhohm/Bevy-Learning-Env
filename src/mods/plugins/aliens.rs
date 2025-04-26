use bevy::ecs::query;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::transform;

use crate::mods::resources::*;
use crate::mods::components::*;
use crate::mods::bezier_paths::*;
use crate::{WIDTH, HEIGHT, ALIEN_MISSILE_VELOCITY};

pub struct AliensPlugin;

impl Plugin for AliensPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_aliens.in_set(StartupSet::SpawnActors));
        //app.add_systems(Update, update_aliens);
    }
}

fn spawn_aliens(
    mut commands: Commands,
    game_assets: Res<RGameAssets>,
) {
        
    commands.spawn((
        SpriteBundle {
            texture: game_assets.aliens.bumblebee.ship[0].clone(),
            transform: Transform {
                translation: vec3(WIDTH/-2.0, 0.0, 0.0),
                scale: Vec3 { x: (3.0), y: (3.0), z: (0.0) },
                rotation:Quat::from_rotation_z(std::f32::consts::PI/ 2.0),
                ..default()
                },
            ..default()
        },
        CAlien,
        CShip,
        AlienStates::Entering,
        lower_left_looping_entry_path(),
    ));

}

fn update_aliens (
    time: Res<Time>,
    //mut commands: Commands,
    mut alien_query: Query<&mut Transform, (With<CAlien>, With<CShip>)>,
    //sprite_assets: Res<RGameAssets>,
    //images: Res<Assets<Image>>,
) {
    let oscillation_speed = 2.0;
    let amplitude = 100.0;
    let time = time.elapsed_seconds();

    let dx = 0.5;
    let dy = (time * oscillation_speed).cos() * amplitude;  
    let offset =   (time * oscillation_speed).sin() * amplitude;

    for mut transform in alien_query.iter_mut() {
        
        transform.translation.y = offset;
        transform.translation.x += dx;

        let rotation = Vec2::new(dx*-100.0, -1.0*dy).normalize_or_zero();
        println!("x:{}, y:{}", dx, dy);
        transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, rotation);
        
        if transform.translation.x > WIDTH/2.0 {
            transform.translation.x = -WIDTH/2.0
        }
    }
        
}
