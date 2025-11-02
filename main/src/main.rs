// 
use crate::algo_lib::collections::array_2d::Array2D;

use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::misc::rand::Random;
use crate::algo_lib::misc::simulated_annealing::SimulatedAnnealing;
fn emulate(a: &Array2D<usize>) -> usize {
    const DX: [i32; 4] = [-1, 0, 0, 1];
    const DY: [i32; 4] = [0, -1, 1, 0];
    const REV: [usize; 4] = [3, 2, 1, 0];
    let mut a = a.clone();
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    while x != a.len() - 1 || y != a[0].len() - 1 {
        steps += 1;
        let dir = a[x][y];
        a[x][y] = REV[dir];
        let xn = (x as i32 + DX[dir]);
        let yn = (y as i32 + DY[dir]);
        if xn >= 0 && (xn < a.len() as i32) && yn >= 0 && (yn < a[0].len() as i32) {
            x = xn as usize;
            y = yn as usize;
        }
    }
    steps
}
fn stress() {
    const N: usize = 8;
    let mut a = Array2D::new(0, N, N);
    let mut rnd = Random::new(342324);
    for i in 0..N {
        for j in 0..N {
            a[i][j] = rnd.gen_range(0..4);
        }
    }
    let steps = emulate(&a);
    let mut sa = SimulatedAnnealing::new(
        10.0,
        crate::algo_lib::misc::simulated_annealing::SearchFor::MaximumScore,
        100.0,
        1e-4,
        steps as f64,
    );
    while sa.should_continue() {
        let x = rnd.gen_range(0..N);
        let y = rnd.gen_range(0..N);
        let old = a[x][y];
        a[x][y] = rnd.gen_range(0..4);
        let new_steps = emulate(&a);
        dbg!(new_steps);
        if !sa.should_go(new_steps as f64) {
            a[x][y] = old;
        } else {
            dbg!(new_steps);
        }
    }
}
fn solve(input: &mut Input, out: &mut Output) {}
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
pub mod array_2d {
use crate::algo_lib::io::output::{Output, Writable};
use crate::algo_lib::misc::num_traits::Number;
use std::io::Write;
use std::ops::{Index, IndexMut, Mul};
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array2D<T> {
    rows: usize,
    cols: usize,
    v: Vec<T>,
}
pub struct Iter<'a, T> {
    array: &'a Array2D<T>,
    row: usize,
    col: usize,
}
impl<T> Array2D<T>
where
    T: Clone,
{
    #[allow(unused)]
    pub fn new(empty: T, rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            v: vec![empty; rows * cols],
        }
    }
    pub fn new_f(
        rows: usize,
        cols: usize,
        mut f: impl FnMut(usize, usize) -> T,
    ) -> Self {
        let mut v = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            for c in 0..cols {
                v.push(f(r, c));
            }
        }
        Self { rows, cols, v }
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.rows()
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn swap(&mut self, row1: usize, row2: usize) {
        assert!(row1 < self.rows);
        assert!(row2 < self.rows);
        if row1 != row2 {
            for col in 0..self.cols {
                self.v.swap(row1 * self.cols + col, row2 * self.cols + col);
            }
        }
    }
    pub fn transpose(&self) -> Self {
        Self::new_f(self.cols, self.rows, |r, c| self[c][r].clone())
    }
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            array: self,
            row: 0,
            col: 0,
        }
    }
    pub fn pref_sum(&self) -> Self
    where
        T: Number,
    {
        let mut res = Self::new(T::ZERO, self.rows + 1, self.cols + 1);
        for i in 0..self.rows {
            for j in 0..self.cols {
                let value = self[i][j] + res[i][j + 1] + res[i + 1][j] - res[i][j];
                res[i + 1][j + 1] = value;
            }
        }
        res
    }
}
impl<T> Writable for Array2D<T>
where
    T: Writable,
{
    fn write(&self, output: &mut Output) {
        for r in 0..self.rows {
            self[r].write(output);
            output.write_all(b"\n").unwrap();
        }
    }
}
impl<T> Index<usize> for Array2D<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        &self.v[(index) * self.cols..(index + 1) * self.cols]
    }
}
impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(index) * self.cols..(index + 1) * self.cols]
    }
}
impl<T> Mul for &Array2D<T>
where
    T: Number,
{
    type Output = Array2D<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        let n = self.rows;
        let m = self.cols;
        assert_eq!(m, rhs.rows);
        let k2 = rhs.cols;
        let mut res = Array2D::new(T::ZERO, n, k2);
        for i in 0..n {
            for j in 0..m {
                for k in 0..k2 {
                    res[i][k] += self[i][j] * rhs[j][k];
                }
            }
        }
        res
    }
}
impl<T> Array2D<T>
where
    T: Number,
{
    pub fn pown(&self, pw: usize) -> Self {
        assert_eq!(self.rows, self.cols);
        let n = self.rows;
        if pw == 0 {
            Self::new_f(n, n, |r, c| if r == c { T::ONE } else { T::ZERO })
        } else if pw == 1 {
            self.clone()
        } else {
            let half = self.pown(pw / 2);
            let half2 = &half * &half;
            if pw & 1 == 0 { half2 } else { &half2 * self }
        }
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.col == self.array.cols {
            self.col = 0;
            self.row += 1;
        }
        if self.row >= self.array.rows {
            return None;
        }
        let elem = &self.array[self.row][self.col];
        self.col += 1;
        Some(elem)
    }
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
pub mod human_readable_usize {
use std::fmt::Debug;
pub struct HumanReadableUsize(pub usize);
impl Debug for HumanReadableUsize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.0;
        if v < 1000 {
            write!(f, "{}", self.0)?
        } else if v < 10_000 {
            write!(f, "{}.{}K", v / 1000, (v % 1000) / 100)?
        } else if v < 1_000_000 {
            write!(f, "{}K", v / 1000)?
        } else if v < 10_000_000 {
            write!(f, "{}.{}M", v / 1_000_000, (v % 1_000_000) / (100_000))?
        } else {
            write!(f, "{}M", v / 1_000_000)?
        }
        Ok(())
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
}
pub trait Signum {
    fn signum(&self) -> i32;
}
pub trait Number: Copy + Add<
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
        to_i32(self) -> i32 { self as i32 } fn to_f64(self) -> f64 { self as f64 } }
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
pub mod rand {
use crate::algo_lib::misc::gen_vector::gen_vec;
use crate::algo_lib::misc::num_traits::Number;
use std::ops::Range;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct Random {
    state: u64,
}
impl Random {
    pub fn gen_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
    #[allow(dead_code)]
    pub fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        (from as u64 + self.gen_u64() % ((to - from) as u64)) as usize
    }
    pub fn gen_index<T>(&mut self, a: &[T]) -> usize {
        self.gen_range(0..a.len())
    }
    #[allow(dead_code)]
    #[inline(always)]
    pub fn gen_double(&mut self) -> f64 {
        (self.gen_u64() as f64) / (usize::MAX as f64)
    }
    #[allow(dead_code)]
    pub fn new(seed: u64) -> Self {
        let state = if seed == 0 { 787788 } else { seed };
        Self { state }
    }
    pub fn new_time_seed() -> Self {
        let time = SystemTime::now();
        let seed = (time.duration_since(UNIX_EPOCH).unwrap().as_nanos() % 1_000_000_000)
            as u64;
        if seed == 0 { Self::new(787788) } else { Self::new(seed) }
    }
    #[allow(dead_code)]
    pub fn gen_permutation(&mut self, n: usize) -> Vec<usize> {
        let mut result: Vec<_> = (0..n).collect();
        for i in 0..n {
            let idx = self.next_in_range(0, i + 1);
            result.swap(i, idx);
        }
        result
    }
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in 1..a.len() {
            a.swap(i, self.gen_range(0..i + 1));
        }
    }
    pub fn gen_range<T>(&mut self, range: Range<T>) -> T
    where
        T: Number,
    {
        let from = T::to_i32(range.start);
        let to = T::to_i32(range.end);
        assert!(from < to);
        let len = (to - from) as usize;
        T::from_i32(self.next_in_range(0, len) as i32 + from)
    }
    pub fn gen_vec<T>(&mut self, n: usize, range: Range<T>) -> Vec<T>
    where
        T: Number,
    {
        gen_vec(n, |_| self.gen_range(range.clone()))
    }
    pub fn gen_nonempty_range(&mut self, n: usize) -> Range<usize> {
        let x = self.gen_range(0..n);
        let y = self.gen_range(0..n);
        if x <= y { x..y + 1 } else { y..x + 1 }
    }
    pub fn gen_bool(&mut self) -> bool {
        self.gen_range(0..2) == 0
    }
}
}
pub mod simulated_annealing {
use std::{cmp::max, collections::VecDeque, time::Instant};
use crate::f;
use crate::algo_lib::misc::human_readable_usize::HumanReadableUsize;
use super::{num_traits::HasConstants, ord_f64::OrdF64, rand::Random};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchFor {
    MinimumScore,
    MaximumScore,
}
struct AcceptRate {
    accepted_on: VecDeque<usize>,
}
impl AcceptRate {
    pub fn new() -> Self {
        Self {
            accepted_on: VecDeque::default(),
        }
    }
    pub fn add(&mut self, iter: usize) {
        self.accepted_on.push_back(iter);
        if self.accepted_on.len() > 100 {
            self.accepted_on.pop_front();
        }
    }
    pub fn get_accept_percent(&self, iter: usize) -> f64 {
        if self.accepted_on.is_empty() {
            return 0.0;
        }
        let first = *self.accepted_on.front().unwrap();
        let cnt_iters = iter - first + 1;
        let accepted = self.accepted_on.len();
        (accepted as f64) / (cnt_iters as f64) * 100.0
    }
}
struct SaveChecker {
    saved_score: OrdF64,
    saved_at_ms: f64,
}
impl SaveChecker {
    pub fn new() -> Self {
        Self {
            saved_score: OrdF64::ZERO,
            saved_at_ms: 0.0,
        }
    }
    pub fn should_save(
        &mut self,
        score: OrdF64,
        search_for: SearchFor,
        time_ms: f64,
    ) -> bool {
        if time_ms < self.saved_at_ms + 10000.0 {
            return false;
        }
        match search_for {
            SearchFor::MaximumScore => {
                if score <= self.saved_score {
                    return false;
                }
            }
            SearchFor::MinimumScore => {
                if score >= self.saved_score {
                    return false;
                }
            }
        }
        eprintln!("Save score = {}", score);
        self.saved_at_ms = time_ms;
        self.saved_score = score;
        true
    }
}
pub struct SimulatedAnnealing {
    rnd: Random,
    instant: Instant,
    max_time_millis: u128,
    search_for: SearchFor,
    start_temp: OrdF64,
    finish_temp: OrdF64,
    current_temperature: OrdF64,
    last_score: OrdF64,
    best_seen_score: OrdF64,
    last_delta: OrdF64,
    last_printed_status_iter: usize,
    max_num_status_updates: usize,
    iterations_passed: usize,
    silent: bool,
    accept_rate: AcceptRate,
    save_cheker: SaveChecker,
    out_prefix: String,
}
impl SimulatedAnnealing {
    ///
    /// Read:
    /// - https://apps.topcoder.com/forums/?module=Thread&threadID=696596&start=0
    /// - https://codeforces.com/blog/entry/94437
    ///
    pub fn new<T>(
        max_time_sec: f64,
        search_for: SearchFor,
        start_temp: f64,
        finish_temp: f64,
        start_score: T,
    ) -> Self
    where
        OrdF64: From<T>,
    {
        assert_ne!(start_temp, 0.0);
        assert_ne!(finish_temp, 0.0);
        let last_score: OrdF64 = start_score.into();
        assert!(start_temp >= finish_temp);
        let mut save_cheker = SaveChecker::new();
        save_cheker.saved_score = last_score;
        Self {
            rnd: Random::new(787788),
            instant: Instant::now(),
            max_time_millis: (max_time_sec * 1000.0) as u128,
            search_for,
            start_temp: f!(start_temp),
            finish_temp: f!(finish_temp),
            current_temperature: f!(start_temp),
            best_seen_score: last_score,
            last_score,
            last_delta: f!(0.0),
            last_printed_status_iter: 0,
            max_num_status_updates: max(max_time_sec as usize, 10),
            iterations_passed: 0,
            silent: false,
            accept_rate: AcceptRate::new(),
            save_cheker,
            out_prefix: String::new(),
        }
    }
    pub fn set_silent(&mut self, silent: bool) {
        self.silent = silent;
    }
    pub fn elapsed_ms(&self) -> f64 {
        self.instant.elapsed().as_secs_f64() * 1000.0
    }
    pub fn with_out_prefix(&mut self, prefix: String) {
        self.out_prefix = prefix;
    }
    fn print_status(&self) {
        let elapsed_ms = self.instant.elapsed().as_millis();
        eprintln!(
            "{}After {}ms ({:?} iters), % of accepted changes = {:.3}%, score is: {}, best: {}",
            self.out_prefix, elapsed_ms, HumanReadableUsize(self.iterations_passed), self
            .acceptance_percent(), self.last_score, self.best_seen_score,
        );
    }
    pub fn should_continue(&mut self) -> bool {
        let elapsed = self.instant.elapsed().as_millis();
        let part_time_elapsed = self.instant.elapsed().as_millis() as f64
            / self.max_time_millis as f64;
        self.current_temperature = self.start_temp
            * (self.finish_temp / self.start_temp).powf(part_time_elapsed);
        let status_iter = (part_time_elapsed * (self.max_num_status_updates as f64))
            as usize;
        if status_iter != self.last_printed_status_iter && !self.silent {
            self.last_printed_status_iter = status_iter;
            self.print_status();
        }
        elapsed < self.max_time_millis
    }
    pub fn should_go<T>(&mut self, new_score: T) -> bool
    where
        OrdF64: From<T>,
    {
        self.iterations_passed += 1;
        let prev_score = self.last_score;
        let new_score: OrdF64 = new_score.into();
        let delta_if_positive_is_good = {
            let delta: OrdF64 = new_score - prev_score;
            match self.search_for {
                SearchFor::MinimumScore => -delta,
                SearchFor::MaximumScore => delta,
            }
        };
        self.last_delta = delta_if_positive_is_good;
        if delta_if_positive_is_good >= f!(0.0) {
            self.last_score = new_score;
            match self.search_for {
                SearchFor::MaximumScore => {
                    if new_score > self.best_seen_score {
                        self.best_seen_score = new_score;
                    }
                }
                SearchFor::MinimumScore => {
                    if new_score < self.best_seen_score {
                        self.best_seen_score = new_score;
                    }
                }
            }
            if delta_if_positive_is_good != f!(0.0) {
                self.accept_rate.add(self.iterations_passed);
            }
            return true;
        }
        let accept_probability = std::f64::consts::E
            .powf((delta_if_positive_is_good / self.current_temperature).0);
        assert!(accept_probability <= 1.0 + 1e-9);
        assert!(accept_probability >= 0.0);
        if self.rnd.gen_double() <= accept_probability {
            self.last_score = new_score;
            self.accept_rate.add(self.iterations_passed);
            true
        } else {
            false
        }
    }
    pub fn should_save(&mut self, last_time: bool) -> bool {
        let time = if last_time { f64::MAX } else { self.elapsed_ms() };
        self.save_cheker.should_save(self.last_score, self.search_for, time)
    }
    /// Get the simulated annealing's current temperature.
    pub fn current_temperature(&self) -> f64 {
        self.current_temperature.0
    }
    /// Get the simulated annealing's last delta.
    pub fn last_delta(&self) -> f64 {
        self.last_delta.0
    }
    pub fn last_score(&self) -> f64 {
        self.last_score.0
    }
    pub fn acceptance_percent(&self) -> f64 {
        self.accept_rate.get_accept_percent(self.iterations_passed)
    }
}
}
}
}
