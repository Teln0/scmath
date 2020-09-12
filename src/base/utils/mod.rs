use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct HeapArray<T> {
    inner: Box<[T]>
}

impl<T: Default> HeapArray<T> {
    pub fn new(size: usize) -> HeapArray<T> {
        let mut inner = Vec::new();
        inner.resize_with(size, Default::default);
        HeapArray {
            inner: inner.into_boxed_slice(),
        }
    }
}

impl<T> HeapArray<T> {
    pub fn new_with_boxed_slice(slice: Box<[T]>) -> HeapArray<T> {
        HeapArray {
            inner: slice
        }
    }

    pub fn new_with_vec(vec: Vec<T>) -> HeapArray<T> {
        HeapArray {
            inner: vec.into_boxed_slice()
        }
    }

    pub fn size(&self) -> usize {
        self.inner.len()
    }
}

impl<T: Copy> HeapArray<T> {
    pub fn new_with_slice(slice: &[T]) -> HeapArray<T> {
        let inner: Box<[T]> = slice.iter().map(|e| {
            *e
        }).collect();

        HeapArray {
            inner
        }
    }
}

impl<T> Deref for HeapArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<T> DerefMut for HeapArray<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.inner.deref_mut()
    }
}

impl<T> Index<usize> for HeapArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T> IndexMut<usize> for HeapArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.inner[index]
    }
}