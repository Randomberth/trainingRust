mod operations;
use crate::operations::Relation;

use std::io;

fn main() {
    let _pacha: Vec<&str> = vec!["11", "13", "16", "20", "24", "28", "32", "36", "42"];
    let _plato = vec!["22", "36"];
    let mut _cohef: f64;

    println!("Numero de dientes del plato:");

    let mut input_plate_string = String::new();
    io::stdin()
        .read_line(&mut input_plate_string)
        .expect("Error");

    let mut _input_plate_number: u8 = input_plate_string.trim().parse().expect("Error");

    println!("Numero de dientes del pinhon:");

    let mut input_pinhon_string = String::new();
    io::stdin()
        .read_line(&mut input_pinhon_string)
        .expect("Error");

    let mut _input_pinhon_number: u8 = input_pinhon_string.trim().parse().expect("Error");

    println!();
    println!("{} : {}", _input_plate_number, _input_pinhon_number);

    //    let relation1 = operations::Relation::new("Placa1".to_string(), "Pinhon1".to_string());
    let relation1 =
        <Dientes as operations::Relation>::new("Placa1".to_string(), "Pinhon1".to_string());

    /*
    for element_plato in plato {
        println!("");
        println!("Para plato {:?}", element_plato);
        for element_pacha in pacha.iter() {
            cohef = element_plato.parse::<f64>().expect("Error")
                / element_pacha.parse::<f64>().expect("Error");
            println!("Pinhon :{:?}, Coheficiente :{:.2}", element_pacha, cohef);
        }
    }*/
}
