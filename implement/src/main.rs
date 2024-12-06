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
        false
    }

    fn print_name(&self) {
        println!("  Saludo:  {}", self.name)
    }

    fn print_profession(&self) {
        println!("  a quien:  {}", self.profession)
    }

    fn new(name: String, profession: String) -> Self {
        Self { name, profession }
    }
}

fn main() {
    let person = Person::new(String::from("Hola"), String::from("Mundo"));

    println!("");
    person.print_name();
    println!("");
    person.print_profession();
    println!("");
    println!("Do you learn? {}", person.is_human());
}
