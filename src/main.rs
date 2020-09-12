#![feature(const_generics)]
#![feature(array_map)]
#![feature(core_intrinsics)]
#![feature(associated_type_bounds)]

use crate::base::Vec2f;
use crate::base::geometry::{Line, Polygon};

#[allow(incomplete_features)]
#[allow(dead_code)]

mod base;

fn main() {
    let a = Vec2f::new(0_f64, 0_f64);
    let line = Line::new(
        Vec2f::new(1_f64, -1_f64),
        Vec2f::new(1_f64, 1_f64)
    );

    let poly = Polygon::new([
        Vec2f::new(0_f64, 0_f64),
        Vec2f::new(1_f64, 0_f64),
        Vec2f::new(0_f64, 1_f64),
        Vec2f::new(1_f64, 1_f64),
    ]);

    println!("{}", a.is_on_left_of(&line));
    println!("{}", poly.self_intersecting());
}
