use crate::io::input::{Input, Readable};
use crate::iters::shifts::Shift;
use crate::misc::num_traits::Number;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct PointT<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T: Number> PointT<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn dist2(&self, p2: &PointT<T>) -> T {
        let dx = self.x - p2.x;
        let dy = self.y - p2.y;
        dx * dx + dy * dy
    }

    pub fn vect_mul(p1: &PointT<T>, p2: &PointT<T>, p3: &PointT<T>) -> T {
        (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
    }

    pub fn scal_mul(p1: &PointT<T>, p2: &PointT<T>, p3: &PointT<T>) -> T {
        Self::scal_mul2(&(*p2 - *p1), &(*p3 - *p1))
    }

    pub fn scal_mul2(p1: &PointT<T>, p2: &PointT<T>) -> T {
        p1.x * p2.x + p1.y * p2.y
    }

    pub fn vect_mul2(p1: &PointT<T>, p2: &PointT<T>) -> T {
        p1.x * p2.y - p1.y * p2.x
    }

    pub fn apply_shift(&self, shift: &Shift) -> Self {
        Self {
            x: self.x + T::from_i32(shift.dx),
            y: self.y + T::from_i32(shift.dy),
        }
    }

    pub fn index_vec2d<'a, Elem>(&self, arr: &'a [Vec<Elem>]) -> Option<&'a Elem> {
        if self.x >= T::ZERO
            && self.x < T::from_i32(arr.len() as i32)
            && self.y >= T::ZERO
            && self.y < T::from_i32(arr[T::to_i32(self.x) as usize].len() as i32)
        {
            let x = T::to_i32(self.x) as usize;
            let y = T::to_i32(self.y) as usize;
            Some(&arr[x][y])
        } else {
            None
        }
    }

    pub fn rotateCCW(&self) -> Self {
        Self::new(T::ZERO - self.y, self.x)
    }

    pub const ZERO: PointT<T> = PointT {
        x: T::ZERO,
        y: T::ZERO,
    };
}

impl<T> Add for PointT<T>
where
    T: Number,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for PointT<T>
where
    T: Number,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for PointT<T>
where
    T: Number,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for PointT<T>
where
    T: Number,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Readable for PointT<T>
where
    T: Number + Readable,
{
    fn read(input: &mut Input) -> Self {
        let x = input.read();
        let y = input.read();
        Self { x, y }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PointWithIdT<T: Number> {
    pub p: PointT<T>,
    id: u32,
}

impl<T> PointWithIdT<T>
where
    T: Number,
{
    pub fn new(p: PointT<T>, id: usize) -> Self {
        Self { p, id: id as u32 }
    }

    pub fn id(&self) -> usize {
        self.id as usize
    }
}
