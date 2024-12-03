use rand::*;
use rand_distr::{Distribution, Normal, NormalError};
mod custom_type;
mod password_classic;
mod password_generator;

fn main() -> Result<(), NormalError> {
    let mut rng = thread_rng();
    let normal = Normal::new(5.0, 3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2, 9) distribution", v);
    println!("");

    custom_type::custom_type();
    password_generator::generator();

    println!("");
    println!("Password with defined characters");
    password_classic::password_f();

    Ok(())
}
