


//? Modules -----------------------------------------------------------------------------------------------------------------------

use std::any::{self, Any};
use std::str::FromStr;

use bevy::prelude::*;
// use bevy::prelude::Plugin;

use bevy::{  // Bevy game engine (https://bevyengine.org/)
    prelude::*,
    window::Window,
    winit::WinitWindows,
    render::camera::Camera,
    render::camera::OrthographicProjection,


};

use winit::window::{
    Icon,
    WindowId
};

use crate::components::*;
use crate::components::entities::Entity;
use crate::config::globals::*;
use crate::util::terminal;


//? Main --------------------------------------------------------------------------------------------------------------------------
pub fn run_app() {

    App::new()  // Create a new app
        .insert_resource(ClearColor(Color::rgb_u8(32, 32, 200)))  // Set the background color
        .add_plugins(DefaultPlugins.set(WindowPlugin {  // Set the window plugin
            primary_window: Some(Window {  // Set the window descriptor
                title: APP_NAME.to_string(),  // Set the title
                resolution: (1280.0, 720.0).into(),  // Set the height
                ..Default::default()  // Set the rest of the window descriptor
            }),
            ..Default::default()  // Set the rest of the window plugin
        }))
        .add_startup_system(setup)  // Setup the app
        .run();  // Run the app (main window)

}


fn setup(mut commands: Commands) {
    let debug: bool = true;
    if debug {println!("{}", terminal::set_fg("Debug Mode", "blue"));}

    //* Set Camera
    commands.spawn(Camera2dBundle::default());  // Spawn the camera

    set_scene(commands);  // Set the environment
}


/// Set the `environment`.
/// (Background, etc.)
/// 
/// ### Parameters:
/// - `commands`: `Commands` - The commands to spawn the entities
fn set_scene(mut commands: Commands) {

    commands.spawn(SpriteBundle {
        sprite: Sprite{
            color: Color::rgb_u8(255, 255, 255),
            custom_size: Some(Vec2::new(128.0, 128.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}


/// Set all the stuff for the `game`.   
/// - entities, power ups, etc.
// fn set_game(mut commands: Commands) {
fn set_game() {
    // println!("{}", terminal::set_fg("Setting the game... ", "blue"));
    // Player Instance
    let mut player = entities::Player::new();
    player.update() ;
    println!("{:#?}", player);

}
