pub struct Vector<T, const SIZE: usize> {
    pub coords: [T; SIZE]
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

type_and_impl_vector_2d!(i64, Vec2i);
type_and_impl_vector_2d!(u64, Vec2u);
type_and_impl_vector_2d!(f64, Vec2f);

