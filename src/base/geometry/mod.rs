use crate::base::utils::HeapArray;

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