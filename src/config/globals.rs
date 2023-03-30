// globals is a module that contains global variables that are used throughout the application

// use::std::sync::Mutex;  // Mutex is a type that allows only one thread to access the data at a time
// lazy_static! {  
    //     pub static ref GLOBALS: Mutex<Globals> = Mutex::new(Globals::new());
    // }



//? App Data ----------------------------------------------------------------------------------------------------------------------
pub const APP_NAME: &str = "Space Flight";
pub const APP_VERSION: &str = "v0.1.2";
pub const APP_AUTHOR: &str = "Yrrrrrf";


//? PATHS -------------------------------------------------------------------------------------------------------------------------
pub const ASSETS_PATH: &str = "assets/";
pub const IMG_PATH: &str = "assets/images/";
pub const FONT_PATH: &str = "assets/fonts/";
pub const SOUND_PATH: &str = "assets/sounds/";


//? Window Data -------------------------------------------------------------------------------------------------------------------
pub const WIDTH: f32 = 720.0;
pub const HEIGHT: f32 = 480.0;

