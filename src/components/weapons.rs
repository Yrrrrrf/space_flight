/* 
Weapons

This file contains the weapons that the Entities can use
*/


// #[derive(Clone, Copy, Debug)]
//  implement the Sized trait
//  the Sized trait is automatically implemented on all types that have a known size at compile time
//  types that do not have a known size include slices and trait objects
//  the Sized trait is used to indicate that a type has a known size at compile time

use std::{collections::HashMap, fmt, fmt::Debug};

use crate::util::terminal;



#[derive(Debug, Clone)]
pub struct Weapon {
    w_type: WeaponType,  // The type defines the weapon characteristics
    damage:u16,  // damage per shot
    cadence: u16,  // shots per second
    magazine: u16, // current magazine
    magazine_size: u16,  // max magazine size
    reload_time: u16,  // time to reload
}


impl Weapon {
    /// Create a new weapon
    /// 
    /// ### Parameters:
    /// - `w_type`: `WeaponType` - The type of the weapon
    /// 
    /// ### Returns:
    /// - `Weapon` - The new weapon
    pub fn new(w_type: WeaponType) -> Weapon {
        let stats: HashMap<String, u16> = w_type.get_stats();
        return Weapon {
            w_type: w_type,
            damage: stats.get("damage").unwrap().clone(),
            cadence: stats.get("cadence").unwrap().clone(),
            magazine_size: stats.get("magazine_size").unwrap().clone(),
            magazine: stats.get("magazine_size").unwrap().clone(),
            reload_time: stats.get("reload_time").unwrap().clone(),
        }
    }


    /// Set the magazine to the max
    /// 
    /// ### Parameters:
    /// - `self`: `&mut Self` - The weapon to reload
    pub fn reload(&mut self) {
        self.magazine = self.magazine_size;
    }


    /// Add a certain amount of ammo to the magazine
    /// Only adds ammo if the magazine is not full
    /// 
    /// ### Parameters:
    /// - `self`: `&mut Self` - The weapon to reload
    /// - `ammo`: `i8` - The amount of ammo to add
    /// 
    /// ### Returns:
    /// - `bool` - True if the magazine was reloaded
    pub fn reload_ammo(&mut self, ammo: u8) -> bool {
        if self.magazine < self.magazine_size {  // if the magazine is not full
            self.magazine += ammo as u16;  // add the ammo
            return true;  // return true
        }
        false  // return false
    }


    /// Shoot the weapon
    /// 
    /// ### Parameters:
    /// - `self`: `&mut Self` - The weapon to shoot
    pub fn shoot(&mut self) -> bool {
        if self.magazine > 0 {
            self.magazine -= 1;
            return true;
        }
        false
    }


    /// Check if the weapon is loaded
    /// 
    /// ### Parameters:
    /// - `self`: `&Self` - The weapon to check
    /// 
    /// ### Returns:
    /// - `bool` - True if the weapon is loaded
    fn is_loaded(&self) -> bool {
        return self.magazine > 0
    }

}


// create the weapons enum
#[derive(Debug, Clone)]
pub enum WeaponType {
    Plasma,  // Default weapon
    Laser,  // Most cadence weapon
    Railgun,  // Charge time
    Rocket,  // Splash damage
    Missile,  // Rocket with guidance
}


impl WeaponType {
    /// Get the stats of a weapon depending on its type
    /// 
    /// ### Parameters:
    /// - `self`: `&Self` - The weapon type
    /// 
    /// ### Returns:
    pub fn get_stats(&self) -> HashMap<String, u16> {
        let mut stats: HashMap<String, u16> = HashMap::new();
        match self {
        _ | WeaponType::Plasma => {  // default weapon
            stats.insert("damage".to_string(),          10);
            stats.insert("cadence".to_string(),          3);
            stats.insert("magazine_size".to_string(),   21);
            stats.insert("reload_time".to_string(),      6);
        },
        WeaponType::Laser => {
            stats.insert("damage".to_string(),           1);
            stats.insert("cadence".to_string(),         40);
            stats.insert("magazine_size".to_string(),  200);
            stats.insert("reload_time".to_string(),      3); 
            },
        }
        return stats;
    }


    /// 
    /// ### Parameters:
    /// - `self`: `&Self` - The weapon type
    /// 
    /// ### Returns:
    /// - `String` - The type of the weapon
    fn get_type(&self) -> String {
        match self {
            WeaponType::Plasma => "Plasma".to_string(),
            WeaponType::Laser => "Laser".to_string(),
            WeaponType::Railgun => "Railgun".to_string(),
            WeaponType::Rocket => "Rocket".to_string(),
            WeaponType::Missile => "Missile".to_string(),
        }
    }


}


// implement the Display trait for the WeaponType enum
impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let mut debug_str: String = String::from(&format!("{} {{\n", terminal::set_fg(&terminal::get_type_of(&self).split("::").last().unwrap(), "green")));
        let mut debug_str: String = String::from(&format!("{} {{\n", terminal::set_fg(&self.get_type(), "green")));
        // let mut debug_str: String = String::from(&format!("{} {{\n", terminal::set_fg(&), "green")));
        self.get_stats().iter().for_each(|(key, value)| {debug_str += &format!("    {:>13}: {:>3}\n", key, value);});
        write!(f, "{}}}", debug_str)
    }
}
