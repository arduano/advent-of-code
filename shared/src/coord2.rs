use std::ops::*;

use crate::One;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub struct Pos2<T>(Vec2<T>);

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn to_pos(self) -> Pos2<T> {
        Pos2(self)
    }

    pub fn zero() -> Self
    where
        T: Default,
    {
        Vec2 {
            x: T::default(),
            y: T::default(),
        }
    }

    pub fn left() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Vec2 {
            x: -T::one(),
            y: T::default(),
        }
    }

    pub fn right() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Vec2 {
            x: T::one(),
            y: T::default(),
        }
    }

    pub fn up() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Vec2 {
            x: T::default(),
            y: T::one(),
        }
    }

    pub fn down() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Vec2 {
            x: T::default(),
            y: -T::one(),
        }
    }

    pub fn four_directions() -> [Self; 4]
    where
        T: Default + One + Neg<Output = T>,
    {
        [Self::left(), Self::right(), Self::up(), Self::down()]
    }

    pub fn five_directions() -> [Self; 5]
    where
        T: Default + One + Neg<Output = T>,
    {
        [
            Self::zero(),
            Self::left(),
            Self::right(),
            Self::up(),
            Self::down(),
        ]
    }

    pub fn eight_directions() -> [Self; 8]
    where
        T: Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        [
            Self::left(),
            Self::right(),
            Self::up(),
            Self::down(),
            Self::left() + Self::up(),
            Self::left() + Self::down(),
            Self::right() + Self::up(),
            Self::right() + Self::down(),
        ]
    }

    pub fn nine_directions() -> [Self; 9]
    where
        T: Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        [
            Self::zero(),
            Self::left(),
            Self::right(),
            Self::up(),
            Self::down(),
            Self::left() + Self::up(),
            Self::left() + Self::down(),
            Self::right() + Self::up(),
            Self::right() + Self::down(),
        ]
    }

    fn iter_directions<const L: usize>(self, dirs: [Self; L]) -> impl Iterator<Item = Self>
    where
        T: Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        dirs.into_iter().map(move |dir| self + dir)
    }

    pub fn iter_four_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.iter_directions(Self::four_directions())
    }

    pub fn iter_five_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.iter_directions(Self::five_directions())
    }

    pub fn iter_eight_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.iter_directions(Self::eight_directions())
    }

    pub fn iter_nine_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.iter_directions(Self::nine_directions())
    }

    pub fn rot_left(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn rot_right(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Vec2 {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn flip(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }

    pub fn flip_x(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Vec2 {
            x: -self.x,
            y: self.y,
        }
    }

    pub fn flip_y(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Vec2 {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Vec2<U> {
        Vec2 {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl<T> Pos2<T> {
    pub fn new(x: T, y: T) -> Self {
        Pos2(Vec2 { x, y })
    }

    pub fn to_vec(self) -> Vec2<T> {
        self.0
    }

    pub fn zero() -> Self
    where
        T: Default,
    {
        Pos2(Vec2::zero())
    }

    pub fn left() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Pos2(Vec2::left())
    }

    pub fn right() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Pos2(Vec2::right())
    }

    pub fn up() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Pos2(Vec2::up())
    }

    pub fn down() -> Self
    where
        T: Default + One + Neg<Output = T>,
    {
        Pos2(Vec2::down())
    }

    pub fn rot_left(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Pos2(self.0.rot_left())
    }

    pub fn rot_right(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Pos2(self.0.rot_right())
    }

    pub fn flip(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Pos2(self.0.flip())
    }

    pub fn flip_x(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Pos2(self.0.flip_x())
    }

    pub fn flip_y(&self) -> Self
    where
        T: Copy + Neg<Output = T>,
    {
        Pos2(self.0.flip_y())
    }

    pub fn iter_four_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.0.iter_four_directions().map(Pos2)
    }

    pub fn iter_five_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.0.iter_five_directions().map(Pos2)
    }

    pub fn iter_eight_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.0.iter_eight_directions().map(Pos2)
    }

    pub fn iter_nine_directions<'a>(self) -> impl 'a + Iterator<Item = Self>
    where
        T: 'a + Default + One + Neg<Output = T> + Add<Output = T> + Copy,
    {
        self.0.iter_nine_directions().map(Pos2)
    }

    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Pos2<U> {
        Pos2(self.0.map(f))
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({:?}, {:?})", self.x, self.y)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Pos2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pos2({:?}, {:?})", self.x, self.y)
    }
}

impl<T> Deref for Pos2<T> {
    type Target = Vec2<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Pos2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Pos2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Add<Output = T> + Copy> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> Add<Vec2<T>> for Pos2<T> {
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Pos2(Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        })
    }
}

impl<T: Add<Output = T> + Copy> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Add<Output = T> + Copy> AddAssign<Vec2<T>> for Pos2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T: Sub<Output = T> + Copy> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Vec2<T>> for Pos2<T> {
    type Output = Self;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Pos2(Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        })
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: Sub<Output = T> + Copy> SubAssign<Vec2<T>> for Pos2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Pos2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Pos2(Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        })
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vec2<T>> for Pos2<T> {
    type Output = Self;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Pos2(Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        })
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Pos2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<Vec2<T>> for Pos2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<Vec2<T>> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Pos2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Pos2(Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        })
    }
}

impl<T: Div<Output = T> + Copy> Div<Vec2<T>> for Pos2<T> {
    type Output = Self;

    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Pos2(Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        })
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<Vec2<T>> for Vec2<T> {
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<T> for Pos2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<Vec2<T>> for Pos2<T> {
    fn div_assign(&mut self, rhs: Vec2<T>) {
        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Pos2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Pos2(Vec2 {
            x: -self.x,
            y: -self.y,
        })
    }
}
