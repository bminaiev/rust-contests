use crate::collections::array_2d::Array2D;
use crate::f;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::iters::shifts::Shift;
use crate::misc::num_traits::Number;
use crate::misc::ord_f64::OrdF64;
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

    pub fn side(&self) -> i32 {
        if self.y > T::ZERO || (self.y == T::ZERO && self.x >= T::ZERO) {
            return 0;
        }
        return 1;
    }

    pub fn dist_manh(&self, p2: &PointT<T>) -> T {
        let dx = self.x - p2.x;
        let dy = self.y - p2.y;
        let dx_abs = if dx < T::ZERO { T::ZERO - dx } else { dx };
        let dy_abs = if dy < T::ZERO { T::ZERO - dy } else { dy };
        dx_abs + dy_abs
    }

    pub fn angle_to(&self, other: &PointT<T>) -> OrdF64
    where
        f64: From<T>,
    {
        let dy = other.y - self.y;
        let dx = other.x - self.x;
        OrdF64(f64::atan2(dy.into(), dx.into()))
    }

    pub fn swap_x_y(&self) -> Self {
        Self::new(self.y, self.x)
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

    pub fn shift(&self, dx: T, dy: T) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    pub fn scale(&self, coef: T) -> Self {
        Self {
            x: self.x * coef,
            y: self.y * coef,
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

    pub fn index_arr2d<'a, Elem>(&self, arr: &'a Array2D<Elem>) -> Option<&'a Elem>
    where
        Elem: Clone,
    {
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

    pub fn rotate_ccw(&self) -> Self {
        Self::new(T::ZERO - self.y, self.x)
    }

    pub const ZERO: PointT<T> = PointT {
        x: T::ZERO,
        y: T::ZERO,
    };

    pub fn conv_float(&self) -> PointT<OrdF64>
    where
        f64: From<T>,
    {
        PointT::new(OrdF64(self.x.into()), OrdF64(self.y.into()))
    }
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

impl<T> Writable for PointT<T>
where
    T: Number + Writable,
{
    fn write(&self, output: &mut Output) {
        self.x.write(output);
        output.put(b' ');
        self.y.write(output);
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
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

impl PointT<OrdF64> {
    pub fn rotate_ccw_angle(&self, angle: OrdF64) -> Self {
        let cos = f!(angle.0.cos());
        let sin = f!(angle.0.sin());
        let x = self.x * cos - self.y * sin;
        let y = self.y * cos + self.x * sin;
        Self { x, y }
    }
}
