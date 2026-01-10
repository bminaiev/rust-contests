// 
use crate::algo_lib::collections::min_priority_queue::MinPriorityQueue;

use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
#[derive(Clone, Copy)]
struct Remote {
    to: usize,
    dist: i64,
}
struct CentroidDecomposition {
    alive: Vec<bool>,
    size: Vec<usize>,
    ups: Vec<Vec<Remote>>,
    children: Vec<Vec<Remote>>,
}
impl CentroidDecomposition {
    fn new(g: &[Vec<usize>]) -> Self {
        let n = g.len();
        let mut res = Self {
            alive: vec![true; n],
            size: vec![0; n],
            ups: vec![vec![]; n],
            children: vec![vec![]; n],
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
        self.build_paths(g, root, root, 0, root);
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
    ) {
        self.ups[v].push(Remote { to: centroid, dist });
        self.children[centroid].push(Remote { to: v, dist });
        for &to in &g[v] {
            if to != p && self.alive[to] {
                self.build_paths(g, to, v, dist + 1, centroid);
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Event {
    time: i64,
    v: usize,
    lang: i64,
    visit: bool,
}
fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let u = input.usize() - 1;
        let v = input.usize() - 1;
        g[u].push(v);
        g[v].push(u);
    }
    let mut from = vec![];
    let mut to = vec![];
    for _ in 0..n {
        from.push(input.i64());
        to.push(input.i64());
    }
    if to[n - 1] < from[0] {
        for i in 0..n {
            let nfr = -to[i];
            let nto = -from[i];
            from[i] = nfr;
            to[i] = nto;
        }
    }
    let start = from[0];
    const INF: i64 = 1e18 as i64;
    for i in 0..n {
        if to[i] < start {
            from[i] = INF;
            to[i] = INF;
        } else {
            from[i] = (from[i] - start).max(0);
            to[i] = to[i] - start;
            assert!(to[i] >= from[i]);
        }
    }
    assert!(from[n - 1] != INF);
    let mut cd = CentroidDecomposition::new(&g);
    for v in 0..n {
        cd.children[v].sort_by_key(|u| from[u.to]);
    }
    let mut pq = MinPriorityQueue::new();
    pq.push(Event {
        time: 0,
        v: 0,
        lang: to[0],
        visit: true,
    });
    let mut child_iter = vec![0; n];
    let mut max_seen_lang = vec![- 1; n];
    let mut seen = vec![false; n];
    while let Some(Event { time, v, lang, visit }) = pq.pop() {
        if visit {
            if seen[v] {
                continue;
            }
            if v == n - 1 {
                out.println(time - 1);
                return;
            }
            seen[v] = true;
            for tos in &cd.ups[v] {
                pq.push(Event {
                    time: time + tos.dist,
                    v: tos.to,
                    lang,
                    visit: false,
                });
            }
        } else {
            if max_seen_lang[v] >= lang {
                continue;
            }
            max_seen_lang[v] = lang;
            while child_iter[v] < cd.children[v].len()
                && from[cd.children[v][child_iter[v]].to] <= lang
            {
                let child = &cd.children[v][child_iter[v]];
                let arrive_at = time + child.dist + 1;
                pq.push(Event {
                    time: arrive_at,
                    v: child.to,
                    lang: to[child.to],
                    visit: true,
                });
                child_iter[v] += 1;
            }
        }
    }
    unreachable!();
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
pub mod min_priority_queue {
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[derive(Default, Clone)]
pub struct MinPriorityQueue<T>(
    BinaryHeap<Reverse<T>>,
)
where
    T: Ord;
impl<T> MinPriorityQueue<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }
    pub fn with_capacity(n: usize) -> Self {
        Self(BinaryHeap::with_capacity(n))
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn peek(&self) -> Option<&T> {
        match self.0.peek() {
            None => None,
            Some(elem) => Some(&elem.0),
        }
    }
    pub fn push(&mut self, elem: T) {
        self.0.push(Reverse(elem))
    }
    pub fn pop(&mut self) -> Option<T> {
        match self.0.pop() {
            None => None,
            Some(elem) => Some(elem.0),
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|elem| &elem.0)
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
}
}
