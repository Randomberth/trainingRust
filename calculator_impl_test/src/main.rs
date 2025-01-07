use crate::operations::Calculator;
use crate::operations::Numbers;

mod operations;
fn main() {
    let numbers: Numbers = operations::Numbers::new(5.0, 3.0);
    println!("");
    println!("La suma es     : {}", numbers.suma());
    println!("La resta es    : {}", numbers.resta());
    println!("El producto es : {}", numbers.multiplo());
    println!("La división es : {}", numbers.division());
    println!("El módulo es   : {}", numbers.modulo());
    println!("Fin");
    println!("");
}

mod test;
