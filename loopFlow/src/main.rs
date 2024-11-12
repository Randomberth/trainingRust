fn main() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
    println!("");
    // while loop
    println!("while loop :");
    println!("");
    {
        let mut number: i32 = 10;

        while number != 0 {
            println!("{number}");

            number -= 1;
        }
        println!("LIFTOFF!!!");
    }
    // while loop with array
    println!("");
    println!("while loop with array  :");
    {
        let a: [i32; 5] = [10, 20, 30, 40, 50];
        let mut index: usize = 0;

        while index < 5 {
            println!("the value is : {}", a[index]);
            index += 1;
        }
    }
    // for loop with array
    println!("");
    println!("for loop with array :");
    {
        let a: [i32; 5] = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is : {}", element);
        }
    }
    // range loop
    println!("");
    println!("range loop :");
    {
        for number in (1..10).rev() {
            println!("the value is : {}", number);
        }
        println!("LIFTOFF!!!");
    }
}
