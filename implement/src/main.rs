struct Person {
    name: String,
    profession: String,
}

trait Human {
    fn is_human(&self) -> bool;
    fn print_name(&self);
    fn print_profession(&self);
    fn new(name: String, profession: String) -> Self;
}

impl Human for Person {
    fn is_human(&self) -> bool {
        true
    }

    fn print_name(&self) {
        println!("  Name:  {}", self.name)
    }

    fn print_profession(&self) {
        println!("  Profession:  {}", self.profession)
    }

    fn new(name: String, profession: String) -> Self {
        Self { name, profession }
    }
}

fn main() {
    let person = Person::new(String::from("Randomberth"), String::from("AliAli"));

    println!("");
    person.print_name();
    println!("");
    person.print_profession();
    println!("");
    println!("Is person human? {}", person.is_human());
}
