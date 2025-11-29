// 

use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::math::combinations::CombinationsFact;
use crate::algo_lib::math::modulo::Mod_998_244_353;
type Mod = Mod_998_244_353;
fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut xx = vec![];
    let mut yy = vec![];
    const MD: i64 = 998_244_353;
    for _ in 0..n {
        let x = (input.i64() + MD) % MD;
        let y = (input.i64() + MD) % MD;
        xx.push(Mod::new(x));
        yy.push(Mod::new(y));
    }
    let cnk = CombinationsFact::<Mod>::new(n + 1);
    let mult = vec![Mod::ZERO; n + 1];
    for d in 1..n {}
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
pub mod last_exn {
use std::collections::BTreeSet;
pub trait LastExn<T> {
    fn last_exn(&self) -> &T;
}
impl<T> LastExn<T> for &[T] {
    fn last_exn(&self) -> &T {
        self.last().unwrap()
    }
}
impl<T> LastExn<T> for Vec<T> {
    fn last_exn(&self) -> &T {
        self.last().unwrap()
    }
}
impl<T> LastExn<T> for BTreeSet<T> {
    fn last_exn(&self) -> &T {
        self.iter().next_back().unwrap()
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
pub mod math {
pub mod combinations {
use crate::algo_lib::math::factorials::gen_facts;
use crate::algo_lib::misc::num_traits::Number;
pub trait Combinations<T> {
    fn c(&self, n: usize, k: usize) -> T;
}
pub struct CombinationsFact<T> {
    fact: Vec<T>,
    fact_inv: Vec<T>,
}
impl<T> CombinationsFact<T>
where
    T: Number,
{
    #[allow(unused)]
    pub fn new(n: usize) -> Self {
        let fact = gen_facts(n);
        let mut fact_inv = fact.clone();
        assert_eq!(fact_inv.len(), n + 1);
        fact_inv[n] = T::ONE / fact_inv[n];
        for i in (1..n).rev() {
            fact_inv[i] = fact_inv[i + 1] * T::from_i32((i + 1) as i32);
        }
        Self { fact, fact_inv }
    }
    pub fn fact(&self, n: usize) -> T {
        self.fact[n]
    }
}
impl<T> Combinations<T> for CombinationsFact<T>
where
    T: Number,
{
    fn c(&self, n: usize, k: usize) -> T {
        if k > n {
            return T::ZERO;
        }
        self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
    }
}
}
pub mod factorials {
use crate::algo_lib::misc::num_traits::Number;
///
/// Generate factorials of all numbers up to `n`
///
pub fn gen_facts<T>(n: usize) -> Vec<T>
where
    T: Number,
{
    let mut res = Vec::with_capacity(n);
    res.push(T::ONE);
    for x in 1..=n {
        let num = T::from_i32(x as i32);
        res.push(*res.last().unwrap() * num);
    }
    res
}
}
pub mod modulo {
use crate::algo_lib::collections::last_exn::LastExn;
use crate::algo_lib::io::input::{Input, Readable};
use crate::algo_lib::io::output::{Output, Writable};
use crate::algo_lib::misc::num_traits::{ConvSimple, HasConstants, Number};
use std::io::Write;
use std::marker::PhantomData;
pub trait Value: Clone + Copy + Eq + Default + Ord {
    fn val() -> i32;
}
#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct ModWithValue<M>(
    i32,
    PhantomData<M>,
)
where
    M: Value;
impl<M> ModWithValue<M>
where
    M: Value,
{
    #[allow(unused)]
    pub const ZERO: Self = Self(0, PhantomData);
    #[allow(unused)]
    pub const ONE: Self = Self(1, PhantomData);
    #[allow(unused)]
    pub const TWO: Self = Self(2, PhantomData);
    fn rev_rec(a: i32, m: i32) -> i32 {
        if a == 1 {
            return a;
        }
        ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32
    }
    #[allow(dead_code)]
    pub fn inv(self) -> Self {
        ModWithValue(Self::rev_rec(self.0, M::val()), PhantomData)
    }
    pub fn value(&self) -> i32 {
        self.0
    }
    pub fn i64(&self) -> i64 {
        self.0 as i64
    }
    #[allow(dead_code)]
    pub fn new<T: Number>(x: T) -> Self {
        let mut x = x.to_i32();
        if x < 0 {
            x += M::val();
            if x < 0 {
                x %= M::val();
                x += M::val();
            }
        } else if x >= M::val() {
            x -= M::val();
            if x >= M::val() {
                x %= M::val();
            }
        }
        assert!(0 <= x && x < M::val());
        Self(x, PhantomData)
    }
    pub fn pown(self, pw: usize) -> Self {
        if pw == 0 {
            Self::ONE
        } else if pw == 1 {
            self
        } else {
            let half = self.pown(pw / 2);
            let res = half * half;
            if pw % 2 == 0 { res } else { res * self }
        }
    }
    pub fn gen_powers(base: Self, n: usize) -> Vec<Self> {
        let mut res = Vec::with_capacity(n);
        res.push(Self::ONE);
        for _ in 1..n {
            res.push(*res.last_exn() * base);
        }
        res
    }
}
impl<M> std::fmt::Display for ModWithValue<M>
where
    M: Value,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<M> std::fmt::Debug for ModWithValue<M>
where
    M: Value + Copy + Eq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const MAX: i32 = 100;
        if self.0 <= MAX {
            write!(f, "{}", self.0)
        } else if self.0 >= M::val() - MAX {
            write!(f, "-{}", M::val() - self.0)
        } else {
            for denom in 1..MAX {
                let num = *self * Self(denom, PhantomData);
                if num.0 <= MAX {
                    return write!(f, "{}/{}", num.0, denom);
                } else if num.0 >= M::val() - MAX {
                    return write!(f, "-{}/{}", M::val() - num.0, denom);
                }
            }
            write!(f, "(?? {} ??)", self.0)
        }
    }
}
impl<M> std::ops::Add for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let res = self.0 + rhs.0;
        if res >= M::val() {
            ModWithValue(res - M::val(), PhantomData)
        } else {
            ModWithValue(res, PhantomData)
        }
    }
}
impl<M> std::ops::AddAssign for ModWithValue<M>
where
    M: Value,
{
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        if self.0 >= M::val() {
            self.0 -= M::val();
        }
    }
}
impl<M> std::ops::Sub for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let res = self.0 - rhs.0;
        if res < 0 {
            ModWithValue(res + M::val(), PhantomData)
        } else {
            ModWithValue(res, PhantomData)
        }
    }
}
impl<M> std::ops::SubAssign for ModWithValue<M>
where
    M: Value,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        if self.0 < 0 {
            self.0 += M::val();
        }
    }
}
impl<M> std::ops::Mul for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let res = (self.0 as i64) * (rhs.0 as i64) % (M::val() as i64);
        ModWithValue(res as i32, PhantomData)
    }
}
impl<M> std::ops::MulAssign for ModWithValue<M>
where
    M: Value,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = ((self.0 as i64) * (rhs.0 as i64) % (M::val() as i64)) as i32;
    }
}
impl<M> std::ops::Div for ModWithValue<M>
where
    M: Value,
{
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        let rhs_inv = rhs.inv();
        self * rhs_inv
    }
}
impl<M> std::ops::DivAssign for ModWithValue<M>
where
    M: Value,
{
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv();
    }
}
impl<M> Writable for ModWithValue<M>
where
    M: Value,
{
    fn write(&self, output: &mut Output) {
        output.write_fmt(format_args!("{}", self.0)).unwrap();
    }
}
impl<M> Readable for ModWithValue<M>
where
    M: Value,
{
    fn read(input: &mut Input) -> Self {
        let i32 = input.i32();
        Self::new(i32)
    }
}
impl<M> HasConstants<ModWithValue<M>> for ModWithValue<M>
where
    M: Value,
{
    const MAX: ModWithValue<M> = ModWithValue::ZERO;
    const MIN: ModWithValue<M> = ModWithValue::ZERO;
    const ZERO: ModWithValue<M> = ModWithValue::ZERO;
    const ONE: ModWithValue<M> = ModWithValue::ONE;
    const TWO: ModWithValue<M> = ModWithValue::TWO;
}
impl<M> ConvSimple<ModWithValue<M>> for ModWithValue<M>
where
    M: Value,
{
    fn from_i32(val: i32) -> ModWithValue<M> {
        ModWithValue::new(val)
    }
    fn to_i32(self) -> i32 {
        self.0
    }
    fn to_f64(self) -> f64 {
        self.0 as f64
    }
}
pub trait ConstValue: Value + Copy {
    const VAL: i32;
}
impl<V: ConstValue> Value for V {
    fn val() -> i32 {
        Self::VAL
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct Value7();
impl ConstValue for Value7 {
    const VAL: i32 = 1_000_000_007;
}
pub type Mod7 = ModWithValue<Value7>;
#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct Value9();
impl ConstValue for Value9 {
    const VAL: i32 = 1_000_000_009;
}
pub type Mod9 = ModWithValue<Value9>;
#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd, Hash)]
pub struct Value_998_244_353();
impl ConstValue for Value_998_244_353 {
    const VAL: i32 = 998_244_353;
}
pub type Mod_998_244_353 = ModWithValue<Value_998_244_353>;
pub trait ModuloTrait: Number {
    fn mod_value() -> i32;
    fn pown(self, n: usize) -> Self;
}
impl<V: Value> ModuloTrait for ModWithValue<V> {
    fn mod_value() -> i32 {
        V::val()
    }
    fn pown(self, n: usize) -> Self {
        self.pown(n)
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
}
}
