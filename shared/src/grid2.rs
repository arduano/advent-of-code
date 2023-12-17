use std::ops::*;

use crate::{Pos2, ToUnsignedIndex};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Grid2<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid2<T> {
    pub fn new_empty() -> Self {
        Self {
            data: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn new_default(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        Self {
            data: (0..width * height)
                .map(|_| T::default())
                .collect::<Vec<_>>(),
            width,
            height,
        }
    }

    pub fn new_with(width: usize, height: usize, val: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: (0..width * height).map(|_| val.clone()).collect::<Vec<_>>(),
            width,
            height,
        }
    }

    pub fn from_fn(width: usize, height: usize, mut f: impl FnMut(Pos2<usize>) -> T) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(f(Pos2::new(x, y)));
            }
        }
        Self {
            data,
            width,
            height,
        }
    }

    pub fn map<U>(&self, f: impl FnMut(&T) -> U) -> Grid2<U> {
        Grid2 {
            data: self.data.iter().map(f).collect(),
            width: self.width,
            height: self.height,
        }
    }

    fn get_pos_index<I: ToUnsignedIndex + Copy>(&self, pos: Pos2<I>) -> Option<usize> {
        let pos = Pos2::new(pos.x.to_index()?, pos.y.to_index()?);
        let index = pos.y * self.width + pos.x;

        if pos.x >= self.width || pos.y >= self.height {
            return None;
        }

        if index < self.data.len() {
            Some(index)
        } else {
            None
        }
    }

    fn get_pos_index_or_panic<I: ToUnsignedIndex + std::fmt::Debug + Copy>(
        &self,
        pos: Pos2<I>,
    ) -> usize {
        let index = self.get_pos_index(pos);
        if let Some(index) = index {
            index
        } else {
            panic!(
                "Index out of bounds: {:?}, of array size {:?}",
                pos,
                Pos2::new(self.width, self.height)
            );
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, pos: Pos2<impl ToUnsignedIndex + std::fmt::Debug + Copy>) -> Option<&T> {
        Some(&self.data[self.get_pos_index(pos)?])
    }

    pub fn get_mut(
        &mut self,
        pos: Pos2<impl ToUnsignedIndex + std::fmt::Debug + Copy>,
    ) -> Option<&mut T> {
        let index = self.get_pos_index(pos)?;
        Some(&mut self.data[index])
    }

    pub fn is_in_bounds(&self, pos: Pos2<impl ToUnsignedIndex + std::fmt::Debug + Copy>) -> bool {
        self.get_pos_index(pos).is_some()
    }

    pub fn set(&mut self, pos: Pos2<impl ToUnsignedIndex + std::fmt::Debug + Copy>, value: T) {
        let index = self.get_pos_index_or_panic(pos);
        self.data[index] = value;
    }
}

impl<T, I: ToUnsignedIndex + std::fmt::Debug + Copy> Index<Pos2<I>> for Grid2<T> {
    type Output = T;

    fn index(&self, index: Pos2<I>) -> &Self::Output {
        &self.data[self.get_pos_index_or_panic(index)]
    }
}

impl<T, I: ToUnsignedIndex + std::fmt::Debug + Copy> IndexMut<Pos2<I>> for Grid2<T> {
    fn index_mut(&mut self, index: Pos2<I>) -> &mut Self::Output {
        let index = self.get_pos_index_or_panic(index);
        &mut self.data[index]
    }
}

impl<T> From<Vec<Vec<T>>> for Grid2<T> {
    fn from(vecs: Vec<Vec<T>>) -> Self {
        let width = vecs[0].len();
        let height = vecs.len();

        let mut data = Vec::with_capacity(width * height);

        for row in vecs {
            assert_eq!(row.len(), width);
            data.extend(row);
        }

        Self {
            data,
            width,
            height,
        }
    }
}
