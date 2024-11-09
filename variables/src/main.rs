fn main() {
    let x: u32 = 5;
    let x: u32 = x + 1;
    {
        let x: u32 = x + 2;
        println!("Inner scope : {x}")
    }
    println!("First statement : {x}");
    //scope of shadowing
    {
        let spaces: &str = "    ";
        println!("Spaces string : {spaces}");
        let spaces: usize = spaces.len();
        println!("Spaces number : {spaces}");
        //println!("First statement : {}", spaces_num);
    }
    // scope of operators
    {
        let sum: i32 = 5 + 10;
        let difference: f64 = 95.5 - 4.3;
        let product: i32 = 4 * 25;
        let quotient: i64 = 100 / 3;
        let remainder: i32 = 101 % 3;
        let t: bool = true;
        let tup: (i32, f64, u8) = (500, 5.8, 2);
        let (a, _b, _c) = tup;

        println!("");
        println!("Types :");
        println!("Sum        : {sum}");
        println!("Difference : {}", difference);
        println!("Product    : {}", product);
        println!("Quotien    : {}", quotient);
        println!("Remainder  : {}", remainder);
        println!("Bool type  : {}", t);
        println!("Tuple  (a) : {}", a);
        println!("Tuple  (b) : {}", _b);
    }
}
