use crate::f;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::iters::shifts::Shift;
use crate::misc::num_traits::Number;
use crate::misc::ord_f64::OrdF64;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct PointT<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T: Ord + Number> Ord for PointT<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl<T: Ord + Number> PartialOrd for PointT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmp(other).into()
    }
}

impl<T: Number> PointT<T> {
    pub fn new<U: Into<T>>(x: U, y: U) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
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
        1
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

    pub fn rotate_ccw(&self) -> Self {
        Self::new(T::ZERO - self.y, self.x)
    }

    pub const ZERO: PointT<T> = PointT {
        x: T::ZERO,
        y: T::ZERO,
    };

    pub fn conv_float(&self) -> PointT<OrdF64> {
        PointT::new(OrdF64(self.x.to_f64()), OrdF64(self.y.to_f64()))
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

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
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

impl PointWithIdT<OrdF64> {
    pub fn dist(&self, other: &Self) -> OrdF64 {
        self.p.dist2(&other.p).sqrt()
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

impl Mul<OrdF64> for PointT<OrdF64> {
    type Output = PointT<OrdF64>;

    fn mul(self, rhs: OrdF64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
