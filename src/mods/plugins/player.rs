use bevy::math::vec3;
use bevy::prelude::*;

use crate::mods::resources::*;
use crate::mods::components::*;
use crate::{WIDTH, HEIGHT, PLAYER_MISSILE_VELOCITY};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player.in_set(StartupSet::SpawnActors));
        app.add_systems(Update, update_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    game_assets: Res<RGameAssets>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: game_assets.player.ship[0].clone(),
            transform: Transform {
                translation: vec3(0.0, - HEIGHT *7.0/16.0, 0.0),
                scale: Vec3 { x: (3.0), y: (3.0), z: (0.0) },
                ..default()
                },
            ..default()
        },
        CPlayer,
        CShip,
    ));

}

fn move_player(
    keyboard_input: &Res<ButtonInput<KeyCode>>,
    transform: &mut Transform,
    size:f32,
) {
    let speed = 2.5;
    if keyboard_input.pressed(KeyCode::ArrowLeft) && (transform.translation.x - size / 2.0 > -WIDTH / 2.0) {
        transform.translation.x -= speed;
        
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) && (transform.translation.x + size / 2.0 < WIDTH / 2.0) {
        transform.translation.x += speed;
    }
}

fn fire_player_missile(
    commands: &mut Commands,
    player_transform: &Transform,
    sprite_assets: &Res<RGameAssets>,
) {
    commands.spawn((
        SpriteBundle {
            texture: sprite_assets.player.missile.clone(),
            transform: Transform { 
                translation : player_transform.translation + Vec3::Y * 20.0,
                scale: Vec3 { x: (3.0), y: (3.0), z: (0.0) },
                ..default()
            },
            ..default()
        },
        CMissile,
        CPlayer,
        CVelocity { x: 0.0, y: PLAYER_MISSILE_VELOCITY},
    ));  

}

fn update_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<CPlayer>, With<CShip>)>,
    player_missiles: Query<&CMissile, With<CPlayer>>,
    sprite_assets: Res<RGameAssets>,
    images: Res<Assets<Image>>,
    mut debug: ResMut<RDebugBezier>,   
) {
    if let Some(mut transform) = player_query.iter_mut().next() {

        if keyboard_input.just_pressed(KeyCode::KeyD) {
            debug.0 = !debug.0;
            println!("Debug draw: {}", debug.0);
        }   

        if keyboard_input.just_pressed(KeyCode::Space) && player_missiles.iter().count()< 2 {
            fire_player_missile(&mut commands, &transform, &sprite_assets);
        }

        if let Some(size) = images.get(sprite_assets.player.ship[0].clone()) {
            if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::ArrowRight) {
                move_player(&keyboard_input, &mut transform, size.size().x as f32);
            }
        } else {
            eprintln!("Player sprite asset not found!");
        }
    } else {
        eprintln!("Player ship not found in query!");
    }
}


