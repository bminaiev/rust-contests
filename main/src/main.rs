
use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;

use crate::algo_lib::geometry::segment::SegmentT;
use crate::algo_lib::geometry::segment_intersection_dd::segment_intersection_dd;
use crate::algo_lib::misc::double_double::DoubleDouble;

type Point = PointT<DoubleDouble>;

fn seg_sum(p1: Point, p2: Point) -> DoubleDouble {
    p1.x * p2.y - p1.y * p2.x
}

fn solve_case(a: &[Point]) -> f64 {
    let n = a.len();
    if n < 6 {
        return 0.0;
    }
    let get_p = |base: usize, offset: i32| -> Point {
        let i = ((base + 2 * n) as i32 + offset) % (n as i32);
        a[i as usize]
    };
    let get = |base: usize, offsets: [i32; 4]| -> PointT<DoubleDouble> {
        let p0 = get_p(base, offsets[0]);
        let p1 = get_p(base, offsets[1]);
        let p2 = get_p(base, offsets[2]);
        let p3 = get_p(base, offsets[3]);
        let seg1 = SegmentT::new(p0, p1);
        let seg2 = SegmentT::new(p2, p3);
        let inter = segment_intersection_dd(&seg1, &seg2);
        inter.unwrap()
    };
    let get_def = |i: usize| -> DoubleDouble {
        let p0 = get(i, [-2, 0, -1, 1]);
        let p1 = get(i, [-1, 1, 0, 2]);
        seg_sum(p0, p1)
    };
    let mut sum = DoubleDouble::ZERO;
    for i in 0..n {
        sum += get_def(i);
    }
    assert!(sum >= DoubleDouble::ZERO);
    if n % 2 == 0 {
        return sum.to_f64();
    }
    let mut res = DoubleDouble::NEG_INFINITY;
    for base in 0..n {
        // base+1 and base+2 <- same color
        let mut cur_sum = sum;
        for i in 0..4 {
            cur_sum -= get_def(base + i);
        }
        let p0 = get(base, [-2, 0, -1, 1]);
        let p1 = get(base, [-1, 1, 0, 3]);
        let p2 = get(base, [0, 3, 2, 4]);
        let p3 = get(base, [2, 4, 3, 5]);
        cur_sum += seg_sum(p0, p1);
        cur_sum += seg_sum(p1, p2);
        cur_sum += seg_sum(p2, p3);
        if cur_sum > res {
            res = cur_sum;
        }
    }
    res.to_f64()
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for i in 0..n {
            a.push(Point::new(input.i64(), input.i64()));
        }
        let res = solve_case(&a);
        out.println(res / 2.0);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}


//END MAIN


fn main() {
    let input = crate::algo_lib::io::input::Input::new_stdin();
    let mut output = crate::algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
pub mod algo_lib {
pub mod geometry {
pub mod direction {
use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::misc::num_traits::Number;
use std::cmp::Ordering;

///
/// Sorted counter-clock-wise
/// starting from (0;0) -> (inf; 0)
///
pub struct DirectionT<T>(PointT<T>)
where
    T: Number;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum Side {
    PositiveY,
    NegativeY,
}

impl<T> DirectionT<T>
where
    T: Number,
{
    pub fn new(from: PointT<T>, to: PointT<T>) -> Self {
        Self(to - from)
    }

    pub fn inverse(&self) -> Self {
        Self(PointT::ZERO - self.0)
    }

    fn side(&self) -> Side {
        if self.0.y > T::ZERO || (self.0.y == T::ZERO && self.0.x >= T::ZERO) {
            Side::PositiveY
        } else {
            Side::NegativeY
        }
    }
}

impl<T> PartialEq<Self> for DirectionT<T>
where
    T: Number + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<T> Eq for DirectionT<T> where T: Number + Ord {}

impl<T> PartialOrd<Self> for DirectionT<T>
where
    T: Number + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.side().cmp(&other.side()).then(
                PointT::<T>::vect_mul2(&self.0, &other.0)
                    .cmp(&T::ZERO)
                    .reverse(),
            ),
        )
    }
}

impl<T> Ord for DirectionT<T>
where
    T: Number + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
}
pub mod line {
use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::math::gcd::gcd;
use crate::algo_lib::misc::num_traits::HasConstants;
use crate::algo_lib::misc::ord_f64::OrdF64;
use std::fmt::Debug;

type Point = PointT<OrdF64>;

// a*x + b*y + c = 0
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub a: OrdF64,
    pub b: OrdF64,
    pub c: OrdF64,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let a = p2.y - p1.y;
        let b = p1.x - p2.x;
        let res = Self {
            a,
            b,
            c: -(p1.x * a + p1.y * b),
        };
        // debug_assert!(res.on_line(p1));
        // debug_assert!(res.on_line(p2));
        res
    }

    pub fn new_gcd(p1: (i64, i64), p2: (i64, i64)) -> Self {
        let mut a = p2.1 - p1.1;
        let mut b = p1.0 - p2.0;
        let mut c = -(p1.0 * a + p1.1 * b);
        let mut g = gcd(a, b);
        g = gcd(g, c);
        g = g.abs();
        a /= g;
        b /= g;
        c /= g;
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn norm(&mut self) {
        if self.a > OrdF64::EPS {
        } else if self.a < -OrdF64::EPS {
            self.a = -self.a;
            self.b = -self.b;
            self.c = -self.c;
        } else if self.b > OrdF64::EPS {
        } else if self.b < -OrdF64::EPS {
            self.b = -self.b;
            self.c = -self.c;
        }
    }

    pub fn new3(mut a: i64, mut b: i64, mut c: i64) -> Self {
        let mut g = gcd(a, b);
        g = gcd(g, c);
        g = g.abs();
        a /= g;
        b /= g;
        c /= g;
        let res = Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        };
        debug_assert!(res.a != OrdF64::ZERO || res.b != OrdF64::ZERO);
        res
    }

    pub fn on_line(&self, p: &Point) -> bool {
        (self.a * p.x + self.b * p.y + self.c).eq_with_default_eps(&OrdF64::ZERO)
    }

    pub fn intersect(&self, other: &Self) -> Option<Point> {
        let denom = self.b * other.a - other.b * self.a;
        if denom.eq_with_default_eps(&OrdF64::ZERO) {
            return None;
        }
        let y_num = other.c * self.a - self.c * other.a;
        let x_num = self.c * other.b - other.c * self.b;
        let res = Point::new(x_num / denom, y_num / denom);
        debug_assert!(
            self.abs_dist(&res)
                .eq_with_eps(&OrdF64::ZERO, OrdF64::SMALL_EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            self,
            res,
            self.abs_dist(&res)
        );
        debug_assert!(
            other
                .abs_dist(&res)
                .eq_with_eps(&OrdF64::ZERO, OrdF64::SMALL_EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            other,
            res,
            self.abs_dist(&res)
        );
        Some(res)
    }

    pub fn signed_dist(&self, p: &Point) -> OrdF64 {
        (self.a * p.x + self.b * p.y + self.c) / (self.a * self.a + self.b * self.b).sqrt()
    }

    pub fn abs_dist(&self, p: &Point) -> OrdF64 {
        self.signed_dist(p).abs()
    }

    pub fn abs_dist2(&self, p: &Point) -> OrdF64 {
        let z = self.a * p.x + self.b * p.y + self.c;
        z * z / (self.a * self.a + self.b * self.b)
    }

    pub fn closest_to_zero(&self) -> Point {
        let den = self.a * self.a + self.b * self.b;
        Point::new(-self.a * self.c / den, -self.b * self.c / den)
    }

    pub fn closest_on_line(&self, p: Point) -> Point {
        let den = self.a * self.a + self.b * self.b;
        let num = self.a * p.x + self.b * p.y + self.c;
        Point::new(p.x - self.a * num / den, p.y - self.b * num / den)
    }

    pub fn mirror(&self, p: Point) -> Point {
        let closest = self.closest_on_line(p);
        Point::new(OrdF64::TWO * closest.x - p.x, OrdF64::TWO * closest.y - p.y)
    }

    pub fn project(&self, p: Point) -> OrdF64 {
        (-self.b * p.x + self.a * p.y) / (self.a * self.a + self.b * self.b).sqrt()
    }
}
}
pub mod line_dd {
use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::math::gcd::gcd;
use crate::algo_lib::misc::double_double::DoubleDouble;
use std::fmt::Debug;

type Point = PointT<DoubleDouble>;

// a*x + b*y + c = 0
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    pub a: DoubleDouble,
    pub b: DoubleDouble,
    pub c: DoubleDouble,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let a = p2.y - p1.y;
        let b = p1.x - p2.x;
        let res = Self {
            a,
            b,
            c: -(p1.x * a + p1.y * b),
        };
        // debug_assert!(res.on_line(p1));
        // debug_assert!(res.on_line(p2));
        res
    }

    pub fn new_gcd(p1: (i64, i64), p2: (i64, i64)) -> Self {
        let mut a = p2.1 - p1.1;
        let mut b = p1.0 - p2.0;
        let mut c = -(p1.0 * a + p1.1 * b);
        let mut g = gcd(a, b);
        g = gcd(g, c);
        g = g.abs();
        a /= g;
        b /= g;
        c /= g;
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn norm(&mut self) {
        if self.a > DoubleDouble::EPS {
        } else if self.a < -DoubleDouble::EPS {
            self.a = -self.a;
            self.b = -self.b;
            self.c = -self.c;
        } else if self.b > DoubleDouble::EPS {
        } else if self.b < -DoubleDouble::EPS {
            self.b = -self.b;
            self.c = -self.c;
        }
    }

    pub fn new3(mut a: i64, mut b: i64, mut c: i64) -> Self {
        let mut g = gcd(a, b);
        g = gcd(g, c);
        g = g.abs();
        a /= g;
        b /= g;
        c /= g;
        let res = Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        };
        debug_assert!(res.a != DoubleDouble::ZERO || res.b != DoubleDouble::ZERO);
        res
    }

    pub fn on_line(&self, p: &Point) -> bool {
        (self.a * p.x + self.b * p.y + self.c).eq_with_default_eps(&DoubleDouble::ZERO)
    }

    pub fn intersect(&self, other: &Self) -> Option<Point> {
        let denom = self.b * other.a - other.b * self.a;
        if denom.eq_with_default_eps(&DoubleDouble::ZERO) {
            return None;
        }
        let y_num = other.c * self.a - self.c * other.a;
        let x_num = self.c * other.b - other.c * self.b;
        let res = Point::new(x_num / denom, y_num / denom);
        debug_assert!(
            self.abs_dist(&res)
                .eq_with_eps(&DoubleDouble::ZERO, DoubleDouble::EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            self,
            res,
            self.abs_dist(&res)
        );
        debug_assert!(
            other
                .abs_dist(&res)
                .eq_with_eps(&DoubleDouble::ZERO, DoubleDouble::EPS),
            "line = {:?}, p = {:?}, dist = {:?}",
            other,
            res,
            self.abs_dist(&res)
        );
        Some(res)
    }

    pub fn signed_dist(&self, p: &Point) -> DoubleDouble {
        (self.a * p.x + self.b * p.y + self.c) / (self.a * self.a + self.b * self.b).sqrt()
    }

    pub fn abs_dist(&self, p: &Point) -> DoubleDouble {
        self.signed_dist(p).abs()
    }

    pub fn abs_dist2(&self, p: &Point) -> DoubleDouble {
        let z = self.a * p.x + self.b * p.y + self.c;
        z * z / (self.a * self.a + self.b * self.b)
    }

    pub fn closest_to_zero(&self) -> Point {
        let den = self.a * self.a + self.b * self.b;
        Point::new(-self.a * self.c / den, -self.b * self.c / den)
    }

    pub fn closest_on_line(&self, p: Point) -> Point {
        let den = self.a * self.a + self.b * self.b;
        let num = self.a * p.x + self.b * p.y + self.c;
        Point::new(p.x - self.a * num / den, p.y - self.b * num / den)
    }

    pub fn mirror(&self, p: Point) -> Point {
        let closest = self.closest_on_line(p);
        Point::new(
            DoubleDouble::TWO * closest.x - p.x,
            DoubleDouble::TWO * closest.y - p.y,
        )
    }

    pub fn project(&self, p: Point) -> DoubleDouble {
        (-self.b * p.x + self.a * p.y) / (self.a * self.a + self.b * self.b).sqrt()
    }
}
}
pub mod point {
use crate::f;
use crate::algo_lib::io::input::{Input, Readable};
use crate::algo_lib::io::output::{Output, Writable};
use crate::algo_lib::iters::shifts::Shift;
use crate::algo_lib::misc::num_traits::Number;
use crate::algo_lib::misc::ord_f64::OrdF64;
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
}
pub mod segment {
use std::cmp::Ordering;

use crate::algo_lib::geometry::direction::DirectionT;
use crate::algo_lib::geometry::line::Line;
use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::geometry::segment_intersection::inside_bounding_box;
use crate::algo_lib::geometry::segment_intersection_dd::inside_bounding_box_dd;
use crate::algo_lib::misc::double_double::DoubleDouble;
use crate::algo_lib::misc::num_traits::{HasConstants, Number};
use crate::algo_lib::misc::ord_f64::OrdF64;

#[derive(Copy, Clone, Debug)]
pub struct SegmentT<T>
where
    T: Number,
{
    pub from: PointT<T>,
    pub to: PointT<T>,
}

impl<T> SegmentT<T>
where
    T: Number + Ord,
{
    pub fn new(from: PointT<T>, to: PointT<T>) -> Self {
        Self { from, to }
    }

    pub fn dir(&self) -> DirectionT<T> {
        DirectionT::new(self.from, self.to)
    }

    ///
    /// 1 means "[p] is to the left from [self.from] -> [self.to] ray"
    /// 0 means "on the same line"
    ///
    pub fn to_the_left(&self, p: &PointT<T>) -> i32 {
        let v_mul = PointT::<T>::vect_mul(&self.from, &self.to, p);
        match v_mul.cmp(&T::ZERO) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

impl SegmentT<OrdF64> {
    pub fn to_line(&self) -> Line {
        Line::new(&self.from, &self.to)
    }

    pub fn contains(&self, p: &PointT<OrdF64>) -> bool {
        // TODO: should use eps?
        inside_bounding_box(self, p) && PointT::vect_mul(&self.from, &self.to, p) == OrdF64::ZERO
    }

    pub fn len2(&self) -> OrdF64 {
        self.from.dist2(&self.to)
    }

    pub fn len(&self) -> OrdF64 {
        self.len2().sqrt()
    }
}

impl SegmentT<DoubleDouble> {
    pub fn to_line(&self) -> crate::algo_lib::geometry::line_dd::Line {
        crate::algo_lib::geometry::line_dd::Line::new(&self.from, &self.to)
    }

    pub fn contains(&self, p: &PointT<DoubleDouble>) -> bool {
        inside_bounding_box_dd(self, p)
            && PointT::vect_mul(&self.from, &self.to, p)
                .eq_with_eps(&DoubleDouble::ZERO, DoubleDouble::EPS)
    }

    pub fn len2(&self) -> DoubleDouble {
        self.from.dist2(&self.to)
    }

    pub fn len(&self) -> DoubleDouble {
        self.len2().sqrt()
    }
}
}
pub mod segment_intersection {
use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::geometry::segment::SegmentT;
use crate::algo_lib::misc::ord_f64::OrdF64;
use std::cmp::{max, min};

type Point = PointT<OrdF64>;
type Segment = SegmentT<OrdF64>;

fn inside_one_dim(range: (OrdF64, OrdF64), val: OrdF64) -> bool {
    min(range.0, range.1) - OrdF64::EPS <= val && val <= max(range.0, range.1) + OrdF64::EPS
}

pub fn inside_bounding_box(seg: &Segment, p: &Point) -> bool {
    inside_one_dim((seg.from.x, seg.to.x), p.x) && inside_one_dim((seg.from.y, seg.to.y), p.y)
}

pub fn segment_intersection(seg1: &Segment, seg2: &Segment) -> Option<Point> {
    seg1.to_line()
        .intersect(&seg2.to_line())
        .filter(|&inter| inside_bounding_box(seg1, &inter) && inside_bounding_box(seg2, &inter))
}
}
pub mod segment_intersection_dd {
use crate::algo_lib::geometry::segment::SegmentT;
use crate::algo_lib::{geometry::point::PointT, misc::double_double::DoubleDouble};
use std::cmp::{max, min};

type Point = PointT<DoubleDouble>;
type Segment = SegmentT<DoubleDouble>;

fn inside_one_dim(range: (DoubleDouble, DoubleDouble), val: DoubleDouble) -> bool {
    min(range.0, range.1) - DoubleDouble::EPS <= val
        && val <= max(range.0, range.1) + DoubleDouble::EPS
}

pub fn inside_bounding_box_dd(seg: &Segment, p: &Point) -> bool {
    inside_one_dim((seg.from.x, seg.to.x), p.x) && inside_one_dim((seg.from.y, seg.to.y), p.y)
}

pub fn segment_intersection_dd(seg1: &Segment, seg2: &Segment) -> Option<Point> {
    seg1.to_line().intersect(&seg2.to_line()).filter(|&inter| {
        inside_bounding_box_dd(seg1, &inter) && inside_bounding_box_dd(seg2, &inter)
    })
}
}
}
pub mod io {
pub mod input {
use std::fmt::Debug;
use std::io::Read;
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

pub struct Input {
    input: Box<dyn Read>,
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
}

macro_rules! read_integer_fun {
    ($t:ident) => {
        #[allow(unused)]
        pub fn $t(&mut self) -> $t {
            self.read_integer()
        }
    };
}

impl Input {
    const DEFAULT_BUF_SIZE: usize = 4096;

    ///
    /// Using with stdin:
    /// ```no_run
    /// use algo_lib::io::input::Input;
    /// let stdin = std::io::stdin();
    /// let input = Input::new(Box::new(stdin));
    /// ```
    ///
    /// For read files use ``new_file`` instead.
    ///
    ///
    pub fn new(input: Box<dyn Read>) -> Self {
        Self {
            input,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn new_stdin() -> Self {
        let stdin = std::io::stdin();
        Self::new(Box::new(stdin))
    }

    pub fn new_file<P: AsRef<Path>>(path: P) -> Self {
        let file = std::fs::File::open(&path)
            .unwrap_or_else(|_| panic!("Can't open file: {:?}", path.as_ref().as_os_str()));
        Self::new(Box::new(file))
    }

    pub fn new_with_size(input: Box<dyn Read>, buf_size: usize) -> Self {
        Self {
            input,
            buf: vec![0; buf_size],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn new_file_with_size<P: AsRef<Path>>(path: P, buf_size: usize) -> Self {
        let file = std::fs::File::open(&path)
            .unwrap_or_else(|_| panic!("Can't open file: {:?}", path.as_ref().as_os_str()));
        Self::new_with_size(Box::new(file), buf_size)
    }

    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            Some(res)
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            Some(self.buf[self.at])
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !char::from(b).is_whitespace() {
                return;
            }
            self.get();
        }
    }

    pub fn next_token(&mut self) -> Option<Vec<u8>> {
        self.skip_whitespace();
        let mut res = Vec::new();
        while let Some(c) = self.get() {
            if char::from(c).is_whitespace() {
                break;
            }
            res.push(c);
        }
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    //noinspection RsSelfConvention
    pub fn is_exhausted(&mut self) -> bool {
        self.peek().is_none()
    }

    pub fn has_more_elements(&mut self) -> bool {
        !self.is_exhausted()
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }

    pub fn vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
        let mut res = Vec::with_capacity(size);
        for _ in 0usize..size {
            res.push(self.read());
        }
        res
    }

    pub fn string_vec(&mut self, size: usize) -> Vec<Vec<u8>> {
        let mut res = Vec::with_capacity(size);
        for _ in 0usize..size {
            res.push(self.string());
        }
        res
    }

    pub fn read_line(&mut self) -> String {
        let mut res = String::new();
        while let Some(c) = self.get() {
            if c == b'\n' {
                break;
            }
            if c == b'\r' {
                if self.peek() == Some(b'\n') {
                    self.get();
                }
                break;
            }
            res.push(c.into());
        }
        res
    }

    #[allow(clippy::should_implement_trait)]
    pub fn into_iter<T: Readable>(self) -> InputIterator<T> {
        InputIterator {
            input: self,
            phantom: Default::default(),
        }
    }

    fn read_integer<T: FromStr + Debug>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let res = self.read_string();
        res.parse::<T>().unwrap()
    }

    fn read_string(&mut self) -> String {
        match self.next_token() {
            None => {
                panic!("Input exhausted");
            }
            Some(res) => unsafe { String::from_utf8_unchecked(res) },
        }
    }

    pub fn string_as_string(&mut self) -> String {
        self.read_string()
    }

    pub fn string(&mut self) -> Vec<u8> {
        self.read_string().into_bytes()
    }

    fn read_char(&mut self) -> char {
        self.skip_whitespace();
        self.get().unwrap().into()
    }

    fn read_float(&mut self) -> f64 {
        self.read_string().parse().unwrap()
    }

    pub fn f64(&mut self) -> f64 {
        self.read_float()
    }

    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = self.input.read(&mut self.buf).unwrap();
            self.buf_read != 0
        } else {
            true
        }
    }

    read_integer_fun!(i32);
    read_integer_fun!(i64);
    read_integer_fun!(i128);
    read_integer_fun!(u32);
    read_integer_fun!(u64);
    read_integer_fun!(usize);
}

pub trait Readable {
    fn read(input: &mut Input) -> Self;
}

impl Readable for String {
    fn read(input: &mut Input) -> Self {
        input.read_string()
    }
}

impl Readable for char {
    fn read(input: &mut Input) -> Self {
        input.read_char()
    }
}

impl Readable for f64 {
    fn read(input: &mut Input) -> Self {
        input.read_string().parse().unwrap()
    }
}

impl Readable for f32 {
    fn read(input: &mut Input) -> Self {
        input.read_string().parse().unwrap()
    }
}

impl<T: Readable> Readable for Vec<T> {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.vec(size)
    }
}

pub struct InputIterator<T: Readable> {
    input: Input,
    phantom: PhantomData<T>,
}

impl<T: Readable> Iterator for InputIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.skip_whitespace();
        self.input.peek().map(|_| self.input.read())
    }
}

macro_rules! read_integer {
    ($t:ident) => {
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.read_integer()
            }
        }
    };
}

read_integer!(i8);
read_integer!(i16);
read_integer!(i32);
read_integer!(i64);
read_integer!(i128);
read_integer!(isize);
read_integer!(u8);
read_integer!(u16);
read_integer!(u32);
read_integer!(u64);
read_integer!(u128);
read_integer!(usize);
}
pub mod output {
use std::io::Write;

pub struct Output {
    output: Box<dyn Write>,
    buf: Vec<u8>,
    at: usize,
    auto_flush: bool,
}

impl Output {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(output: Box<dyn Write>) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            auto_flush: false,
        }
    }

    pub fn new_stdout() -> Self {
        let stdout = std::io::stdout();
        Self::new(Box::new(stdout))
    }

    pub fn new_file(path: impl AsRef<std::path::Path>) -> Self {
        let file = std::fs::File::create(path).unwrap();
        Self::new(Box::new(file))
    }

    pub fn new_with_auto_flush(output: Box<dyn Write>) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            auto_flush: true,
        }
    }

    pub fn flush(&mut self) {
        if self.at != 0 {
            self.output.write_all(&self.buf[..self.at]).unwrap();
            self.at = 0;
            self.output.flush().expect("Couldn't flush output");
        }
    }

    pub fn print<T: Writable>(&mut self, s: T) {
        s.write(self);
    }

    pub fn println<T: Writable>(&mut self, s: T) {
        s.write(self);
        self.put(b'\n');
    }

    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }

    pub fn maybe_flush(&mut self) {
        if self.auto_flush {
            self.flush();
        }
    }

    pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
        for i in arg {
            i.write(self);
            self.put(b'\n');
        }
    }

    pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }

    pub fn print_iter_ref<'a, T: 'a + Writable, I: Iterator<Item = &'a T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut start = 0usize;
        let mut rem = buf.len();
        while rem > 0 {
            let len = (self.buf.len() - self.at).min(rem);
            self.buf[self.at..self.at + len].copy_from_slice(&buf[start..start + len]);
            self.at += len;
            if self.at == self.buf.len() {
                self.flush();
            }
            start += len;
            rem -= len;
        }
        if self.auto_flush {
            self.flush();
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush();
        Ok(())
    }
}

pub trait Writable {
    fn write(&self, output: &mut Output);
}

impl Writable for &str {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for String {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for char {
    fn write(&self, output: &mut Output) {
        output.put(*self as u8);
    }
}

impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write(&self, output: &mut Output) {
        self[..].write(output);
    }
}

macro_rules! write_to_string {
    ($t:ident) => {
        impl Writable for $t {
            fn write(&self, output: &mut Output) {
                self.to_string().write(output);
            }
        }
    };
}

write_to_string!(u8);
write_to_string!(u16);
write_to_string!(u32);
write_to_string!(u64);
write_to_string!(u128);
write_to_string!(usize);
write_to_string!(i8);
write_to_string!(i16);
write_to_string!(i32);
write_to_string!(i64);
write_to_string!(i128);
write_to_string!(isize);
write_to_string!(f32);
write_to_string!(f64);

impl<T: Writable, U: Writable> Writable for (T, U) {
    fn write(&self, output: &mut Output) {
        self.0.write(output);
        output.put(b' ');
        self.1.write(output);
    }
}

impl<T: Writable, U: Writable, V: Writable> Writable for (T, U, V) {
    fn write(&self, output: &mut Output) {
        self.0.write(output);
        output.put(b' ');
        self.1.write(output);
        output.put(b' ');
        self.2.write(output);
    }
}
}
}
pub mod iters {
pub mod shifts {
#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Shift {
    pub dx: i32,
    pub dy: i32,
}

impl Shift {
    pub fn rev(&self) -> Self {
        Self {
            dx: -self.dx,
            dy: -self.dy,
        }
    }
}

// x goes down
// y goes right
pub const SHIFT_DOWN: Shift = Shift { dx: 1, dy: 0 };
pub const SHIFT_UP: Shift = Shift { dx: -1, dy: 0 };
pub const SHIFT_RIGHT: Shift = Shift { dx: 0, dy: 1 };
pub const SHIFT_LEFT: Shift = Shift { dx: 0, dy: -1 };

pub const SHIFTS_4: [Shift; 4] = [SHIFT_DOWN, SHIFT_LEFT, SHIFT_UP, SHIFT_RIGHT];
pub const SHIFTS_8: [Shift; 8] = [
    SHIFT_DOWN,
    SHIFT_LEFT,
    SHIFT_UP,
    SHIFT_RIGHT,
    Shift { dx: -1, dy: -1 },
    Shift { dx: -1, dy: 1 },
    Shift { dx: 1, dy: -1 },
    Shift { dx: 1, dy: 1 },
];

pub const SHIFTS_9: [Shift; 9] = [
    SHIFT_DOWN,
    SHIFT_LEFT,
    SHIFT_UP,
    SHIFT_RIGHT,
    Shift { dx: -1, dy: -1 },
    Shift { dx: -1, dy: 1 },
    Shift { dx: 1, dy: -1 },
    Shift { dx: 1, dy: 1 },
    Shift { dx: 0, dy: 0 },
];

pub fn shift_by_nswe(c: u8) -> Shift {
    match c {
        b'S' | b's' => SHIFT_DOWN,
        b'N' | b'n' => SHIFT_UP,
        b'E' | b'e' => SHIFT_RIGHT,
        b'W' | b'w' => SHIFT_LEFT,
        _ => panic!("Unexpected direction!"),
    }
}

pub fn shift_by_uldr(c: u8) -> Shift {
    match c {
        b'D' | b'd' => SHIFT_DOWN,
        b'U' | b'u' => SHIFT_UP,
        b'R' | b'r' => SHIFT_RIGHT,
        b'L' | b'l' => SHIFT_LEFT,
        _ => panic!("Unexpected direction!"),
    }
}
}
}
pub mod math {
pub mod gcd {
use crate::algo_lib::misc::num_traits::Number;


fn extended_gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if a == 0 {
        *x = 0;
        *y = 1;
        return b;
    }
    let mut x1 = 0;
    let mut y1 = 0;
    let d = extended_gcd(b % a, a, &mut x1, &mut y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    d
}

///
///
/// Find any solution to equation A*x + B*y = C
///
/// Returns [false] if [C] is not divisible by gcd(A, B)
///

pub fn diophantine(a: i64, b: i64, c: i64, x0: &mut i64, y0: &mut i64, g: &mut i64) -> bool {
    *g = extended_gcd(a.abs(), b.abs(), x0, y0);
    if c % *g != 0 {
        return false;
    }
    *x0 *= c / *g;
    *y0 *= c / *g;
    if a < 0 {
        *x0 *= -1;
    }
    if b < 0 {
        *y0 *= -1;
    }
    true
}


pub fn gcd<T>(x: T, y: T) -> T
where
    T: Number + std::ops::Rem<Output = T>,
{
    if x == T::ZERO {
        y
    } else {
        gcd(y % x, x)
    }
}

pub fn lcm<T>(x: T, y: T) -> T
where
    T: Number + std::ops::Rem<Output = T>,
{
    x / gcd(x, y) * y
}

pub fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let g = extended_gcd(a, m, &mut x, &mut y);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}
}
}
pub mod misc {
pub mod dbg_macro {
#[macro_export]

macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        eprint!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val);
        ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
        eprintln!();
    };
    ($first_val:expr) => {
        eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val)
    };
}
}
pub mod double_double {
use crate::algo_lib::misc::num_traits::{ConvSimple, HasConstants};
use std::cmp::Ordering;
use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A normalized sum of two `f64`s, providing roughly 106 significant bits.
///
/// Arithmetic retains the low component; converting to `f64` rounds back to
/// ordinary double precision. The exponent range is still that of `f64`, and
/// the extra precision is not guaranteed near overflow or underflow.
/// Comparisons follow floating-point semantics: NaN is unordered, and
/// `Ord::cmp` panics when either operand is NaN.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DoubleDouble {
    hi: f64,
    lo: f64,
}

impl DoubleDouble {
    pub const ZERO: Self = Self { hi: 0.0, lo: 0.0 };
    pub const ONE: Self = Self { hi: 1.0, lo: 0.0 };
    pub const TWO: Self = Self { hi: 2.0, lo: 0.0 };
    pub const NAN: Self = Self {
        hi: f64::NAN,
        lo: 0.0,
    };
    pub const INFINITY: Self = Self {
        hi: f64::INFINITY,
        lo: 0.0,
    };
    pub const NEG_INFINITY: Self = Self {
        hi: f64::NEG_INFINITY,
        lo: 0.0,
    };
    pub const MAX: Self = Self {
        hi: f64::MAX,
        // Largest f64 below half an ulp of f64::MAX.
        lo: 9.979201547673598e291,
    };
    pub const MIN: Self = Self {
        hi: -Self::MAX.hi,
        lo: -Self::MAX.lo,
    };
    pub const EPS: Self = Self { hi: 1e-9, lo: 0.0 };

    /// Normalizes `hi + lo` using error-free TwoSum, without requiring either
    /// input to have the larger magnitude. The fields stay private so that
    /// arithmetic and comparisons can rely on this normalization.
    pub fn new(hi: f64, lo: f64) -> Self {
        let sum = hi + lo;
        if !sum.is_finite() {
            return Self::from(sum);
        }
        let v = sum - hi;
        let error = (hi - (sum - v)) + (lo - v);
        Self { hi: sum, lo: error }
    }

    pub fn to_f64(self) -> f64 {
        self.hi + self.lo
    }

    /// Truncates toward zero, saturating at the integer bounds (NaN becomes 0).
    /// Unlike casting `to_f64()`, this preserves integer bits in the low part.
    pub fn to_i128(self) -> i128 {
        if !self.hi.is_finite() {
            return self.hi as i128;
        }
        let limit = -(i128::MIN as f64);
        if self.hi >= limit {
            if self.hi > limit || self.lo >= 0.0 {
                return i128::MAX;
            }
            return (i128::MAX + self.lo.floor() as i128) + 1;
        }
        if self.hi <= -limit {
            if self.hi < -limit || self.lo <= 0.0 {
                return i128::MIN;
            }
            return i128::MIN + self.lo.ceil() as i128;
        }
        let hi = self.hi as i128;
        let lo = self.lo as i128;
        let integer = hi + lo;
        let fraction = (self.hi - hi as f64) + (self.lo - lo as f64);
        if integer > 0 && fraction < 0.0 {
            integer - 1
        } else if integer < 0 && fraction > 0.0 {
            integer + 1
        } else {
            integer
        }
    }

    pub fn abs(self) -> Self {
        if self.hi.is_sign_negative() {
            -self
        } else {
            self
        }
    }

    /// Compares using either absolute or relative tolerance, as in `OrdF64`.
    /// NaN never compares equal; infinities compare equal only to themselves.
    pub fn eq_with_eps(&self, other: &Self, eps: Self) -> bool {
        if !self.is_finite() || !other.is_finite() {
            return self == other;
        }
        let abs_diff = (*self - *other).abs();
        abs_diff <= eps || abs_diff <= self.abs().min(other.abs()) * eps
    }

    pub fn eq_with_default_eps(&self, other: &Self) -> bool {
        self.eq_with_eps(other, Self::EPS)
    }

    /// Square root with a correction retaining double-double precision.
    /// Negative inputs return NaN; signed zero and positive infinity are preserved.
    pub fn sqrt(self) -> Self {
        let root = self.hi.sqrt();
        if root == 0.0 || !root.is_finite() {
            return Self::from(root);
        }
        // Scale tiny inputs so the squared-root residual does not underflow.
        // Even the smallest subnormal becomes large enough after this scaling.
        if self.hi < f64::MIN_POSITIVE / f64::EPSILON {
            let scaled = self * Self::from(2.0_f64.powi(108));
            return scaled.sqrt() * Self::from(2.0_f64.powi(-54));
        }
        // FMA computes hi - root^2 without rounding (or overflowing) root^2 first.
        let residual = (-root).mul_add(root, self.hi) + self.lo;
        Self::new(root, residual / (2.0 * root))
    }

    pub fn is_finite(self) -> bool {
        self.hi.is_finite()
    }

    pub fn is_nan(self) -> bool {
        self.hi.is_nan()
    }
}

impl Add for DoubleDouble {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let high = Self::new(self.hi, rhs.hi);
        if !high.is_finite() {
            return high;
        }
        let low = Self::new(self.lo, rhs.lo);
        let sum = Self::new(high.hi, high.lo + low.hi);
        Self::new(sum.hi, sum.lo + low.lo)
    }
}

impl Sub for DoubleDouble {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl Mul for DoubleDouble {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let hi = self.hi * rhs.hi;
        if !hi.is_finite() {
            return Self::from(hi);
        }
        let lo = self.hi.mul_add(rhs.hi, -hi);
        let lo = self.hi.mul_add(rhs.lo, lo);
        let lo = self.lo.mul_add(rhs.hi, lo);
        let lo = self.lo.mul_add(rhs.lo, lo);
        Self::new(hi, lo)
    }
}

impl Div for DoubleDouble {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let q1 = Self::from(self.hi / rhs.hi);
        if !q1.is_finite() || !rhs.is_finite() || self.hi == 0.0 {
            return q1;
        }
        let r1 = self - rhs * q1;
        let q2 = Self::from(r1.hi / rhs.hi);
        let r2 = r1 - rhs * q2;
        let q3 = Self::from(r2.hi / rhs.hi);
        q1 + q2 + q3
    }
}

impl Neg for DoubleDouble {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
        }
    }
}

impl AddAssign for DoubleDouble {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for DoubleDouble {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for DoubleDouble {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for DoubleDouble {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Eq for DoubleDouble {}

impl Ord for DoubleDouble {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for DoubleDouble {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hi.partial_cmp(&other.hi)? {
            Ordering::Equal => self.lo.partial_cmp(&other.lo),
            order => Some(order),
        }
    }
}

impl From<f64> for DoubleDouble {
    fn from(value: f64) -> Self {
        Self { hi: value, lo: 0.0 }
    }
}

impl From<i128> for DoubleDouble {
    fn from(value: i128) -> Self {
        let hi = value as f64;
        // A positive value may round to 2^127, outside the i128 range.
        let remainder = if hi == -(i128::MIN as f64) {
            value - i128::MAX - 1
        } else {
            value - hi as i128
        };
        Self::new(hi, remainder as f64)
    }
}

impl From<u128> for DoubleDouble {
    fn from(value: u128) -> Self {
        let high = Self::from((value >> 64) as i128);
        let scale = 18446744073709551616.0; // 2^64, exact in f64.
        Self {
            hi: high.hi * scale,
            lo: high.lo * scale,
        } + Self::from(value as u64)
    }
}

macro_rules! from_integer {
    ($($ty:ty),* $(,)?) => {
        $(impl From<$ty> for DoubleDouble {
            fn from(value: $ty) -> Self {
                Self::from(value as i128)
            }
        })*
    };
}

from_integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl From<f32> for DoubleDouble {
    fn from(value: f32) -> Self {
        Self::from(value as f64)
    }
}

impl From<DoubleDouble> for f64 {
    fn from(value: DoubleDouble) -> Self {
        value.to_f64()
    }
}

impl HasConstants<Self> for DoubleDouble {
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    const TWO: Self = Self::TWO;
}

impl ConvSimple<Self> for DoubleDouble {
    fn from_i32(value: i32) -> Self {
        Self::from(value)
    }

    fn to_i32(self) -> i32 {
        self.to_i128().clamp(i32::MIN as i128, i32::MAX as i128) as i32
    }

    fn to_f64(self) -> f64 {
        self.to_f64()
    }

    fn to_i128(self) -> i128 {
        self.to_i128()
    }
}

impl Sum for DoubleDouble {
    /// Pairwise summation, retaining O(log n) partial sums of power-of-two sizes.
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut partials: Vec<Option<Self>> = Vec::new();
        for mut value in iter {
            let mut level = 0;
            loop {
                if level == partials.len() {
                    partials.push(Some(value));
                    break;
                }
                if let Some(previous) = partials[level].take() {
                    value = previous + value;
                    level += 1;
                } else {
                    partials[level] = Some(value);
                    break;
                }
            }
        }
        partials
            .into_iter()
            .flatten()
            .fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for DoubleDouble {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for DoubleDouble {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a> Product<&'a Self> for DoubleDouble {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}
}
pub mod num_traits {
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait HasConstants<T> {
    const MAX: T;
    const MIN: T;
    const ZERO: T;
    const ONE: T;
    const TWO: T;
}

pub trait ConvSimple<T> {
    fn from_i32(val: i32) -> T;
    fn to_i32(self) -> i32;
    fn to_f64(self) -> f64;
    fn to_i128(self) -> i128;
}

pub trait Signum {
    fn signum(&self) -> i32;
}

pub trait Number:
    Copy
    + Clone
    + Default
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + PartialOrd
    + PartialEq
    + HasConstants<Self>
    + Default
    + Debug
    + Sized
    + ConvSimple<Self>
{
}

impl<
        T: Copy
            + Add<Output = Self>
            + AddAssign
            + Sub<Output = Self>
            + SubAssign
            + Mul<Output = Self>
            + MulAssign
            + Div<Output = Self>
            + DivAssign
            + PartialOrd
            + PartialEq
            + HasConstants<Self>
            + Default
            + Debug
            + Sized
            + ConvSimple<Self>,
    > Number for T
{
}

macro_rules! has_constants_impl {
    ($t: ident) => {
        impl HasConstants<$t> for $t {
            const MAX: $t = $t::MAX;
            const MIN: $t = $t::MIN;
            const ZERO: $t = 0;
            const ONE: $t = 1;
            const TWO: $t = 2;
        }

        impl ConvSimple<$t> for $t {
            fn from_i32(val: i32) -> $t {
                val as $t
            }

            fn to_i32(self) -> i32 {
                self as i32
            }

            fn to_f64(self) -> f64 {
                self as f64
            }

            fn to_i128(self) -> i128 {
                self as i128
            }
        }
    };
}

has_constants_impl!(i32);
has_constants_impl!(i64);
has_constants_impl!(i128);
has_constants_impl!(u32);
has_constants_impl!(u64);
has_constants_impl!(u128);
has_constants_impl!(usize);
has_constants_impl!(u8);

impl ConvSimple<Self> for f64 {
    fn from_i32(val: i32) -> Self {
        val as f64
    }

    fn to_i32(self) -> i32 {
        self as i32
    }

    fn to_f64(self) -> f64 {
        self
    }

    fn to_i128(self) -> i128 {
        self as i128
    }
}

impl HasConstants<Self> for f64 {
    const MAX: Self = Self::MAX;
    const MIN: Self = -Self::MAX;
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
}

impl<T: Number + Ord> Signum for T {
    fn signum(&self) -> i32 {
        match self.cmp(&T::ZERO) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        }
    }
}
}
pub mod ord_f64 {
use crate::algo_lib::io::input::{Input, Readable};
use crate::algo_lib::io::output::{Output, Writable};
use crate::algo_lib::misc::num_traits::{ConvSimple, HasConstants};
use std::cmp::{min, Ordering};
use std::f64::consts::PI;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::num::ParseFloatError;
use std::ops::{Neg, Rem};
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone, Default)]
pub struct OrdF64(pub f64);

impl OrdF64 {
    pub const EPS: Self = Self(1e-9);
    pub const SMALL_EPS: Self = Self(1e-4);
    pub const PI: Self = Self(PI);

    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    pub fn eq_with_eps(&self, other: &Self, eps: Self) -> bool {
        let abs_diff = (*self - *other).abs();
        abs_diff <= eps || abs_diff <= min(self.abs(), other.abs()) * eps
    }

    pub fn eq_with_default_eps(&self, other: &Self) -> bool {
        self.eq_with_eps(other, Self::EPS)
    }

    pub fn sqrt(&self) -> Self {
        Self(self.0.sqrt())
    }

    pub fn powf(&self, n: f64) -> Self {
        Self(self.0.powf(n))
    }
}

impl Eq for OrdF64 {}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for OrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl std::ops::Add for OrdF64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for OrdF64 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for OrdF64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for OrdF64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl std::ops::Mul for OrdF64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::MulAssign for OrdF64 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl std::ops::Div for OrdF64 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::DivAssign for OrdF64 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Neg for OrdF64 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Display for OrdF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for OrdF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Writable for OrdF64 {
    fn write(&self, output: &mut Output) {
        output.write_fmt(format_args!("{}", self.0)).unwrap();
    }
}

impl Readable for OrdF64 {
    fn read(input: &mut Input) -> Self {
        Self(input.read::<f64>())
    }
}

impl HasConstants<Self> for OrdF64 {
    const MAX: Self = Self(f64::MAX);
    const MIN: Self = Self(-f64::MAX);
    const ZERO: Self = Self(0.0);
    const ONE: Self = Self(1.0);
    const TWO: Self = Self(2.0);
}

impl ConvSimple<Self> for OrdF64 {
    fn from_i32(val: i32) -> Self {
        Self(val as f64)
    }

    fn to_i32(self) -> i32 {
        self.0 as i32
    }

    fn to_f64(self) -> f64 {
        self.0
    }

    fn to_i128(self) -> i128 {
        self.0 as i128
    }
}

impl FromStr for OrdF64 {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<f64>() {
            Ok(value) => Ok(Self(value)),
            Err(error) => Err(error),
        }
    }
}

impl From<OrdF64> for f64 {
    fn from(x: OrdF64) -> Self {
        x.0
    }
}

impl Rem for OrdF64 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0 % rhs.0)
    }
}

#[macro_export]
macro_rules! f {
    ($a:expr) => {
        OrdF64($a)
    };
}

impl From<usize> for OrdF64 {
    fn from(x: usize) -> Self {
        f!(x as f64)
    }
}

impl From<i32> for OrdF64 {
    fn from(x: i32) -> Self {
        f!(x as f64)
    }
}

impl From<i64> for OrdF64 {
    fn from(x: i64) -> Self {
        f!(x as f64)
    }
}

impl From<f64> for OrdF64 {
    fn from(x: f64) -> Self {
        f!(x)
    }
}
}
}
}
