use bevy::prelude::*;

use crate::mods::resources::*;
use crate::mods::components::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_in_assets.in_set(StartupSet::LoadAssets));
    }
}

fn load_in_assets(
    mut commands:Commands,
    asset_server:Res<AssetServer>,
) {
    let sprite_assets: RGameAssets = load_assets(&asset_server);
    commands.insert_resource(sprite_assets);
    //commands.insert_resource(DebugBezier(false));
}

pub fn load_assets(asset_server:&Res<AssetServer>) -> RGameAssets {
    let paths = AssetPaths::new();
    let player_path = paths.player_path.clone();
    let missile_path = paths.missile_path.clone();
    let alien_path = paths.alien_path.clone();
    let explosion_path = paths.explosion_path.clone();
    let alien_missile = asset_server.load(format!("{}{}", alien_path, missile_path));

    RGameAssets {
        player: ShipAssets {
            name: "Player Fighter".to_string(),
            ship: [
                asset_server.load(format!("{}fighter/single.png", player_path)),
                asset_server.load(format!("{}fighter/double.png", player_path)),
            ],
            missile: asset_server.load(format!("{}{}", player_path, missile_path)),
        },
        aliens: Aliens { 
            alien_missile : alien_missile.clone(),
            bosconian : ShipAssets{
                name: "Bosconian".to_string(),
                ship: [
                    asset_server.load(format!("{}bosconian/1.png", alien_path)),
                    asset_server.load(format!("{}bosconian/2.png", alien_path)),
                ],
                missile: alien_missile.clone()
            },
            bumblebee : ShipAssets{
                name: "Bumblebee".to_string(),
                ship: [
                    asset_server.load(format!("{}bumblebee/1.png", alien_path)),
                    asset_server.load(format!("{}bumblebee/2.png", alien_path)),
                ],
                missile: alien_missile.clone()
            },
            butterfly : ShipAssets{
                name: "Butterfly".to_string(),
                ship: [
                    asset_server.load(format!("{}butterfly/1.png", alien_path)),
                    asset_server.load(format!("{}butterfly/2.png", alien_path)),
                ],
                missile: alien_missile.clone()
            },
            galaga : ShipAssets{
                name: "Galaga".to_string(),
                ship: [
                    asset_server.load(format!("{}galaga/normal/1.png", alien_path)),
                    asset_server.load(format!("{}galaga/normal/2.png", alien_path)),
                ],
                missile: alien_missile.clone()
            },
            damaged_galaga : ShipAssets{
                name: "Galaga".to_string(),
                ship: [
                    asset_server.load(format!("{}galaga/damaged/1.png", alien_path)),
                    asset_server.load(format!("{}galaga/damaged/2.png", alien_path)),
                ],
                missile: alien_missile.clone()
            },
            galaxian : ShipAssets{
                name: "Galaxian".to_string(),
                ship: [
                    asset_server.load(format!("{}galaxian/1.png", alien_path)),
                    asset_server.load(format!("{}galaxian/2.png", alien_path)),
                ],
                missile: alien_missile.clone()
            },
            scorpion : ShipAssets{
                name: "Scorpion".to_string(),
                ship: [
                    asset_server.load(format!("{}scorpion/1.png", alien_path)),
                    asset_server.load(format!("{}scorpion/2.png", alien_path)),
                ],
                missile: alien_missile
            },
        },
        explosions : [
            asset_server.load(format!("{}{}/1.png",player_path, explosion_path)),
            asset_server.load(format!("{}{}/2.png",player_path, explosion_path)),
            asset_server.load(format!("{}{}/3.png",player_path, explosion_path)),
            asset_server.load(format!("{}{}/4.png",player_path, explosion_path)),
            asset_server.load(format!("{}{}/5.png",player_path, explosion_path)),
        ],
    }
}


pub struct AssetPaths {
    pub alien_path:String,
    pub player_path:String,
    pub missile_path:String,
    pub explosion_path:String,
}

impl AssetPaths {
    pub fn new()->Self{
        Self {
            alien_path: "sprites/aliens/".to_string(),
            player_path: "sprites/player/".to_string(),
            missile_path: "missiles/1.png".to_string(),
            explosion_path: "explosions".to_string(), 
        }
    }
}