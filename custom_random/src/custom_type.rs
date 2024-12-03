use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Test {
    h: i32,
    j: i32,
}

/*
impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Test1 {{ h: {}, j: {} }}", self.h, self.j)
    }
}*/

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

impl Distribution<Test> for Standard {
    fn sample<P: Rng + ?Sized>(&self, rng: &mut P) -> Test {
        let (rand_h, rand_j) = rng.gen();
        Test {
            h: rand_h,
            j: rand_j,
        }
    }
}

pub fn custom_type() {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    let rand_test: Test = rng.gen();
    //println!("");
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
    println!("{:?}", rand_test);
}
