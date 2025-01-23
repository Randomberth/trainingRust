use rand::distributions::*;
//use rand::Rng;
use rand::*;

fn main() {
    // random number
    {
        let mut rng = rand::thread_rng();

        let n1: u8 = rng.gen();
        let n2: u16 = rng.gen();
        let n3: f64 = rng.gen();

        println!("Generate a random number:");
        println!("");
        println!("Random u8: {}", n1);
        println!("Random u16: {}", n2);
        println!("Random u8: {}", rng.gen::<u32>());
        println!("Random u16: {}", rng.gen::<i32>());
        println!("Random u16: {}", rng.gen::<f64>());
        println!("Random u16: {}", n3);
        println!("");

        // randome number with a range

        println!("Generate a random number with 0..100 range:");
        println!("");
        println!("Integer: {}", rng.gen_range(0..100));
        println!("Float: {}", rng.gen_range(0.0..100.0));

        println!("Generate a random number with range::Distribution:");
        println!("");

        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..7);

        loop {
            let throw = die.sample(&mut rng);
            println!("Rol the die: {}", throw);
            if throw == 6 {
                println!("get 6, time to break");
                break;
            }
        }
    }
}
