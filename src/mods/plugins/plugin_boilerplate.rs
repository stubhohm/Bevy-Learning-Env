use bevy::prelude::*;

use crate::resources::*;
use crate::components::*;


pub struct PluginName;

impl Plugin for PluginName {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
        .add_systems(Update, (update_people, greet_people).chain());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((cPerson, cName("First Character".to_string())));
    commands.spawn((cPerson, cName("Second Character".to_string())));
    commands.spawn((cPerson, cName("Third Character".to_string())));
    commands.spawn((cPerson, cName("Fourth Character".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<rGreetTimer>, query: Query<&cName, With<cPerson>>) {
    if timer.0.tick(time.delta()).just_finished() {    
        for name in &query {
            println!("Hello {}", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut cName, With<cPerson>>) {
    for mut name in &mut query {
        if name.0 == "First Character" {
            name.0 = "Updated First Character".to_string();
            break;
        }
    }
}
