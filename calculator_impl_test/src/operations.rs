#[derive(Debug)]
pub struct Numbers {
    number1: f64,
    number2: f64,
}

pub trait Calculator {
    fn suma(&self) -> f64;
    fn resta(&self) -> f64;
    fn multiplo(&self) -> f64;
    fn division(&self) -> f64;
    fn modulo(&self) -> f64;
    fn new(number1: f64, number2: f64) -> Self;
}

impl Calculator for Numbers {
    fn suma(&self) -> f64 {
        println!(
            "Resultado de {:?} + {:?} : {:?}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 + self.number2)
        );
        println!("");
        self.number1 + self.number2
    }

    fn resta(&self) -> f64 {
        println!(
            "Resultado de {:?} - {:?} : {:?}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 - self.number2)
        );
        println!("");
        self.number1 - self.number2
    }

    fn multiplo(&self) -> f64 {
        println!(
            "Resultado de {:?} * {:?} : {:?}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 * self.number2)
        );
        println!("");
        self.number1 * self.number2
    }
    fn division(&self) -> f64 {
        println!(
            "Resultado de {:?} / {:?} : {:?}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 / self.number2)
        );
        println!("");
        println!(
            "dejando un resto de : {:?}",
            format!("{:?}", self.number1 % self.number2)
        );
        self.number1 / self.number2
    }

    fn modulo(&self) -> f64 {
        println!(
            "Resultado de {:?} % {:?} : {:?}",
            format!("{:.2}", self.number1),
            format!("{:.2}", self.number2),
            format!("{:.2}", self.number1 - self.number2)
        );
        println!("");
        self.number1 % self.number2
    }

    fn new(number1: f64, number2: f64) -> Self {
        Self { number1, number2 }
    }
}
