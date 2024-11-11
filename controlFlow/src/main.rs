use std::io;

fn main() {
    println!("");
    println!("Please enter a number");

    let mut value: String = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("please enter type number");

    let value: usize = value
        .trim()
        .parse()
        .expect("index entered was not a number");

    let number: i32 = 33;

    if number > value.try_into().unwrap() {
        println!("");
        println!("concition was smaller than {}", number);
    } else {
        println!("");
        println!("concition was bigger than {}", number);
    }
}
