#![allow(dead_code)]

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;

mod camera;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainGame,
    MainMenu,
    ControlMenu,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.3, 0.3)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1280.0,
                height: 720.0,
                title: "CFB Manager".to_string(),
                mode: WindowMode::Windowed,
                present_mode: bevy::window::PresentMode::Immediate,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        // Add the camera as a startup system.
        .add_startup_system(camera::spawn_ui_camera)
        // Add the starting state. We want the user to start at the main menu.
        .add_state(GameState::MainMenu)
        .run();
}


fn player_testing() {
    let mut quinn = player::Player {
        name: "Quinn Ewers".to_string(),
        age: 18,
        year: player::Year::Freshman,
        attributes: player::Attributes {
            throw_power: 12,
            throw_accuracy: 14,
        },
        redshirt: true,
        redshirting: false,
    };
    println!("{:?}", quinn);

    let mut arch = player::Player::new(
        String::from("Arch Manning"),
        18,
        player::Year::Freshman,
        false,
    );
    println!("{:?}", arch);
    println!("{:?}", arch.year());

    let x = quinn.attributes.highest_attribute();
    println!("{:?}", quinn.year());
    println!("{:?}", x);

    quinn.advance_year();
    arch.advance_year();
    println!("{:?}", quinn.year());
    println!("{:?}", arch.year());
}
