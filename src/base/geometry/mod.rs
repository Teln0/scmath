use crate::base::utils::HeapArray;
use crate::base::{Vector, Dot, Root, Normalized};
use std::ops::{Sub, Mul, Index, Add, Div};
use crate::base;
use std::fmt::{Display, Debug};
use std::process::Output;

pub struct Line<T> {
    a: T,
    b: T
}

impl<T> Line<T> {
    pub fn new(a: T, b: T) -> Line<T> {
        Line {
            a,
            b
        }
    }
}

pub struct Polygon<T, const VERTICES: usize> {
    pub vertices: [T; VERTICES]
}

macro_rules! type_polygon {
    ($type: ty, $size: expr, $name: ident) => {
        pub type $name = Polygon<$type, $size>;
    }
}

impl<T, const VERTICES: usize> Polygon<T, VERTICES> {
    pub fn new(vertices: [T; VERTICES]) -> Polygon<T, VERTICES> {
        Polygon {
            vertices
        }
    }
}

impl<T, const VERTICES: usize> Index<i64> for Polygon<T, VERTICES> {
    type Output = T;

    fn index(&self, index: i64) -> &Self::Output {
        &self.vertices[(index.rem_euclid(self.vertices.len() as i64)) as usize]
    }
}

impl<T, U, const VERTICES: usize> Polygon<U, VERTICES>
    where T:
    PartialOrd +
    Default +
    Sub<Output = T> +
    Add<Output = T> +
    Div<Output = T> +
    Mul<Output = T> +
    Root<T> +
    Debug +
    Copy,
    U:
    Sub<Output = U> +
    Dot<U, Output = T> +
    Normalized<Output = U> +
    Display +
    Copy{
    pub fn self_intersections(&self) -> HeapArray<T> {
        unimplemented!();
    }

    pub fn self_intersecting(&self) -> bool {
        if self.vertices.len() > 3 {
            let scalar_at_point = |a: U, b: U, c: U| {
                let vec_a = (b - a).normalized();
                let vec_b = (b - c).normalized();

                println!("POINTS : {} {} {}", a, b, c);
                println!("VECS : {} {}", vec_a, vec_b);
                println!("DOT : {:#?}", vec_a.dot(&vec_b));

                vec_a.dot(&vec_b)
            };

            let mut prev_sign: Option<i8> = None;

            for i in 0..(self.vertices.len() as i64) {
                println!("TESTING POINT {}", i);

                let sign = scalar_at_point(self[i - 1], self[i], self[i + 1]);
                let sign: Option<i8> = if sign > T::default() {Some(1)} else if sign < T::default() {Some(-1)} else {None};

                if let Some(s) = prev_sign {
                    if let Some(s2) = sign {
                        if s != s2 {
                            return true;
                        }
                    }
                }
                if sign.is_some() {
                    prev_sign = sign;
                }
            }
        }

        false
    }
}

type_polygon!(crate::base::Vec2f, 3, Triangle);
type_polygon!(crate::base::Vec2f, 4, Quadrilateral);
type_polygon!(crate::base::Vec2f, 5, Pentagon);
type_polygon!(crate::base::Vec2f, 6, Hexagon);
type_polygon!(crate::base::Vec2f, 7, Heptagon);
type_polygon!(crate::base::Vec2f, 8, Octogon);

type PolyhedronFace = HeapArray<usize>;

pub struct Polyhedron<T, const FACES: usize, const VERTICES: usize> {
    pub vertices: [T; VERTICES],
    pub faces: [PolyhedronFace; FACES] // A face is an array of indexes referring to vertices
}

impl<T, const FACES: usize, const VERTICES: usize>  Polyhedron<T, FACES, VERTICES>  {
    pub fn new(vertices: [T; VERTICES], faces: [PolyhedronFace; FACES]) -> Polyhedron<T, FACES, VERTICES> {
        Polyhedron {
            vertices,
            faces
        }
    }
}

impl<T: std::cmp::PartialOrd> Vector<T, 2> {
    pub fn xy_lexicographical_comp(&self, rhs: &Vector<T, 2>) -> i8 {
        if self.coords[0] > rhs.coords[0] {
            return 1;
        }
        if self.coords[0] < rhs.coords[0] {
            return -1;
        }

        if self.coords[1] > rhs.coords[1] {
            return 1;
        }
        if self.coords[1] < rhs.coords[1] {
            return -1;
        }

        return 0;
    }
}

impl<T: std::cmp::PartialOrd + Sub<Output = T> + Mul<Output = T> + Copy + Default> Vector<T, 2> {
    pub fn is_on_left_of(&self, rhs: &Line<Vector<T, 2>>) -> bool {
        return
            (rhs.b.coords[0] - rhs.a.coords[0]) * (self.coords[1] - rhs.a.coords[1]) -
            (self.coords[0] - rhs.a.coords[0]) * (rhs.b.coords[1] - rhs.a.coords[1]) > T::default()
    }
}