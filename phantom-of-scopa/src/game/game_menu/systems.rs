use super::components::*;
use bevy::prelude::*;

pub fn setup_menu(mut commands: Commands) {
    println!("Entered in-game menu!");
    commands.spawn((
        InGameMenuUI,
        NodeBundle {
            style: Style { ..default() },
            ..default()
        },
    ));
}

pub fn teardown_menu() {
    println!("Exiting in-game menu :(");
}
