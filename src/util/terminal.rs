// ? util methods for (any) rust code

use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)
use::std::io::Read;
use std::str::FromStr;  // io library is part of the standard library (std) (Read trait)


// now using generics
pub (crate) fn ask<T: std::str::FromStr>() -> T where <T as FromStr>::Err: std::fmt::Debug {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<T>().unwrap();
}


pub (crate) fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub (crate) fn get_type_of<T>(_: &T) -> String {
    return std::any::type_name::<T>().to_string();
}


// ? Terminal related scripts  ----------------------------------------------------------------------------------------------------


/// Clear and cursor position
pub (crate) fn clear() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}


/// Return a string with the color set
pub (crate) fn set_fg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        "r" | "red"   => return format!("\x1b[31m{}\x1b[0m", string),  // Red
        "g" | "green" => return format!("\x1b[32m{}\x1b[0m", string),  // Green
        "y" | "yellow" => return format!("\x1b[33m{}\x1b[0m", string),  // Yellow
        "b" | "blue" => return format!("\x1b[34m{}\x1b[0m", string),  // Blue
        "m" | "magenta" => return format!("\x1b[35m{}\x1b[0m", string),  // Magenta
        "c" | "cyan" => return format!("\x1b[36m{}\x1b[0m", string),  // Cyan
        "w" | "white" | _ => return format!("\x1b[37m{}\x1b[0m", string),  // White (default)
    }
}


/// Return a string with the foreground color set
pub (crate) fn set_bg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        "r" | "red"   => return format!("\x1b[41m{}\x1b[0m", string),  // Red
        "g" | "green" => return format!("\x1b[42m{}\x1b[0m", string),  // Green
        "y" | "yellow" => return format!("\x1b[43m{}\x1b[0m", string),  // Yellow
        "b" | "blue" => return format!("\x1b[44m{}\x1b[0m", string),  // Blue
        "m" | "magenta" => return format!("\x1b[45m{}\x1b[0m", string),  // Magenta
        "c" | "cyan" => return format!("\x1b[46m{}\x1b[0m", string),  // Cyan
        "w" | "white" | _ => return format!("\x1b[47m{}\x1b[0m", string),  // White (default)
    }
}
