use crate::algo_lib::collections::compressed_coords::CompressedCoords;

use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::geometry::segment_intersection_coef::segment_interection_coef;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::math::frac::FracT;
use crate::algo_lib::misc::gen_vector::gen_vec;
use crate::algo_lib::misc::vec_apply_delta::ApplyDelta;
use crate::algo_lib::seg_trees::lazy_seg_tree_max_add::{Node, SegTreeMaxAdd};
type Point = PointT<i64>;
type Frac = FracT<i128>;
type SegTree = SegTreeMaxAdd<i32>;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Position {
    edge_id: usize,
    coef: Frac,
}
fn solve(input: &mut Input, out: &mut Output) {
    let zero = Frac::new(0, 1);
    let one = Frac::new(1, 1);
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let a = gen_vec(n, |_| Point::new(input.i64(), input.i64()));
        let b = gen_vec(m, |_| Point::new(input.i64(), input.i64()));
        let p = input.vec::<usize>(n).sub_from_all(1);
        let gen_positions = |a: &[Point], rev: bool| -> Vec<Position> {
            let mut res = Vec::with_capacity(a.len());
            let mut edge_id = 0;
            for i in 0..a.len() {
                let A = a[i];
                let B = a[(i + 1) % a.len()];
                loop {
                    let C = b[edge_id];
                    let D = b[(edge_id + 1) % b.len()];
                    if let Some(coef) = segment_interection_coef([C, D], [A, B]) {
                        if coef >= zero && coef < one {
                            if let Some(coef2) = segment_interection_coef(
                                [A, B],
                                [C, D],
                            ) {
                                if coef2 > one {
                                    res.push(Position { edge_id, coef });
                                    break;
                                }
                            }
                        }
                    }
                    if rev {
                        edge_id = (edge_id + b.len() - 1) % b.len();
                    } else {
                        edge_id = (edge_id + 1) % b.len();
                    }
                }
            }
            res
        };
        let from_pos = gen_positions(&a, false);
        let to_pos = {
            let a_rev = {
                let mut a_rev = a.clone();
                a_rev.reverse();
                a_rev
            };
            let mut positions = gen_positions(&a_rev, true);
            positions.reverse();
            positions
        };
        let mut all_coords = from_pos.clone();
        all_coords.extend_from_slice(&to_pos);
        let compressed = CompressedCoords::new(&all_coords);
        let mut res = vec![];
        let sz = compressed.size();
        let mut st = SegTree::new(sz, |_| Node { max_val: 0 });
        for &idx in p.iter().rev() {
            let from = compressed.get(from_pos[idx]);
            let to = compressed.get(to_pos[idx]) + 1;
            if from < to {
                st.update(from..to, 1);
            } else {
                st.update(from..sz, 1);
                st.update(0..to, 1);
            }
            res.push(st.get(0..sz).max_val);
        }
        res.reverse();
        out.println(res);
    }
}
pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

fn main() {
    let input = crate::algo_lib::io::input::Input::new_stdin();
    let mut output = crate::algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
pub mod algo_lib {
pub mod collections {
pub mod compressed_coords {
pub struct CompressedCoords<T: Ord> {
    values: Vec<T>,
}
impl<T: Ord + Clone> CompressedCoords<T> {
    pub fn new(values: &[T]) -> Self {
        let mut values = values.to_vec();
        values.sort();
        values.dedup();
        Self { values }
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn get(&self, x: T) -> usize {
        self.values.binary_search(&x).unwrap()
    }
}
}
}
pub mod geometry {
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
        Self { x: x.into(), y: y.into() }
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
    pub const ZERO: PointT<T> = PointT { x: T::ZERO, y: T::ZERO };
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
pub mod segment_intersection_coef {
use crate::algo_lib::{geometry::point::PointT, math::frac::FracT};
type Point = PointT<i64>;
type Frac = FracT<i128>;
fn c(p: Point) -> PointT<i128> {
    PointT::new(p.x as i128, p.y as i128)
}
pub fn segment_interection_coef(s1: [Point; 2], s2: [Point; 2]) -> Option<Frac> {
    let A = c(s1[0]);
    let B = c(s1[1]);
    let C = c(s2[0]);
    let D = c(s2[1]);
    let s_adc = PointT::vect_mul(&A, &D, &C);
    let s_bcd = PointT::vect_mul(&B, &C, &D);
    if s_adc + s_bcd == 0 {
        return None;
    }
    Some(Frac::new(s_adc, s_adc + s_bcd))
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
        #[allow(unused)] pub fn $t (& mut self) -> $t { self.read_integer() }
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
            .unwrap_or_else(|_| {
                panic!("Can't open file: {:?}", path.as_ref().as_os_str())
            });
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
            .unwrap_or_else(|_| {
                panic!("Can't open file: {:?}", path.as_ref().as_os_str())
            });
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
        if self.refill_buffer() { Some(self.buf[self.at]) } else { None }
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
        if res.is_empty() { None } else { Some(res) }
    }
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
            Some(res) => unsafe { String::from_utf8_unchecked(res) }
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
        impl Readable for $t { fn read(input : & mut Input) -> Self { input
        .read_integer() } }
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
    pub fn print_iter_ref<'a, T: 'a + Writable, I: Iterator<Item = &'a T>>(
        &mut self,
        iter: I,
    ) {
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
        impl Writable for $t { fn write(& self, output : & mut Output) { self.to_string()
        .write(output); } }
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
        Self { dx: -self.dx, dy: -self.dy }
    }
}
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
pub mod frac {
use std::cmp::Ordering;
use crate::algo_lib::{
    math::gcd::gcd, misc::num_traits::{ConvSimple, HasConstants, Number},
};
#[derive(Clone, Copy, Default, Debug)]
pub struct FracT<T: Number> {
    pub num: T,
    pub denom: T,
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> FracT<T> {
    pub fn new(mut num: T, mut denom: T) -> Self {
        if denom == T::ZERO {
            return match num.cmp(&T::ZERO) {
                Ordering::Less => {
                    Self {
                        num: T::ZERO - T::ONE,
                        denom: T::ZERO,
                    }
                }
                Ordering::Equal => {
                    Self {
                        num: T::ZERO,
                        denom: T::ZERO,
                    }
                }
                Ordering::Greater => {
                    Self {
                        num: T::ONE,
                        denom: T::ZERO,
                    }
                }
            };
        }
        if denom < T::ZERO {
            num *= T::ZERO - T::ONE;
            denom *= T::ZERO - T::ONE;
        }
        let num_abs = if num < T::ZERO { T::ZERO - num } else { num };
        let g = T::ONE;
        Self {
            num: num / g,
            denom: denom / g,
        }
    }
}
impl<T: Number> PartialEq for FracT<T> {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.denom == other.denom
    }
}
impl<T: Number> Eq for FracT<T> {}
impl<T: Number + Ord> PartialOrd for FracT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            (self.num.to_i128() * other.denom.to_i128())
                .cmp(&(other.num.to_i128() * self.denom.to_i128())),
        )
    }
}
impl<T: Number + Ord> Ord for FracT<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Mul for FracT<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.num, self.denom * rhs.denom)
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::MulAssign for FracT<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Add for FracT<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.denom + rhs.num * self.denom, self.denom * rhs.denom)
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::AddAssign for FracT<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Div for FracT<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.denom, self.denom * rhs.num)
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::DivAssign for FracT<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::Sub for FracT<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.num * rhs.denom - rhs.num * self.denom, self.denom * rhs.denom)
    }
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> std::ops::SubAssign for FracT<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<T: Number + std::ops::Rem<Output = T>> HasConstants<FracT<T>> for FracT<T> {
    const MAX: Self = FracT {
        num: T::MAX,
        denom: T::ONE,
    };
    const MIN: Self = FracT {
        num: T::MIN,
        denom: T::ONE,
    };
    const ZERO: Self = FracT {
        num: T::ZERO,
        denom: T::ONE,
    };
    const ONE: Self = FracT {
        num: T::ONE,
        denom: T::ONE,
    };
    const TWO: Self = FracT {
        num: T::TWO,
        denom: T::ONE,
    };
}
impl<T: Number + std::ops::Rem<Output = T> + Ord> ConvSimple<Self> for FracT<T> {
    fn from_i32(val: i32) -> Self {
        Self::new(T::from_i32(val), T::ONE)
    }
    fn to_i32(self) -> i32 {
        (self.num / self.denom).to_i32()
    }
    fn to_f64(self) -> f64 {
        self.num.to_f64() / self.denom.to_f64()
    }
    fn to_i128(self) -> i128 {
        (self.num / self.denom).to_i128()
    }
}
}
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
pub fn diophantine(
    a: i64,
    b: i64,
    c: i64,
    x0: &mut i64,
    y0: &mut i64,
    g: &mut i64,
) -> bool {
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
    if x == T::ZERO { y } else { gcd(y % x, x) }
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
    if g != 1 { None } else { Some((x % m + m) % m) }
}
}
}
pub mod misc {
pub mod dbg_macro {
#[macro_export]
macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        eprint!("[{}:{}] {} = {:?}", file!(), line!(), stringify!($first_val),
        &$first_val); ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
        eprintln!();
    };
    ($first_val:expr) => {
        eprintln!("[{}:{}] {} = {:?}", file!(), line!(), stringify!($first_val),
        &$first_val)
    };
}
}
pub mod gen_vector {
pub fn gen_vec<T>(n: usize, f: impl FnMut(usize) -> T) -> Vec<T> {
    (0..n).map(f).collect()
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
pub trait Number: Copy + Clone + Default + Add<
        Output = Self,
    > + AddAssign + Sub<
        Output = Self,
    > + SubAssign + Mul<
        Output = Self,
    > + MulAssign + Div<
        Output = Self,
    > + DivAssign + PartialOrd + PartialEq + HasConstants<
        Self,
    > + Default + Debug + Sized + ConvSimple<Self> {}
impl<
    T: Copy + Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign + Div<Output = Self> + DivAssign + PartialOrd
        + PartialEq + HasConstants<Self> + Default + Debug + Sized + ConvSimple<Self>,
> Number for T {}
macro_rules! has_constants_impl {
    ($t:ident) => {
        impl HasConstants <$t > for $t { const MAX : $t = $t ::MAX; const MIN : $t = $t
        ::MIN; const ZERO : $t = 0; const ONE : $t = 1; const TWO : $t = 2; } impl
        ConvSimple <$t > for $t { fn from_i32(val : i32) -> $t { val as $t } fn
        to_i32(self) -> i32 { self as i32 } fn to_f64(self) -> f64 { self as f64 } fn
        to_i128(self) -> i128 { self as i128 } }
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
pub mod vec_apply_delta {
use crate::algo_lib::misc::num_traits::Number;
pub trait ApplyDelta<T> {
    fn add_to_all(self, delta: T) -> Self;
    fn sub_from_all(self, sub: T) -> Self;
}
impl<T> ApplyDelta<T> for Vec<T>
where
    T: Number,
{
    fn add_to_all(mut self, delta: T) -> Self {
        self.iter_mut().for_each(|val| *val += delta);
        self
    }
    fn sub_from_all(mut self, sub: T) -> Self {
        self.iter_mut().for_each(|val| *val -= sub);
        self
    }
}
impl<T> ApplyDelta<T> for Vec<(T, T)>
where
    T: Number,
{
    fn add_to_all(mut self, delta: T) -> Self {
        self.iter_mut()
            .for_each(|(val1, val2)| {
                *val1 += delta;
                *val2 += delta;
            });
        self
    }
    fn sub_from_all(mut self, sub: T) -> Self {
        self.iter_mut()
            .for_each(|(val1, val2)| {
                *val1 -= sub;
                *val2 -= sub;
            });
        self
    }
}
pub trait ApplyDelta2<T> {
    fn add_to_all(&mut self, delta: T);
    fn sub_from_all(&mut self, sub: T);
}
impl<T> ApplyDelta2<T> for [T]
where
    T: Number,
    T: Sized,
{
    fn add_to_all(self: &mut [T], delta: T) {
        self.iter_mut().for_each(|x| *x += delta);
    }
    fn sub_from_all(&mut self, sub: T) {
        self.iter_mut().for_each(|x| *x -= sub);
    }
}
}
}
pub mod seg_trees {
pub mod lazy_seg_tree_max_add {
use crate::algo_lib::{
    misc::num_traits::Number,
    seg_trees::{optimized_seg_tree::OptimizedSegTree, seg_tree_trait::SegTreeNode},
};
#[derive(Clone, Default, Copy, Debug)]
pub struct Node<T: Number> {
    pub max_val: T,
}
impl<T: Number> SegTreeNode for Node<T> {
    #[allow(unused)]
    fn join_nodes(l: &Self, r: &Self, context: &()) -> Self {
        if l.max_val > r.max_val { *l } else { *r }
    }
    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_val += *update;
    }
    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }
    type Update = T;
    type Context = ();
}
pub type SegTreeMaxAdd<T> = OptimizedSegTree<Node<T>>;
}
pub mod optimized_seg_tree {
use std::ops::Range;
use super::seg_tree_trait::SegTreeNode;
/// Iterative lazy segment tree using the same nodes and update composition as `SegTree`.
///
/// Joins must be associative, and applying an update must distribute over a join.
/// `join_updates(old, new)` must represent applying `old` followed by `new`.
/// Neither nodes nor updates need to be `Copy`, and `Default` need not be an identity.
/// Ranges are half-open and must be within `0..len()`; empty queries return `T::default()`.
///
/// Uses a power-of-two leaf base, `2 * base` node slots and `base` lazy slots.
/// Traversals allocate no memory and take O(log n); whole-array queries take O(1).
/// Queries do not mutate the tree: pending tags are applied to the query's
/// accumulators instead of being pushed to children. Node operations themselves
/// may allocate or cost more than O(1), depending on the `SegTreeNode` implementation.
/// Unlike `SegTree`, this does not expose expert node indices or predicate searches.
#[derive(Clone)]
pub struct OptimizedSegTree<T: SegTreeNode> {
    n: usize,
    base: usize,
    height: u32,
    tree: Vec<T>,
    lazy: Vec<Option<T::Update>>,
    missing_right: Vec<bool>,
    context: T::Context,
}
impl<T: SegTreeNode> OptimizedSegTree<T> {
    pub fn new(n: usize, f: impl Fn(usize) -> T) -> Self
    where
        T::Context: Default,
    {
        Self::new_with_context(n, f, T::Context::default())
    }
    pub fn new_with_context(
        n: usize,
        f: impl Fn(usize) -> T,
        context: T::Context,
    ) -> Self {
        assert!(n > 0);
        let base = n.next_power_of_two();
        let mut res = Self {
            n,
            base,
            height: base.trailing_zeros(),
            tree: vec![T::default(); 2 * base],
            lazy: vec![None; base],
            missing_right: vec![false; base],
            context,
        };
        for i in 0..n {
            res.tree[base + i] = f(i);
        }
        let mut last = base + n - 1;
        while last > 1 {
            res.missing_right[last / 2] = last & 1 == 0;
            last >>= 1;
        }
        let mut first = base >> 1;
        let mut last = (base + n - 1) >> 1;
        while first > 0 {
            for v in first..=last {
                res.pull(v);
            }
            first >>= 1;
            last >>= 1;
        }
        res
    }
    #[inline(always)]
    fn pull(&mut self, v: usize) {
        self.tree[v] = if self.missing_right[v] {
            self.tree[2 * v].clone()
        } else {
            T::join_nodes(&self.tree[2 * v], &self.tree[2 * v + 1], &self.context)
        };
    }
    #[inline(always)]
    fn apply(&mut self, v: usize, update: &T::Update) {
        T::apply_update(&mut self.tree[v], update);
        if v < self.base {
            match &mut self.lazy[v] {
                Some(old) => T::join_updates(old, update),
                slot @ None => *slot = Some(update.clone()),
            }
        }
    }
    #[inline(always)]
    fn push(&mut self, v: usize) {
        if let Some(update) = self.lazy[v].take() {
            self.apply(2 * v, &update);
            self.apply(2 * v + 1, &update);
        }
    }
    #[inline]
    fn push_boundaries(&mut self, l: usize, r: usize) {
        let l_skip = l.trailing_zeros();
        let r_skip = r.trailing_zeros();
        for level in ((l_skip.min(r_skip) + 1)..=self.height).rev() {
            let left = l >> level;
            let right = (r - 1) >> level;
            if level > l_skip {
                self.push(left);
            }
            if level > r_skip && (left != right || level <= l_skip) {
                self.push(right);
            }
        }
    }
    #[inline]
    pub fn update(&mut self, range: Range<usize>, update: T::Update) {
        if range.is_empty() {
            return;
        }
        assert!(range.end <= self.n);
        let mut l = range.start + self.base;
        let mut r = range.end + self.base;
        self.push_boundaries(l, r);
        let mut left_dirty = false;
        let mut right_dirty = false;
        while l < r {
            if left_dirty {
                self.pull(l - 1);
            }
            if right_dirty {
                self.pull(r);
            }
            if l & 1 != 0 {
                self.apply(l, &update);
                l += 1;
                left_dirty = true;
            }
            if r & 1 != 0 {
                r -= 1;
                self.apply(r, &update);
                right_dirty = true;
            }
            l >>= 1;
            r >>= 1;
        }
        let mut left = if left_dirty { l - 1 } else { 0 };
        let mut right = if right_dirty { r } else { 0 };
        while left != 0 || right != 0 {
            if left != 0 {
                self.pull(left);
            }
            if right != 0 && right != left {
                self.pull(right);
            }
            left >>= 1;
            right >>= 1;
        }
    }
    #[inline]
    pub fn get(&self, range: Range<usize>) -> T {
        if range.is_empty() {
            return T::default();
        }
        assert!(range.end <= self.n);
        if range.start == 0 && range.end == self.n {
            return self.tree[1].clone();
        }
        let mut l = range.start + self.base;
        let mut r = range.end + self.base;
        let mut left = None;
        let mut right = None;
        while l < r {
            if let Some(ref mut value) = left {
                if let Some(update) = &self.lazy[l - 1] {
                    T::apply_update(value, update);
                }
            }
            if let Some(ref mut value) = right {
                if let Some(update) = &self.lazy[r] {
                    T::apply_update(value, update);
                }
            }
            if l & 1 != 0 {
                left = Some(
                    match left {
                        None => self.tree[l].clone(),
                        Some(ref old) => T::join_nodes(old, &self.tree[l], &self.context),
                    },
                );
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                right = Some(
                    match right {
                        None => self.tree[r].clone(),
                        Some(ref old) => T::join_nodes(&self.tree[r], old, &self.context),
                    },
                );
            }
            l >>= 1;
            r >>= 1;
        }
        let mut lv = l - 1;
        let mut rv = r;
        while lv != rv {
            if let Some(ref mut value) = left {
                if let Some(update) = &self.lazy[lv] {
                    T::apply_update(value, update);
                }
            }
            if let Some(ref mut value) = right {
                if let Some(update) = &self.lazy[rv] {
                    T::apply_update(value, update);
                }
            }
            lv >>= 1;
            rv >>= 1;
        }
        let mut result = match (left, right) {
            (Some(l), Some(r)) => T::join_nodes(&l, &r, &self.context),
            (Some(v), None) | (None, Some(v)) => v,
            _ => unreachable!(),
        };
        while lv > 0 {
            if let Some(update) = &self.lazy[lv] {
                T::apply_update(&mut result, update);
            }
            lv >>= 1;
        }
        result
    }
    pub fn update_point(&mut self, pos: usize, new_node: T) {
        assert!(pos < self.n);
        let v = self.base + pos;
        for level in (1..=self.height).rev() {
            self.push(v >> level);
        }
        self.tree[v] = new_node;
        for level in 1..=self.height {
            self.pull(v >> level);
        }
    }
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn get_context(&self) -> &T::Context {
        &self.context
    }
    /// As with `SegTree`, changing context does not rebuild existing aggregates.
    pub fn update_context(&mut self, f: impl Fn(&mut T::Context)) {
        f(&mut self.context);
    }
}
}
pub mod seg_tree_trait {
pub trait SegTreeNode: Clone + Default {
    fn join_nodes(l: &Self, r: &Self, context: &Self::Context) -> Self;
    fn apply_update(node: &mut Self, update: &Self::Update);
    fn join_updates(current: &mut Self::Update, add: &Self::Update);
    type Update: Clone;
    type Context;
}
}
}
}
