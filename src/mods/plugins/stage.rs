use bevy::prelude::*;
use bevy::render::texture::DepthAttachment;
use rand::Rng;

use crate::mods::constants::{HEIGHT, WIDTH};
use crate::mods::resources::*;
use crate::mods::components::*;


pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_stars)
        .add_systems(Update, move_stars);
    }
}

fn generate_stars(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let rows = 30;
    let columns = 50;
    let h_spacing = WIDTH / columns as f32;
    let v_spacing = HEIGHT / rows as f32;
    
    for row in 0..rows {
        for column in 0..columns {
            let depth = rng.gen_range(0.1..1.0);
            let spawn_check = rng.gen_range(0.1..1.0);
            if spawn_check > depth {
                continue;
            }

            let base_x = column as f32 * h_spacing;
            let base_y = row as f32 * v_spacing;

            let offset_x = rng.gen_range(-v_spacing/2.5..v_spacing/2.5);
            let offset_y = rng.gen_range(-h_spacing/2.5..h_spacing/2.5);
            let position = Vec3::new(base_x + offset_x - WIDTH / 2.0, base_y + offset_y - HEIGHT / 2.0, 0.0);

            

            let brightness = 1.0 - depth;
            let r = rng.gen_range(0.0..brightness);
            let g = rng.gen_range(0.0..brightness);
            let b = rng.gen_range(0.0..brightness);

            let color = Color::rgb(r, g, b);

            commands.spawn((
                SpriteBundle {
                    sprite : Sprite { 
                        color,
                        custom_size: Some(Vec2::splat(2.0)),
                        ..default()
                     },
                    transform: Transform::from_translation(position),
                    ..default() 
                },
                CStar { depth },
            ));
        }
    }
}

fn move_stars(
    time: Res<Time>,
    mut star_query: Query<(&CStar, &mut Transform, &mut Sprite)>,
    level: Res<RLevel>,
) {
    for (star, mut transform, mut sprite) in star_query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * (1.0 - star.depth) * (99.0 + 2.5 * level.0 as f32);
        let depth_scale = star.depth * 5.0;
        let alpha = (time.elapsed_seconds() * depth_scale).sin() * 0.5 + 0.5;
        sprite.color.set_a(alpha);

        if transform.translation.y < - HEIGHT / 2.0 {
            transform.translation.y = HEIGHT / 2.0;
        }
    }
}
