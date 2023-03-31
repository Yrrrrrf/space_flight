#![allow(dead_code)]  // Allow dead code (unused code)
#![allow(unused)]  // Allow unused imports


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

mod config;  // App Data
use config::globals::*;

mod util; // Terminal Utilities
use util::*;

mod components;
use components::{app::run_app, entities::*, weapons::*, power_ups::*};


//? Main --------------------------------------------------------------------------------------------------------------------------

fn main() {
    // terminal::clear();
    print!("{} {}\n", (terminal::set_fg(APP_NAME, "green")), APP_VERSION);
    println!("Author: {}\n", terminal::set_fg(&APP_AUTHOR, "blue"));

    // run_app();  // Run the app




    // println!("{}", WeaponType::Plasma);
    println!("{:#?}", WeaponType::Laser);

}
