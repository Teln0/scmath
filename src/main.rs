#![feature(const_generics)]
#![feature(array_map)]
#![feature(core_intrinsics)]

use crate::base::utils::HeapArray;

#[allow(incomplete_features)]
#[allow(dead_code)]

mod base;

fn main() {
    let a: HeapArray<u64> = HeapArray::new_with_slice(&[1, 2, 3]);
    println!("{}", a[0]);
}
