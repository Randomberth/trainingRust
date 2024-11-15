use std::io;
//use std::io::{self, stdout, Write};
//use termion::cursor;
//use termion::raw::IntoRawMode;

fn main() {
    println!("");
    println!("Seleccione una opcion :");

    println!("");
    println!("Suma     : opcion 1");
    println!("Resta    : opcion 2");
    println!("Producto : opcion 3");
    println!("Dividir  : opcion 4");
    println!("");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Fallo de lectura");

    let op: u32 = match input.trim().parse() {
        Ok(num) if num > 4 => {
            println!("");
            println!("Opción inválida");
            println!("");
            return;
        }
        Ok(num) => num,
        Err(_) => {
            println!("");
            println!("Opcion invalida");
            println!("");
            return;
        }
    };

    //    let mut stdout = stdout().into_raw_mode().unwrap();

    println!("");
    println!("Ingrese primer valor :");

    /*    write!(stdout, "Ingrese primer valor : ").unwrap();
       stdout.flush().unwrap();
       write!(stdout, "{}", cursor::Goto(18, 1)).unwrap();
    */
    let mut val1 = String::new();
    std::io::stdin()
        .read_line(&mut val1)
        .expect("Fallo de lectura");

    let number1: i32 = match val1.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor ingrese un número");
            println!("");
            return;
        }
    };

    println!("Ingrese segundo valor :");
    let mut val2 = String::new();
    io::stdin().read_line(&mut val2).expect("Fallo de lectura");

    let number2: i32 = match val2.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor ingrese un número");
            println!("");
            return;
        }
    };

    match op {
        1 => {
            let response: i32 = number1 + number2;
            println!("");
            println!("La suma de {number1} + {number2} es : {response}");
            println!("");
        }
        2 => {
            let response: i32 = number1 - number2;
            println!("");
            println!("La resta de {number1} - {number2} es : {response}");
            println!("");
        }
        3 => {
            let response: i32 = number1 * number2;
            println!("");
            println!("El producto de {number1} * {number2} es : {response}");
            println!("");
        }
        4 => {
            let response: i32 = number1 / number2;
            let resto: i32 = number1 % number2;
            println!("");
            println!("La división de {number1} / {number2} es : {response}");
            println!("Dejando un resto de : {resto}");
            println!("");
        }
        _ => {
            println!("");
            println!("Opcion invalida");
        }
    }
}
