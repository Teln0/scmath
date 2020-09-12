use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{Display, Formatter};
use core::fmt;
use std::process::Output;

pub mod geometry;
pub(crate) mod utils;

pub struct Vector<T, const SIZE: usize> {
    pub coords: [T; SIZE]
}

pub type Real = f64;

pub struct Complex {
    pub real_part: Real,
    pub imaginary_part: Real
}

pub trait Root<T> {
    fn root(&self) -> T;
}

macro_rules! type_and_impl_vector_2d {
    ($type: ty, $name: ident) => {
        pub type $name = Vector<$type, 2>;
        impl $name {
            pub fn new(x: $type, y: $type) -> $name {
                $name {
                    coords: [x, y]
                }
            }
        }
    }
}

macro_rules! type_and_impl_vector_3d {
    ($type: ty, $name: ident) => {
        pub type $name = Vector<$type, 3>;
        impl $name {
            pub fn new(x: $type, y: $type, z: $type) -> $name {
                $name {
                    coords: [x, y, z]
                }
            }
        }
    }
}

macro_rules! impl_operator_for_vector {
    ($self_vector_type: ty, $rhs_vector_type: ty, $operator_trait: tt, $function_name: ident, $operator_symbol: tt) => {
        impl<T: $operator_trait<Output = T> + Copy, const SIZE: usize> $operator_trait<$rhs_vector_type> for $self_vector_type {
            type Output = Vector<T, SIZE>;

            fn $function_name(self, rhs: $rhs_vector_type) -> Self::Output {
                let mut i = 0;
                let coords: [T; SIZE] = self.coords.map(|e| {
                    let val = (e $operator_symbol rhs.coords[i]);
                    i += 1;
                    return val;
                });
                Vector {
                    coords
                }
            }
        }
    }
}

macro_rules! impl_operator_for_vector_all {
    ($vector_type: ty, $operator_trait: tt, $function_name: ident, $operator_symbol: tt) => {
        impl_operator_for_vector!($vector_type, $vector_type, $operator_trait, $function_name, $operator_symbol);
        impl_operator_for_vector!(&$vector_type, $vector_type, $operator_trait, $function_name, $operator_symbol);
        impl_operator_for_vector!($vector_type, &$vector_type, $operator_trait, $function_name, $operator_symbol);
        impl_operator_for_vector!(&$vector_type, &$vector_type, $operator_trait, $function_name, $operator_symbol);
    }
}

impl<T: Display, const SIZE: usize> Display for Vector<T, SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.coords.len() {
            write!(f, "{}", &self.coords[i])?;
            if i != self.coords.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

pub trait Dot<T> {
    type Output;

    fn dot(&self, rhs: &T) -> Self::Output;
}

pub trait Normalized {
    type Output;

    fn normalized(&self) -> Self::Output;
}

impl<T: Default + Add<Output = T> + Mul<Output = T> + Copy, const SIZE: usize> Dot<Vector<T, SIZE>> for Vector<T, SIZE> {
    type Output = T;

    fn dot(&self, rhs: &Vector<T, SIZE>) -> T {
        let mut result = T::default();
        for i in 0..self.coords.len() {
            result = result + (self.coords[i] * rhs.coords[i]);
        }
        result
    }
}

impl<T: Default + Add<Output = T> + Mul<Output = T> + Root<T> + Copy, const SIZE: usize> Vector<T, SIZE> {
    pub fn magnitude(&self) -> T {
        let mut result = T::default();
        for i in self.coords.iter() {
            let i = *i;
            result = result + (i * i);
        }
        result.root()
    }
}

impl<T: Default + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Root<T> + Copy, const SIZE: usize> Normalized for Vector<T, SIZE> {
    type Output = Self;

    fn normalized(&self) -> Vector<T, SIZE> {
        let mag = self.magnitude();

        let coords = self.coords.map(|e| {
            e / mag
        });

        Vector {
            coords
        }
    }
}

impl<T: Clone + Copy, const SIZE: usize> Clone for Vector<T, SIZE> {
    fn clone(&self) -> Self {
        Vector {
            coords: self.coords
        }
    }
}

impl<T: Copy, const SIZE: usize> Copy for Vector<T, SIZE> {

}

impl<T: Default + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Root<T> + Copy, const SIZE: usize> Vector<T, SIZE> {
    pub fn distance(&self, rhs: &Vector<T, SIZE>) -> T {
        (self - rhs).magnitude()
    }
}

impl Root<f64> for f64 {
    fn root(&self) -> f64 {
        self.sqrt()
    }
}

impl_operator_for_vector_all!(Vector<T, SIZE>, Add, add, +);
impl_operator_for_vector_all!(Vector<T, SIZE>, Sub, sub, -);
impl_operator_for_vector_all!(Vector<T, SIZE>, Mul, mul, *);
impl_operator_for_vector_all!(Vector<T, SIZE>, Div, div, /);

type_and_impl_vector_2d!(i64, Vec2i);
type_and_impl_vector_2d!(u64, Vec2u);
type_and_impl_vector_2d!(f64, Vec2f);
type_and_impl_vector_2d!(Complex, Vec2c);

type_and_impl_vector_3d!(i64, Vec3i);
type_and_impl_vector_3d!(u64, Vec3u);
type_and_impl_vector_3d!(f64, Vec3f);
type_and_impl_vector_3d!(Complex, Vec3c);