use std::io;
mod loop_main;

fn main() {
    loop_main::greet("module");
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

    let number_boy: i32 = if value < 40 {
        (value * 2).try_into().unwrap()
    } else {
        (value * 4).try_into().unwrap()
    };

    if number > value.try_into().unwrap() {
        println!("");
        println!("condition was smaller than {}", number);
        println!("");
        println!("number boy : {}", number_boy);
    } else {
        println!("");
        println!("condition was bigger than {}", number);
        println!("");
        println!("number boy : {}", number_boy);
    }
}
