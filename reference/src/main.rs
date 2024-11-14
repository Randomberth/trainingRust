fn main() {
    let s1 = String::from("123456789");

    let len = calculate_lenght(&s1);

    println!("The length of {s1} is {len}");

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{r1} and {r2}");
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("{r3}");
    }
}

fn calculate_lenght(s: &String) -> usize {
    s.len()
}
