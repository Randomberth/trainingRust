use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    let n3: f64 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u8: {}", rng.gen::<u32>());
    println!("Random u16: {}", rng.gen::<i32>());
    println!("Random u16: {}", rng.gen::<f64>());
    println!("Random u16: {}", n3);
}
