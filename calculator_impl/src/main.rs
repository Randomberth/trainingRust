struct Numbers {
    number1: f64,
    number2: f64,
}

trait Calculator {
    fn suma(&self);
    fn resta(&self);
    fn producto(&self);
    fn division(&self);
    fn new(number1: f64, number2: f64) -> Self;
}

impl Calculator for Numbers {
    fn suma(&self) {
        println!(
            "Resultado : {}",
            format!("{:.2}", self.number1 + self.number2)
        );
    }

    fn resta(&self) {
        println!(
            "Resultado : {}",
            format!("{:.2}", self.number1 - self.number2)
        );
    }

    fn producto(&self) {
        println!(
            "Resultado : {}",
            format!("{:.2}", self.number1 * self.number2)
        );
    }

    fn division(&self) {
        println!(
            "Resultado : {}",
            format!("{:.2}", self.number1 / self.number2)
        );
        println!("Resto     : {}", self.number1 % self.number2)
    }

    fn new(number1: f64, number2: f64) -> Self {
        Self { number1, number2 }
    }
}

fn main() {
    let _numbers: Numbers = Numbers::new(10.0, 3.0);
    _numbers.suma();
    _numbers.resta();
    _numbers.producto();
    _numbers.division();
}

/*
use std::io;





fn main() {
    println!("");
    println!("Seleccione una opción");

    println!("");
    println!("Suma      : opcion 1");
    println!("Resta     : opcion 2");
    println!("Producto  : opcion 3");
    println!("Dividir   : opcion 4");
    println!("");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fallo de Lectura");

    let op: u32 = match input.trim().parse() {
        Ok(num) if num > 4 => {
            println!("");
            println!("Opción NO válida");
            println!("");
            return;
        }
        Ok(num) => num,
        Err(_) => {
            println!("");
            println!("Opción NO válida");
            println!("");
            return;
        }
    };

    println!("imput : {}", input);
}
*/
