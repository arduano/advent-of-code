use std::ops::*;

use crate::{Grid2, Pos2, ToSignedIndex, Vec2};

pub struct IGrid2<T: Default> {
    grid: Grid2<T>,
    up: usize,
    left: usize,
    default: T,
}

fn get_dim_size(current: usize, index: isize) -> usize {
    let index = index.max(0) as usize;
    if index < current {
        return current;
    }

    // Get the next largest square using integer/bit math
    let mut next_fitting_square = 1 << (64 - (index as u64).leading_zeros());

    if next_fitting_square > 1 && next_fitting_square < 16 {
        next_fitting_square = 16;
    }

    next_fitting_square as usize
}

impl<T: Default> IGrid2<T> {
    pub fn new() -> Self {
        Self {
            grid: Grid2::new_default(0, 0),
            up: 0,
            left: 0,
            default: T::default(),
        }
    }

    pub fn up_capacity(&self) -> usize {
        self.up
    }

    pub fn left_capacity(&self) -> usize {
        self.left
    }

    pub fn down_capacity(&self) -> usize {
        self.grid.height() - self.up
    }

    pub fn right_capacity(&self) -> usize {
        self.grid.width() - self.left
    }

    fn get_inner_grid_coord(&self, coord: Pos2<impl ToSignedIndex + Copy>) -> Option<Pos2<usize>> {
        let as_index = coord.map(|c| c.to_index());
        if -as_index.x > self.left_capacity() as isize
            || -as_index.y > self.up_capacity() as isize
            || as_index.x >= self.right_capacity() as isize
            || as_index.y >= self.down_capacity() as isize
        {
            return None;
        }

        Some(Pos2::new(
            (as_index.x + self.left as isize) as usize,
            (as_index.y + self.up as isize) as usize,
        ))
    }

    pub fn expand_to_fit(&mut self, coord: Pos2<impl ToSignedIndex + Copy>) {
        let new_up = get_dim_size(self.up_capacity() + 1, -coord.y.to_index()) - 1;
        let new_left = get_dim_size(self.left_capacity() + 1, -coord.x.to_index()) - 1;
        let new_down = get_dim_size(self.down_capacity(), coord.y.to_index());
        let new_right = get_dim_size(self.right_capacity(), coord.x.to_index());

        let new_width = new_left + new_right;
        let new_height = new_up + new_down;

        if new_up == self.up_capacity()
            && new_left == self.left_capacity()
            && new_down == self.down_capacity()
            && new_right == self.right_capacity()
        {
            return;
        }

        let new_grid = Grid2::from_fn(new_width, new_height, |c| {
            let inner_coord = c.map(|d| d as isize) - Vec2::new(new_left as isize, new_up as isize);
            if let Some(inner_coord) = self.get_inner_grid_coord(inner_coord) {
                std::mem::replace(&mut self.grid[inner_coord], Default::default())
            } else {
                Default::default()
            }
        });

        self.grid = new_grid;
        self.up = new_up;
        self.left = new_left;
    }

    pub fn get(&self, coord: Pos2<impl ToSignedIndex + Copy>) -> &T {
        if let Some(coord) = self.get_inner_grid_coord(coord) {
            return &self.grid[coord];
        } else {
            return &self.default;
        }
    }

    pub fn get_mut(&mut self, coord: Pos2<impl ToSignedIndex + Copy>) -> &mut T {
        self.expand_to_fit(coord);
        let coord = self.get_inner_grid_coord(coord).unwrap();
        &mut self.grid[coord]
    }
}

impl<T: Default, I: ToSignedIndex + std::fmt::Debug + Copy + Default> Index<Pos2<I>> for IGrid2<T> {
    type Output = T;

    fn index(&self, index: Pos2<I>) -> &Self::Output {
        self.get(index)
    }
}

impl<T: Default, I: ToSignedIndex + std::fmt::Debug + Copy + Default> IndexMut<Pos2<I>>
    for IGrid2<T>
{
    fn index_mut(&mut self, index: Pos2<I>) -> &mut Self::Output {
        self.get_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_igrid2() {
        let mut grid = IGrid2::<usize>::new();
        assert_eq!(grid.up_capacity(), 0);
        assert_eq!(grid.left_capacity(), 0);
        assert_eq!(grid.down_capacity(), 0);
        assert_eq!(grid.right_capacity(), 0);

        grid[Pos2::new(0, 0)] = 1;
        assert_eq!(grid[Pos2::new(0, 0)], 1);

        grid[Pos2::new(100, 100)] = 2;
        assert_eq!(grid[Pos2::new(100, 100)], 2);

        grid[Pos2::new(-100, -100)] = 3;
        assert_eq!(grid[Pos2::new(-100, -100)], 3);

        assert_eq!(grid[Pos2::new(100, 100)], 2);
        assert_eq!(grid[Pos2::new(0, 0)], 1);
    }
}
