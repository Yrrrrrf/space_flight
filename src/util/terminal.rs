// ? util methods for (any) rust code

use::std::io;  // io library is part of the standard library (std)
use::std::io::Write;  // io library is part of the standard library (std) (Write trait)
use::std::io::Read;  // io library is part of the standard library (std) (Read trait)


fn ask_str() -> String {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input;
}


fn ask_int() -> i32 {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<i32>().unwrap();
}


fn ask_float() -> f32 {
    let mut input = String::new();  // String::new() is a constructor (used when you want to modify a string)
    print!("Enter something: ");
    io::stdout().flush().unwrap();  // Allows the print!() to be flushed to the console otherwise it will wait for the next println!()
    io::stdin().read_line(&mut input).unwrap();  // Read input from the console
    println!("You entered: {}", input.trim());  // Trim the input to remove the newline character
    return input.trim().parse::<f32>().unwrap();
}


// ? Terminal related scripts  ----------------------------------------------------------------------------------------------------


/// Clear and cursor position
pub(crate) fn clear() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}


/// Return a string with the color set
pub(crate) fn set_fg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        "red"   => return format!("\x1b[31m{}\x1b[0m", string),  // Red
        "green" => return format!("\x1b[32m{}\x1b[0m", string),  // Green
        "yellow" => return format!("\x1b[33m{}\x1b[0m", string),  // Yellow
        "blue" => return format!("\x1b[34m{}\x1b[0m", string),  // Blue
        "magenta" => return format!("\x1b[35m{}\x1b[0m", string),  // Magenta
        "cyan" => return format!("\x1b[36m{}\x1b[0m", string),  // Cyan
        "white" | _ => return format!("\x1b[37m{}\x1b[0m", string),  // White (default)
    }
}


/// Return a string with the foreground color set
pub(crate) fn set_bg(string: &str, fg: &str) -> String {  // Set background color
    match fg {  // Color in Terminal
        "red"   => return format!("\x1b[41m{}\x1b[0m", string),  // Red
        "green" => return format!("\x1b[42m{}\x1b[0m", string),  // Green
        "yellow" => return format!("\x1b[43m{}\x1b[0m", string),  // Yellow
        "blue" => return format!("\x1b[44m{}\x1b[0m", string),  // Blue
        "magenta" => return format!("\x1b[45m{}\x1b[0m", string),  // Magenta
        "cyan" => return format!("\x1b[46m{}\x1b[0m", string),  // Cyan
        "white" | _ => return format!("\x1b[47m{}\x1b[0m", string),  // White (default)
    }
}
