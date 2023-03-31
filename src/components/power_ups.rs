//! PowerUps module
//! 
//! These items can be picked up by the entity struct to modify their stats


use std::{collections::HashMap, fmt, fmt::Debug};

use crate::util::terminal;


#[derive(Debug, Clone)]
pub struct PowerUP {
    power_up_type: PowerUpType,  // The type defines the powerup characteristics
    power: u8,  // the potency of the powerup
}


impl PowerUP {

    /// Create a new powerup
    /// 
    /// ### Parameters:
    /// - `power`: `u16` - The potency of the powerup
    /// 
    /// ### Returns:
    /// - `PowerUP` - The new powerup
    pub fn new(power_up: PowerUpType, power: u8) -> PowerUP {
        return PowerUP {
            power_up_type: power_up,
            power: power,  // the potency of the powerup
        }
    }

}


#[derive(Debug, Clone)]
/// The type of the powerup
pub enum PowerUpType {
    Health,
    Shield,
    Speed,
    Damage,
}


impl PowerUpType {

    /// Get the type of the weapon as a string
    /// 
    /// ### Parameters:
    /// - `self`: `&Self` - The weapon type
    /// 
    /// ### Returns:
    /// - `String` - The type of the weapon
    fn get_type(&self) -> String {
        match self {
            PowerUpType::Health => "Health".to_string(),
            PowerUpType::Shield => "Shield".to_string(),
            PowerUpType::Damage => "Damage".to_string(),
            PowerUpType::Speed => "Speed".to_string(),
        }
    }

}

