use std::collections::BTreeSet;
use std::unreachable;

use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::math::gcd::{gcd, lcm};
use crate::algo_lib::math::primes::{factorize, gen_largest_prime_table};
use crate::algo_lib::misc::rand::Random;
use crate::algo_lib::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};
type SegTree = SegTreeMax<usize>;
fn solve_case(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let largest_primes = gen_largest_prime_table(2 * n + 10);
    let mut primes_powers = vec![];
    for x in 2..largest_primes.len() {
        let fact: Vec<_> = factorize(&largest_primes, x).collect();
        if fact.len() == 1 {
            primes_powers.push(x);
        }
    }
    let mut st = SegTree::new(primes_powers.len(), |pos| MaxValNode { max_val: n, pos });
    let mut exist = vec![false; primes_powers.len()];
    for left in (0..n).rev() {
        for prime in factorize(&largest_primes, a[left]) {
            let mut real_power = 1;
            for _pw in 1..=prime.power {
                real_power *= prime.value;
                let idx = primes_powers.binary_search(&real_power).unwrap();
                let right = st.get(idx..idx + 1).max_val;
                if idx == 0 || st.get(0..idx).max_val < right {
                    if right != left + 1 {
                        exist[idx] = true;
                    }
                }
            }
        }
        for prime in factorize(&largest_primes, a[left]) {
            let mut real_power = 1;
            for _pw in 1..=prime.power {
                real_power *= prime.value;
                let idx = primes_powers.binary_search(&real_power).unwrap();
                st.update_point(
                    idx,
                    MaxValNode {
                        max_val: left,
                        pos: idx,
                    },
                );
            }
        }
    }
    for i in 0..primes_powers.len() {
        let right = st.get(i..i + 1).max_val;
        if i == 0 || st.get(0..i).max_val < right {
            if right != 0 {
                exist[i] = true;
            }
        }
    }
    let mut res = vec![];
    for i in 0..exist.len() {
        if exist[i] {
            res.push(primes_powers[i]);
        }
    }
    res
}
fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);
        let res = solve_case(&a);
        out.println(res.len());
        out.println(res);
    }
}
pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}
fn solve_slow(a: &[usize]) -> Vec<usize> {
    let n = a.len();
    let mut res = BTreeSet::new();
    for l in 0..n {
        for r in l + 1..=n {
            for test in 2.. {
                let mut cur_test = 1;
                for i in l..r {
                    cur_test = lcm(cur_test, gcd(a[i], test));
                }
                if cur_test != test {
                    res.insert(test);
                    break;
                }
            }
        }
    }
    res.into_iter().collect()
}
fn stress() {
    for it in 354.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(2..50);
        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = rnd.gen_range(1..n);
        }
        let res = solve_case(&a);
        let res_slow = solve_slow(&a);
        if res != res_slow {
            dbg!(a, res, res_slow);
            unreachable!();
        }
    }
}

fn main() {
    let input = crate::algo_lib::io::input::Input::new_stdin();
    let mut output = crate::algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
pub mod algo_lib {
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
pub mod primes {
pub fn gen_primes_table(up_to: usize) -> Vec<bool> {
    let mut is_prime = vec![true; up_to + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for p in 2..is_prime.len() {
        if is_prime[p] {
            for another in (2 * p..is_prime.len()).step_by(p) {
                is_prime[another] = false;
            }
        }
    }
    is_prime
}
pub fn gen_largest_prime_table(up_to: usize) -> Vec<usize> {
    let mut largest_prime = vec![0; up_to + 1];
    for p in 2..largest_prime.len() {
        if largest_prime[p] == 0 {
            for another in (p..largest_prime.len()).step_by(p) {
                largest_prime[another] = p;
            }
        }
    }
    largest_prime
}
pub struct PrimesIter<'a> {
    largest_primes: &'a [usize],
    value: usize,
}
#[derive(Clone, Copy)]
pub struct Prime {
    pub value: usize,
    pub power: usize,
}
pub fn factorize(largest_primes: &[usize], value: usize) -> PrimesIter<'_> {
    PrimesIter {
        largest_primes,
        value,
    }
}
impl<'a> Iterator for PrimesIter<'a> {
    type Item = Prime;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 1 {
            None
        } else {
            let prime = self.largest_primes[self.value];
            let mut power = 0;
            while self.value % prime == 0 {
                self.value /= prime;
                power += 1;
            }
            Some(Prime { value: prime, power })
        }
    }
}
pub fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= x {
        if x % i == 0 {
            return false;
        }
        i += 1;
    }
    true
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
}
pub mod seg_trees {
pub mod lazy_seg_tree {
use std::ops::Range;
use crate::algo_lib::seg_trees::seg_tree_trait::SegTreeNode;
///
/// Segment Tree
///
#[derive(Clone)]
pub struct SegTree<T: SegTreeNode> {
    n: usize,
    tree: Vec<T>,
    updates_to_push: Vec<Option<T::Update>>,
    context: T::Context,
    right_nodes: Vec<usize>,
}
impl<T: SegTreeNode> SegTree<T> {
    fn pull(&mut self, v: usize, vr: usize) {
        self.tree[v] = T::join_nodes(&self.tree[v + 1], &self.tree[vr], &self.context);
    }
    fn build(&mut self, v: usize, l: usize, r: usize, init_val: &T) {
        if l + 1 == r {
            self.tree[v] = init_val.clone();
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.build(v + 1, l, m, init_val);
            self.build(vr, m, r, init_val);
            self.pull(v, vr);
        }
    }
    fn push(&mut self, v: usize, l: usize, r: usize) {
        let update = self.updates_to_push[v].clone();
        self.updates_to_push[v] = None;
        match update {
            None => {}
            Some(update) => {
                let m = (l + r) >> 1;
                self.apply_update(v + 1, &update, m - l == 1);
                self.apply_update(v + ((r - l) & !1), &update, r - m == 1);
            }
        }
    }
    fn get_(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> T {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            return self.tree[v].clone();
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        let res = if ql >= m {
            self.get_(vr, m, r, ql, qr)
        } else if qr <= m {
            self.get_(v + 1, l, m, ql, qr)
        } else {
            T::join_nodes(
                &self.get_(v + 1, l, m, ql, qr),
                &self.get_(vr, m, r, ql, qr),
                &self.context,
            )
        };
        self.pull(v, vr);
        res
    }
    fn visit_(
        &mut self,
        v: usize,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
        f: &mut impl FnMut(&T),
    ) {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            f(&self.tree[v]);
            return;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        if ql >= m {
            self.visit_(vr, m, r, ql, qr, f);
        } else if qr <= m {
            self.visit_(v + 1, l, m, ql, qr, f)
        } else {
            self.visit_(v + 1, l, m, ql, qr, f);
            self.visit_(vr, m, r, ql, qr, f);
        };
        self.pull(v, vr);
    }
    fn join_updates(current: &mut Option<T::Update>, add: &T::Update) {
        match current {
            None => *current = Some(add.clone()),
            Some(current) => T::join_updates(current, add),
        };
    }
    fn apply_update(&mut self, v: usize, update: &T::Update, is_leaf: bool) {
        T::apply_update(&mut self.tree[v], update);
        if !is_leaf {
            Self::join_updates(&mut self.updates_to_push[v], update);
        }
    }
    fn modify_(
        &mut self,
        v: usize,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
        update: &T::Update,
    ) {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            self.apply_update(v, update, r - l == 1);
            return;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        if ql >= m {
            self.modify_(vr, m, r, ql, qr, update);
        } else if qr <= m {
            self.modify_(v + 1, l, m, ql, qr, update);
        } else {
            self.modify_(v + 1, l, m, ql, qr, update);
            self.modify_(vr, m, r, ql, qr, update);
        };
        self.pull(v, vr);
    }
    pub fn update(&mut self, range: Range<usize>, update: T::Update) {
        if range.is_empty() {
            return;
        }
        assert!(! range.is_empty());
        self.modify_(0, 0, self.n, range.start, range.end, &update);
    }
    pub fn update_point(&mut self, pos: usize, new_node: T) {
        let mut l = 0;
        let mut r = self.n;
        let mut v: usize = 0;
        let mut to_pull = vec![];
        while r - l > 1 {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.push(v, l, r);
            to_pull.push((v, vr));
            if pos < m {
                r = m;
                v = v + 1;
            } else {
                l = m;
                v = vr;
            }
        }
        self.tree[v] = new_node;
        for (v, vr) in to_pull.into_iter().rev() {
            self.pull(v, vr);
        }
    }
    fn find_last_true_(
        &mut self,
        v: usize,
        l: usize,
        r: usize,
        range: Range<usize>,
        f: &impl Fn(&T) -> bool,
    ) -> Option<usize> {
        if range.start >= r || l >= range.end {
            return None;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        if range.start <= l && r <= range.end {
            if !f(&self.tree[v]) {
                return None;
            }
            if r - l == 1 {
                return Some(l);
            }
        }
        self.push(v, l, r);
        if let Some(res) = self.find_last_true_(vr, m, r, range.clone(), f) {
            Some(res)
        } else {
            self.find_last_true_(v + 1, l, m, range, f)
        }
    }
    pub fn find_last_true(
        &mut self,
        range: Range<usize>,
        f: impl Fn(&T) -> bool,
    ) -> Option<usize> {
        self.find_last_true_(0, 0, self.n, range, &f)
    }
    pub fn get(&mut self, range: Range<usize>) -> T {
        if range.is_empty() {
            return T::default();
        }
        self.get_(0, 0, self.n, range.start, range.end)
    }
    pub fn visit(&mut self, range: Range<usize>, f: &mut impl FnMut(&T)) {
        if range.is_empty() {
            return;
        }
        self.visit_(0, 0, self.n, range.start, range.end, f);
    }
    pub fn new_with_context(
        n: usize,
        f: impl Fn(usize) -> T,
        context: T::Context,
    ) -> Self {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut res = SegTree {
            n,
            tree,
            updates_to_push,
            context,
            right_nodes: vec![],
        };
        res.build_f(0, 0, n, &f);
        res
    }
    pub fn new(n: usize, f: impl Fn(usize) -> T) -> Self
    where
        T::Context: Default,
    {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut res = SegTree {
            n,
            tree,
            updates_to_push,
            context: T::Context::default(),
            right_nodes: vec![],
        };
        res.build_f(0, 0, n, &f);
        res
    }
    fn build_f(&mut self, v: usize, l: usize, r: usize, f: &impl Fn(usize) -> T) {
        if l + 1 == r {
            self.tree[v] = f(l);
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.build_f(v + 1, l, m, f);
            self.build_f(vr, m, r, f);
            self.pull(v, vr);
        }
    }
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn expert_get_node(&self, node: usize) -> &T {
        &self.tree[node]
    }
    pub fn expert_get_left_node(&self, node: usize) -> usize {
        node + 1
    }
    fn build_right_nodes(&mut self, v: usize, l: usize, r: usize) {
        if l + 1 == r {
            self.right_nodes.push(0);
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.right_nodes.push(vr);
            self.build_right_nodes(v + 1, l, m);
            self.build_right_nodes(vr, m, r);
        }
    }
    pub fn expert_get_right_node(&mut self, node: usize) -> usize {
        if self.right_nodes.is_empty() {
            self.build_right_nodes(0, 0, self.n);
        }
        self.right_nodes[node]
    }
    pub fn expert_rebuild_nodes(
        &mut self,
        should_rebuild: impl Fn(&T, &T::Context) -> bool,
    ) {
        self.expert_rebuild_nodes_(0, 0, self.n, &should_rebuild);
    }
    fn expert_rebuild_nodes_(
        &mut self,
        v: usize,
        l: usize,
        r: usize,
        should_rebuild: &impl Fn(&T, &T::Context) -> bool,
    ) {
        if r - l <= 1 || !should_rebuild(&self.tree[v], &self.context) {
            return;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        self.expert_rebuild_nodes_(v + 1, l, m, should_rebuild);
        self.expert_rebuild_nodes_(vr, m, r, should_rebuild);
        self.pull(v, vr);
    }
    pub fn update_context(&mut self, f: impl Fn(&mut T::Context)) {
        f(&mut self.context);
    }
    pub fn get_context(&self) -> &T::Context {
        &self.context
    }
}
}
pub mod lazy_seg_tree_max {
use crate::algo_lib::seg_trees::{lazy_seg_tree::SegTree, seg_tree_trait::SegTreeNode};
#[derive(Clone, Default, Copy, Debug)]
pub struct MaxValNode<T> {
    pub max_val: T,
    pub pos: usize,
}
impl<T> SegTreeNode for MaxValNode<T>
where
    T: Default + Clone + Ord + Copy,
{
    #[allow(unused)]
    fn join_nodes(l: &Self, r: &Self, context: &()) -> Self {
        if l.max_val > r.max_val { *l } else { *r }
    }
    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_val = *update;
    }
    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }
    type Update = T;
    type Context = ();
}
pub type SegTreeMax<T> = SegTree<MaxValNode<T>>;
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
