struct Person {
    name: String,
}

impl Person {
    fn print_name(&self) {
        println!("  Name:  {}", self.name)
    }
}

fn main() {
    let person = Person {
        name: String::from("Papito mi rey fiu fiu"),
    };

    println!("");
    person.print_name();
    println!("");
}
