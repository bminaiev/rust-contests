
use crate::algo_lib::geometry::point::PointT;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
type Point = PointT<i64>;
fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut points = Vec::with_capacity(n);
    let mut vs = vec![];
    for _ in 0..n {
        let x = input.i64();
        let y = input.i64();
        points.push(Point::new(x, y));
        vs.push(input.f64());
    }
    let mut left = 0.0;
    let mut right = 1e9;
    for _ in 0..100 {
        let t = (left + right) / 2.0;
        let mut minx = std::f64::NEG_INFINITY;
        let mut maxx = std::f64::INFINITY;
        let mut miny = std::f64::NEG_INFINITY;
        let mut maxy = std::f64::INFINITY;
        for i in 0..n {
            let x = (points[i].x + points[i].y) as f64;
            let y = (points[i].x - points[i].y) as f64;
            let cur_minx = x - vs[i] * t;
            let cur_maxx = x + vs[i] * t;
            let cur_miny = y - vs[i] * t;
            let cur_maxy = y + vs[i] * t;
            if cur_minx > minx {
                minx = cur_minx;
            }
            if cur_maxx < maxx {
                maxx = cur_maxx;
            }
            if cur_miny > miny {
                miny = cur_miny;
            }
            if cur_maxy < maxy {
                maxy = cur_maxy;
            }
        }
        if minx <= maxx && miny <= maxy {
            right = t;
        } else {
            left = t;
        }
    }
    out.println((left + right) / 2.0);
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
}
}
