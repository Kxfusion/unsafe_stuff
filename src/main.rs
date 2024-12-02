use std::slice;
static mut COUNTER: u32 = 0;

#[macro_export]
macro_rules! cust_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp =  Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("huh");
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

pub struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(func: fn (i32) -> i32, value: i32) -> i32 {
    func(value) + func(value)
}


fn main() {
    let vecc = cust_vec![1, 3, 5];
    println!("{}", do_twice(add_one, 4));
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");

    println!("A baby dog or a {}", <Dog as Animal>::baby_name());

    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    add_to_count(3);

    unsafe {
        println!("{COUNTER}");
        println!("Abs Val of -3: {}", abs(-3));
        dangerous();
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;

    // unsafe {
    //     // println!("r is {}", *r);
    // }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
