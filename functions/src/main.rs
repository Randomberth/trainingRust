fn main() {
    println!("Hello, world!");

    another_function(33);
    println!("");
    print_labeled_measurement(32, "Aliberth");
    println!("");
    println!("");

    let z: i32 = expression_example();
    println!("Expression example : {z}");
}

fn another_function(x: i32) {
    println!("The value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is : {value} {unit_label}");
}

fn expression_example() -> i32 {
    let y: i32 = {
        let x: i32 = 3;
        x + 1
    };
    //println!("Expression example : {y}");
    return y;
}
