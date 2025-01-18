mod operations;
use operations::*;

use std::io;

fn main() {
    println!("Cantidad de platos:");

    let mut input_plate_string = String::new();
    io::stdin()
        .read_line(&mut input_plate_string)
        .expect("Error");

    let input_plate_number: u8 = input_plate_string.trim().parse().expect("Error");

    println!("Numero de pinhones:");

    let mut input_pinhon_string = String::new();
    io::stdin()
        .read_line(&mut input_pinhon_string)
        .expect("Error");

    let input_pinhon_number: u8 = input_pinhon_string.trim().parse().expect("Error");

    println!("");
    println!(
        "{} : {}",
        input_plate_string.trim(),
        input_pinhon_string.trim()
    );

    let mut relacion_p_p: Relation = Relation {
        plate: Vec::new(),
        pinhon: Vec::new(),
    };

    for element_plato in 0..input_plate_number {
        println!("Ingrese cantidad de dientes plato {}", (element_plato + 1));
        let mut input_dent_string = String::new();
        io::stdin()
            .read_line(&mut input_dent_string)
            .expect("Error");

        relacion_p_p.new_plate(input_dent_string.trim().to_string());
    }
    println!("");
    for element_pinhon in 0..input_pinhon_number {
        println!(
            "Ingrese cantidad de dientes pinhon {}",
            (element_pinhon + 1)
        );
        let mut input_dent_string = String::new();
        io::stdin()
            .read_line(&mut input_dent_string)
            .expect("Error");

        relacion_p_p.new_pinhon(input_dent_string.trim().to_string());
    }

    relacion_p_p.show_relation();
}

/*    let _pacha: Vec<&str> = vec!["11", "13", "16", "20", "24", "28", "32", "36", "42"];
let _plato = vec!["22", "36"];
let mut _cohef: f64;


*/

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
