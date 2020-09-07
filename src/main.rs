#![feature(const_generics)]
#![feature(array_map)]

#[allow(incomplete_features)]
#[allow(dead_code)]

mod algebra;

fn main() {
    let a = algebra::Vec3f::new(1f64, 2f64, 5f64);
    println!("{}", a.normalized().magnitude());
}
