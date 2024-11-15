extern crate termion;

use termion::{color, style};

fn main() {
    println!("{}Red", color::Fg(color::LightGreen));
    //println!("{}Blue", color::Fg(color::Blue));
    println!("{}Blue'n'Bold{}", style::Invert, style::Reset);
    println!("{}Just plain italic", style::Italic);
}
