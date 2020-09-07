#![feature(const_generics)]

mod algebra;

fn main() {
    let a = algebra::Vec2i::new(1, 2);
    println!("Hello, world!");
}
