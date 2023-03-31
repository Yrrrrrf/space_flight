//! Weapons module
//! 
//! Contains the weapon types and the weapon struct
//! ALL weapons stats are defined here



use std::{collections::HashMap, fmt, fmt::Debug, default};

use crate::util::terminal;


#[derive(Default, Debug, Clone, PartialEq)]
pub struct Weapon {
    w_type: WeaponType,  // The type defines the weapon characteristics
    current_magazine: u16,  // max magazine size
}


impl Weapon {
    /// Create a new weapon
    /// 
    /// ### Parameters:
    /// - `w_type`: `WeaponType` - The type of the weapon
    /// 
    /// ### Returns:
    /// - `Weapon` - The new weapon
    pub fn new(weapon_type: &WeaponType) -> Weapon {
        return Weapon {
            w_type: weapon_type.clone(),
            current_magazine: weapon_type.get_magazine_size(),
        }
    }


    /// Set the magazine to the max
    /// 
    /// ### Parameters:
    /// - `self`: `&mut Self` - The weapon to reload
    pub fn reload(&mut self) {
        self.current_magazine = self.w_type.get_magazine_size();
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
        if self.current_magazine < self.w_type.get_magazine_size() {  // if the magazine is not full
            self.current_magazine += ammo as u16;  // add the ammo
            return true;  // return true
        }
        false  // return false
    }


    /// Shoot the weapon
    /// 
    /// ### Parameters:
    /// - `self`: `&mut Self` - The weapon to shoot
    pub fn shoot(&mut self) -> bool {
        if self.current_magazine > 0 {
            self.current_magazine -= 1;
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
        return self.current_magazine > 0
    }

}


// create the weapons enum
#[derive(Default, Clone, PartialEq)]
/// The different types of weapons
pub enum WeaponType {
    #[default]  // default weapon <Plasma>
    Plasma,  // Shots a plasma ball
    Laser,  // Most cadence weapon
    Rocket,  // Splash damage
    Missile,  // Rocket with guidance
    Railgun,  // Charge time
}


impl WeaponType {
    /// Get the stats of a weapon depending on its type
    /// 
    /// ### Stats:
    /// - damage: [u16]
    /// - cadence: [u16]
    /// - magazine_size: [u16]
    /// - reload_time: [u16]
    /// - *additional_stats (depends on weapon type)*
    /// 
    /// ### Parameters:
    /// - `self`: `&Self` - The weapon type
    /// 
    /// ### Returns:
    /// - [HashMap]<[String], [u16]> - The stats of the weapon
    pub fn get_stats(&self) -> HashMap<String, u16> {
        let mut stats: HashMap<String, u16> = HashMap::new();
        stats.insert("damage".to_string(), self.get_damage());
        stats.insert("cadence".to_string(), self.get_cadence());
        stats.insert("magazine_size".to_string(), self.get_magazine_size());
        stats.insert("reload_time".to_string(), self.get_reload_time());
        return stats;
    }
        
        
    /// Get the type of the weapon as a string
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


    //? getters ---------------------------------------------------------------

    /// Get the damage of the weapon (per bullet)
    fn get_damage(&self) -> u16 {
        match self {
            WeaponType::Plasma => 10,
            WeaponType::Laser => 1,
            WeaponType::Rocket => 100,
            WeaponType::Missile => 40,
            WeaponType::Railgun => 127,
            _ => 0,
        }
    }


    /// Get the cadence of the weapon (bullets per second)
    fn get_cadence(&self) -> u16 {
        match self {
            WeaponType::Plasma => 3,
            WeaponType::Laser => 32,
            WeaponType::Missile => 2,
            WeaponType::Rocket | WeaponType::Railgun => 1,
            _ => 0,
        }
    }


    /// Get the magazine size of the weapon
    fn get_magazine_size(&self) -> u16 {
        match self {
            WeaponType::Plasma => 21,
            WeaponType::Laser => 200,
            WeaponType::Rocket => 2,
            WeaponType::Missile => 5,
            WeaponType::Railgun => 1,
            _ => 0,
        }
    }


    /// Get the reload time of the weapon (in seconds)
    fn get_reload_time(&self) -> u16 {
        match self {
            WeaponType::Plasma => 3,
            WeaponType::Laser => 5,
            WeaponType::Rocket => 5,
            WeaponType::Missile => 5,
            WeaponType::Railgun => 30,
            _ => 0,
        }
    }

}


/// Implement the [`Debug`] trait for the [`WeaponType`] enum
impl fmt::Debug for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_str: String = String::from(&format!("Weapon {} {{\n", terminal::set_fg(&self.to_string(), "green")));
        self.get_stats().iter().for_each(|(key, value)| {debug_str += &format!("{:>16}: {:>4}\n", key, value);});
        write!(f, "{}}}", debug_str)
    }
}


/// Implement the [`Display`] trait for the [`WeaponType`] enum
impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", terminal::set_fg(&self.get_type(), "green"))
    }
}
