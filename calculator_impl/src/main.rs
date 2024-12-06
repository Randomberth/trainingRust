use std::io;

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
            "Resultado de {} + {}: {}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 + self.number2)
        );
    }

    fn resta(&self) {
        println!(
            "Resultado de {} - {}: {}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 - self.number2)
        );
    }

    fn producto(&self) {
        println!(
            "Resultado de {} * {}: {}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 * self.number2)
        );
    }

    fn division(&self) {
        println!(
            "Resultado de {} / {}: {}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 / self.number2)
        );
        println!("dejando un resto de : {}", self.number1 % self.number2)
    }

    fn new(number1: f64, number2: f64) -> Self {
        Self { number1, number2 }
    }
}

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

    let mut r1 = String::new();
    let mut r2 = String::new();

    println!("imput : {}", input);

    println!("Ingrese primer numero :");
    io::stdin().read_line(&mut r1).expect("Fallo en lectura");

    let p1: f64 = match r1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada invalida");
            return;
        }
    };

    println!("Ingrese segundo numero :");
    io::stdin().read_line(&mut r2).expect("Fallo en lectura");

    let p2: f64 = match r2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada invalida");
            return;
        }
    };

    let _numbers: Numbers = Numbers::new(p1, p2);

    match op {
        1 => {
            println!("");
            _numbers.suma()
        }
        2 => {
            println!("");
            _numbers.resta()
        }
        3 => {
            println!("");
            _numbers.producto()
        }

        4 => {
            println!("");
            _numbers.division()
        }
        _ => println!("Otro número"),
    }
    println!("");
}
