use std::io;

fn main() {
    let array: [i32; 5] = [00, 11, 22, 33, 44];

    println!("");
    println!("Please enter an array index, (from 1 to 4)");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");

    let element: i32 = array[index];

    println!("");
    println!("The value of the element at index {index} is : {element}");
    println!("");
}
