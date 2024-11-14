use std::io;

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
        Ok(num) => num,
        Err(_) => {
            println!("Opcion invalida");
            return;
        }
    };

    match op {
        1 => {
            println!("");
            println!("Suma     : opcion 1");
        }
        2 => {
            println!("");
            println!("Resta    : opcion 2");
        }
        3 => {
            println!("");
            println!("Producto : opcion 3");
        }
        4 => {
            println!("");
            println!("Dividir  : opcion 4");
        }
        _ => {
            println!("");
            println!("Opcion invalida");
        }
    }
}
