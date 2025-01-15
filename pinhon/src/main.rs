fn main() {
    let pacha: Vec<&str> = vec!["11", "13", "16", "20", "24", "28", "32", "36", "42"];
    let plato = vec!["22", "36"];
    let mut cohef: f64;

    for element_plato in plato {
        println!("");
        println!("Para plato {:?}", element_plato);
        for element_pacha in pacha.iter() {
            cohef = element_plato.parse::<f64>().expect("Error")
                / element_pacha.parse::<f64>().expect("Error");
            println!("Pinhon :{:?}, Coheficiente :{:.2}", element_pacha, cohef);
        }
    }
}
