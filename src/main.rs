#![allow(dead_code)]  // Allow dead code (unused code)
#![allow(unused_imports)]  // Allow unused imports
#![allow(unused_variables)]  // Allow unused variables




//? Modules -----------------------------------------------------------------------------------------------------------------------
// extern

use bevy::prelude::*;
use bevy::prelude::Plugin;

use bevy::{  // Bevy game engine (https://bevyengine.org/)
    prelude::*,
    window::Window,
    window::PrimaryWindow,
    window::WindowRef,
    winit::WinitWindows,
    // WindowsDescriptor,
    winit::WinitPlugin,
    render::camera::Camera,
    render::camera::OrthographicProjection,

    diagnostic::FrameTimeDiagnosticsPlugin,  // Diagnostics
    diagnostic::LogDiagnosticsPlugin,  // Log the diagnostics
    diagnostic::EntityCountDiagnosticsPlugin,  // Entity count
};

use winit::window::{
    Icon,
    WindowId
};


mod config;  // App Data
use config::globals::*;

mod components;  // Components
use components::power_ups::*;
use components::entities::{
    Player,
    Enemy
};

mod util; // Terminal Utilities
use util::*;

use crate::components::entities::Entity;


//? Main --------------------------------------------------------------------------------------------------------------------------
fn main() {
    terminal::clear();
    print!("{} {}\n", (terminal::set_fg(APP_NAME, "green")), APP_VERSION);
    println!("Author: {}\n", terminal::set_fg(&APP_AUTHOR, "blue"));


    let app: App = App::new();  // Create a new app

    // let mut winit_plugin: WinitPlugin = WinitPlugin::default();
    let mut winit_plugin = WinitPlugin::build(self, &mut app);




    // App::new()  // Create a new app
    app
        .insert_resource(ClearColor(Color::rgb_u8(32, 32, 200)))  // Set the background color
        // .add_plugins(DefaultPlugins.set(WindowPlugin {  // Set the window plugin
        //     primary_window: Some(Window {
        //         title: APP_NAME.into(),
        //         resolution: (1280., 720.).into(),
        //         resizable: true,
        //         ..Default::default()  // Set the rest of the window descriptor
        //     }),
        //     ..Default::default()  // Set the rest of the window plugin
        // }))
        .add_plugins(DefaultPlugins.set(winit_plugin))  // Set the window plugin
        .add_startup_system(setup)  // Setup the app
        .run();  // Run the app (main window)

}


//? Functions ---------------------------------------------------------------------------------------------------------------------
/// Set the `main window`.
/// 
/// Set Icon, Camera, etc.
fn setup(mut commands: Commands, windows: NonSend<WinitWindows>) {
    //* Set Camera
    commands.spawn(Camera2dBundle::default());  // Spawn the camera

    //* Set Icon
    let (icon_rgba, icon_width, icon_height) = {  // Get the icon data
        let image = image::open(format!("{}spaceship.png", IMG_PATH))  // Open the icon image
        .expect(&terminal::set_fg("File not found", "red"))  // red message if the icon path is wrong
        .into_rgba8();  // Convert the image to RGBA8
        let (width, height) = image.dimensions();  // Get the image dimensions
        let rgba = image.into_raw();  // Get the image raw data
        (rgba, width, height)  // Return the image data
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();  // Create the icon


    // let primary = windows.get_window(WindowId::primary()).unwrap();
    // let primary = windows.get_window().unwrap();  // Get the primary window
    // primary.set_window_icon(Some(icon));  // Set the icon

    set_scene(commands);  // Set the environment
    // set deb_mode(commands);  // Set the debug mode (diagnostics, etc.)



    // ENTITIES
    set_game();  // Set the game (entities, power ups, etc)
    // Unique id attached to Component

    // COMPONENTS
    // Data of the App (instances, etc.)
    // set_components(commands);  // Set the components

    // SYSTEMS
    // Functions that interact with the components (interact with the data)
    // set_systems(commands);  // Set the systems

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
    let mut player = Player::new();
    player.update() ;
    // println!("{:#?}", player);

    // Spawn the player



}