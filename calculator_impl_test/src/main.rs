use crate::operations::Calculator;
use crate::operations::Numbers;
use std::io;

mod operations;
fn main() {
    println!("1.- Suma");
    println!("2.- Resta");
    println!("3.- Multiplicación");
    println!("4.- División");
    println!("5.- Módulo");
    println!("");
    println!("Seleccione una opción");
    println!("");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error en la entrada");

    let input_number: i32 = input.trim().parse().expect("error");

    fn option_oper(val: i32) -> bool {
        val >= 1 && val <= 5
    }

    let numbers: Numbers = operations::Numbers::new(15.0, 5.0);

    if option_oper(input_number) {
        match input_number {
            1 => {
                println!("");
                numbers.suma();
            }
            2 => {
                println!("");
                numbers.resta();
            }
            3 => {
                println!("");
                numbers.multiplo();
            }
            4 => {
                println!("");
                numbers.division();
            }
            5 => {
                println!("");
                numbers.modulo();
            }

            _ => println!("Opción inválida"),
        }
    } else {
        println!("");
        println!("Opción inválida");
        println!("");
    }
}

mod test;
