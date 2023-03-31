/* 
    This file contains the Structs and Enums used in the game
    - entity
        - player
        - enemy
*/


pub trait Entity {
    fn new() -> Self;  // instantiate a new entity
    fn update(&mut self);  // update the entity
    fn draw(&mut self);  // draw the entity
}


#[derive(Debug)]  // Debug trait allows us to print the struct
pub struct Player {
    pub health: i8,  // -128 to 127
    pub shield: i8,
    pub weapon: i8,
    pub speed: f32,  // 32-bit float
}


#[derive(Debug)]  // Debug trait allows us to print the struct
pub struct Enemy {
    pub health: i8,
    pub shield: i8,
    pub weapon: i8,
    pub speed: f32,
}


impl Entity for Player {
    fn new() -> Self {  // instantiate a new player
        Self {
            health: 100,  // 100hp
            shield: 0,  // no shield
            weapon: 100,
            speed: 100.0,
        }
    }

    fn update(&mut self) {
        // update the player
    }

    fn draw(&mut self) {
        // draw the player
    }
}




// trait (interface)
// traits: a trait is a collection of methods that are defined for an unknown type: Self
// This type is the caller of the method (the one that is using the method)

// struct (class)
// structs: a struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group
// If you’re familiar with an object-oriented language, a struct is like an object’s data attributes