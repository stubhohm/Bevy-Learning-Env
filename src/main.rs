use bevy::prelude::*;

mod mods;

use crate::mods::constants::*;
use crate::mods::resources::*;
use crate::mods::components::*;

use crate::mods::plugins::assets::*;
use crate::mods::plugins::bezier::*;
use crate::mods::plugins::player::*;
use crate::mods::plugins::aliens::*;
use crate::mods::plugins::missile::*;
use crate::mods::plugins::stage::*;


fn main () {
    App::new()
    .configure_sets(Startup,
        (StartupSet::LoadAssets, 
        StartupSet::SpawnActors.after(StartupSet::LoadAssets)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Rust Galaga".to_string(),
            resolution: (WIDTH, HEIGHT).into(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(RDebugBezier(true))
    .insert_resource(RLevel(1))
    .add_plugins(AssetPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(AliensPlugin)
    .add_plugins(MissilePlugin)
    .add_plugins(BezierCurvePlugin)
    .add_plugins(StagePlugin)
    .run();
}

