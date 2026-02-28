// 
use std::collections::{BTreeMap, BTreeSet};
use std::vec;

use crate::algo_lib::graph::trees::centroid_decomposition::CentroidDecomposition;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::misc::rand::Random;
use crate::algo_lib::misc::two_min::TwoMin;
fn conv_colors(c: u8) -> usize {
    match c {
        b'R' => 0,
        b'G' => 1,
        b'Y' => 2,
        _ => unreachable!(),
    }
}
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy, Debug)]
struct Idx {
    dist: usize,
    repr: usize,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Value {
    dist: usize,
    v: usize,
}
#[derive(Clone, Debug)]
struct Paths {
    for_repr: BTreeMap<usize, BTreeSet<Value>>,
    all_repr: BTreeSet<Idx>,
}
impl Paths {
    fn new() -> Self {
        Self {
            for_repr: BTreeMap::new(),
            all_repr: BTreeSet::new(),
        }
    }
    fn add(&mut self, idx: Idx, v: usize) {
        let repr = idx.repr;
        if let Some(set) = self.for_repr.get(&repr) {
            let best = set.iter().next().unwrap();
            self.all_repr.remove(&Idx { dist: best.dist, repr });
        }
        self.for_repr.entry(repr).or_default().insert(Value { dist: idx.dist, v });
        let best = self.for_repr[&repr].iter().next().unwrap();
        self.all_repr.insert(Idx { dist: best.dist, repr });
    }
    fn remove(&mut self, idx: Idx, v: usize) {
        let repr = idx.repr;
        let set = self.for_repr.get_mut(&repr).unwrap();
        set.remove(&Value { dist: idx.dist, v });
        self.all_repr.remove(&Idx { dist: idx.dist, repr });
        if let Some(best) = set.iter().next() {
            self.all_repr.insert(Idx { dist: best.dist, repr });
        } else {
            self.for_repr.remove(&repr);
        }
    }
    fn get_two_min(&self) -> TwoMin<usize, usize> {
        let mut res = TwoMin::new(usize::MAX, usize::MAX / 3);
        for idx in self.all_repr.iter().take(2) {
            res.add(idx.repr, idx.dist);
        }
        res
    }
}
struct Solver {
    centroid: CentroidDecomposition,
    paths: Vec<Vec<Paths>>,
    color: Vec<usize>,
}
impl Solver {
    fn new(g: &[Vec<usize>], color: &[usize]) -> Self {
        let mut centroid = CentroidDecomposition::new(g);
        let mut paths = vec![vec![Paths::new(); g.len()]; 2];
        for v in 0..g.len() {
            let my_color = color[v];
            if color[v] == 2 {
                continue;
            }
            for remote in &centroid.ups[v] {
                paths[my_color][remote.to]
                    .add(
                        Idx {
                            dist: remote.dist as usize,
                            repr: remote.last_on_path,
                        },
                        v,
                    );
            }
        }
        Self {
            centroid,
            paths,
            color: color.to_vec(),
        }
    }
    fn update_color(&mut self, v: usize, c: usize) {
        if self.color[v] != 2 {
            for remote in &self.centroid.ups[v] {
                self.paths[self.color[v]][remote.to]
                    .remove(
                        Idx {
                            dist: remote.dist as usize,
                            repr: remote.last_on_path,
                        },
                        v,
                    );
            }
        }
        self.color[v] = c;
        if self.color[v] != 2 {
            for remote in &self.centroid.ups[v] {
                self.paths[self.color[v]][remote.to]
                    .add(
                        Idx {
                            dist: remote.dist as usize,
                            repr: remote.last_on_path,
                        },
                        v,
                    );
            }
        }
    }
    fn query(&self, v: usize) -> i64 {
        let mut mins = [self.paths[0][v].get_two_min(), self.paths[1][v].get_two_min()];
        for remote in &self.centroid.ups[v] {
            if remote.to == v {
                continue;
            }
            for color in 0..2 {
                let remote_two_mins = self.paths[color][remote.to].get_two_min();
                if let Some(dist) = remote_two_mins
                    .get_value_by_not_id(remote.last_on_path)
                {
                    mins[color].add(remote.first_on_path, dist + remote.dist as usize);
                }
            }
        }
        let mut res = usize::MAX;
        for min0 in mins[0].get_values() {
            for min1 in mins[1].get_values() {
                if min0.0 != min1.0 {
                    res = res.min(min0.1 + min1.1);
                }
            }
        }
        if res >= usize::MAX / 5 { -1 } else { res as i64 }
    }
}
struct SolverSimple {
    g: Vec<Vec<usize>>,
    color: Vec<usize>,
}
impl SolverSimple {
    fn new(g: &[Vec<usize>], color: &[usize]) -> Self {
        Self {
            g: g.to_vec(),
            color: color.to_vec(),
        }
    }
    fn update_color(&mut self, v: usize, c: usize) {
        self.color[v] = c;
    }
    fn query(&self, v: usize) -> i64 {
        assert_eq!(self.color[v], 2);
        let mut res = i64::MAX;
        for start in 0..self.g.len() {
            if self.color[start] == 0 {
                let dist = self.dfs(start, start, v, false);
                res = res.min(dist);
            }
        }
        if res >= i64::MAX / 10 {
            res = -1;
        }
        res
    }
    fn dfs(&self, v: usize, p: usize, mid: usize, seen_mid: bool) -> i64 {
        let mut res = i64::MAX / 2;
        if seen_mid && self.color[v] == 1 {
            return 0;
        }
        for &to in &self.g[v] {
            if to != p {
                res = res.min(self.dfs(to, v, mid, seen_mid || to == mid) + 1);
            }
        }
        res
    }
}
fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let q = input.usize();
        let mut g = vec![vec![]; n];
        for _ in 0..(n - 1) {
            let u = input.usize() - 1;
            let v = input.usize() - 1;
            g[u].push(v);
            g[v].push(u);
        }
        let start_colors = input.string();
        let mut color = vec![0; n];
        for i in 0..n {
            color[i] = conv_colors(start_colors[i]);
        }
        let mut solver = Solver::new(&g, &color);
        for _ in 0..q {
            let q_type = input.usize();
            if q_type == 1 {
                let v = input.usize() - 1;
                let c = conv_colors(input.string()[0]);
                solver.update_color(v, c);
            } else {
                assert_eq!(q_type, 2);
                let v = input.usize() - 1;
                let res = solver.query(v);
                out.println(res);
            }
        }
    }
}
fn stress() {
    for it in 13470.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..129);
        let mut g = vec![vec![]; n];
        for i in 1..n {
            let v = rnd.gen_range(0..i);
            g[v].push(i);
            g[i].push(v);
        }
        let mut color = vec![0; n];
        for i in 0..n {
            color[i] = rnd.gen_range(0..3);
        }
        let mut solver = Solver::new(&g, &color);
        let mut solver_simple = SolverSimple::new(&g, &color);
        for _ in 0..100 {
            let v = rnd.gen_range(0..n);
            if rnd.gen_bool() && color[v] == 2 {
                let res = solver.query(v);
                let res_simple = solver_simple.query(v);
                assert_eq!(res, res_simple);
            } else {
                let c = rnd.gen_range(0..3);
                solver.update_color(v, c);
                solver_simple.update_color(v, c);
                color[v] = c;
            }
        }
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
pub mod graph {
pub mod trees {
pub mod centroid_decomposition {
#[derive(Clone, Copy, Debug)]
pub struct Remote {
    pub to: usize,
    pub dist: i64,
    pub first_on_path: usize,
    pub last_on_path: usize,
}
pub struct CentroidDecomposition {
    alive: Vec<bool>,
    size: Vec<usize>,
    pub ups: Vec<Vec<Remote>>,
}
impl CentroidDecomposition {
    pub fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut res = Self {
            alive: vec![true; n],
            size: vec![0; n],
            ups: vec![vec![]; n],
        };
        res.rec(g, 0);
        res
    }
    fn rec(&mut self, g: &[Vec<usize>], mut root: usize) {
        self.calc_sizes(g, root, root);
        let full_size = self.size[root];
        let mut prev = root;
        loop {
            let mut found = false;
            for &to in &g[root] {
                if to != prev && self.alive[to] && self.size[to] * 2 > full_size {
                    prev = root;
                    root = to;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        self.alive[root] = false;
        self.build_paths(g, root, root, 0, root, root, root);
        for &to in &g[root] {
            if self.alive[to] {
                self.rec(g, to);
            }
        }
    }
    fn calc_sizes(&mut self, g: &[Vec<usize>], v: usize, p: usize) {
        self.size[v] = 1;
        for &to in &g[v] {
            if to != p && self.alive[to] {
                self.calc_sizes(g, to, v);
                self.size[v] += self.size[to];
            }
        }
    }
    fn build_paths(
        &mut self,
        g: &[Vec<usize>],
        v: usize,
        p: usize,
        dist: i64,
        centroid: usize,
        first_on_path: usize,
        last_on_path: usize,
    ) {
        self.ups[v]
            .push(Remote {
                to: centroid,
                dist,
                first_on_path: last_on_path,
                last_on_path: first_on_path,
            });
        for &to in &g[v] {
            if to != p && self.alive[to] {
                let mut next_first_on_path = first_on_path;
                if first_on_path == centroid {
                    next_first_on_path = to;
                }
                self.build_paths(g, to, v, dist + 1, centroid, next_first_on_path, v);
            }
        }
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
pub mod two_min {
#[derive(Clone, Debug)]
pub struct TwoMin<IdType: Eq, ValueType: Ord> {
    cnt: usize,
    values: [(IdType, ValueType); 2],
}
impl<IdType: Eq + Copy, ValueType: Ord + Copy> TwoMin<IdType, ValueType> {
    pub fn new(zero_id: IdType, zero_value: ValueType) -> Self {
        Self {
            cnt: 0,
            values: [(zero_id, zero_value), (zero_id, zero_value)],
        }
    }
    fn make_sorted(&mut self) {
        if self.cnt == 2 && self.values[0].1 > self.values[1].1 {
            self.values.swap(0, 1);
        }
    }
    pub fn get_values(&self) -> &[(IdType, ValueType)] {
        &self.values[..self.cnt]
    }
    pub fn add(&mut self, id: IdType, value: ValueType) -> bool {
        if self.cnt >= 1 && self.values[0].0 == id {
            if self.values[0].1 <= value {
                return false;
            }
            self.values[0].1 = value;
            return true;
        }
        if self.cnt >= 2 && self.values[1].0 == id {
            if self.values[1].1 <= value {
                return false;
            }
            self.values[1].1 = value;
            self.make_sorted();
            return true;
        }
        if self.cnt == 0 {
            self.cnt = 1;
            self.values[0] = (id, value);
            return true;
        }
        if self.cnt == 1 {
            self.cnt += 1;
            self.values[1] = (id, value);
            self.make_sorted();
            return true;
        }
        if self.cnt == 2 {
            if self.values[1].1 <= value {
                return false;
            }
            self.values[1] = (id, value);
            self.make_sorted();
            return true;
        }
        unreachable!("cnt is greater than 2?");
    }
    pub fn merge(&mut self, another: &Self) {
        for i in 0..another.cnt {
            self.add(another.values[i].0, another.values[i].1);
        }
    }
    pub fn get_value_by_id(&self, id: IdType) -> Option<ValueType> {
        if self.cnt >= 1 && self.values[0].0 == id {
            return Some(self.values[0].1);
        }
        if self.cnt >= 2 && self.values[1].0 == id {
            return Some(self.values[1].1);
        }
        None
    }
    pub fn get_value_by_not_id(&self, not_id: IdType) -> Option<ValueType> {
        if self.cnt >= 1 && self.values[0].0 != not_id {
            return Some(self.values[0].1);
        }
        if self.cnt >= 2 && self.values[1].0 != not_id {
            return Some(self.values[1].1);
        }
        None
    }
    pub fn get_by_not_id(&self, not_id: IdType) -> Option<(IdType, ValueType)> {
        if self.cnt >= 1 && self.values[0].0 != not_id {
            return Some((self.values[0].0, self.values[0].1));
        }
        if self.cnt >= 2 && self.values[1].0 != not_id {
            return Some((self.values[1].0, self.values[1].1));
        }
        None
    }
}
}
}
}
