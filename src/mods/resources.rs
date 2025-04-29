use bevy::prelude::*;

#[derive(Resource)]
pub struct RGreetTimer(pub Timer);

#[derive(Resource, Deref, DerefMut)]
pub struct RScore(pub i8);


#[derive(Resource)]
pub struct RDebugBezier(pub bool);

#[derive(Resource)]
pub struct RLevel (pub i64);

#[derive(Resource)]
pub struct RGameAssets {
    pub player: ShipAssets,
    pub aliens: Aliens,
    pub explosions: [Handle<Image>; 5],
}

pub struct ShipAssets{
    pub name: String,
    pub ship: [Handle<Image>; 2],
    pub missile: Handle<Image>,
}

pub struct Aliens{
    pub bosconian: ShipAssets,
    pub bumblebee: ShipAssets,
    pub butterfly: ShipAssets,
    pub galaga: ShipAssets,
    pub damaged_galaga: ShipAssets,
    pub galaxian: ShipAssets,
    pub scorpion: ShipAssets,
    pub alien_missile: Handle<Image>,
}