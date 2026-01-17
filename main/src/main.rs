// 
use crate::algo_lib::collections::sqrt_decomposition::{SqrtDecomposition, SqrtNode};

use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::misc::binary_search::binary_search_first_true;
use crate::algo_lib::misc::rand::Random;
#[target_feature(enable = "avx2")]
fn add(a: &mut [i32], delta: i32) {
    for x in a.iter_mut() {
        *x += delta;
    }
}
#[target_feature(enable = "avx2")]
fn assign(a: &mut [i32], val: i32) {
    for x in a.iter_mut() {
        *x = val;
    }
}
#[derive(Clone, Default)]
struct Block {
    mins: Vec<i32>,
    maxs: Vec<i32>,
    set_to: Option<i32>,
    add: i32,
}
impl Block {
    fn len(&self) -> usize {
        self.mins.len()
    }
}
#[target_feature(enable = "avx2")]
fn calc_one_set(a: &[i32], set_value: i32) -> u64 {
    let mut res = 0u64;
    for &x in a {
        let tmp = (x as u64) * (set_value as u64);
        res = res.overflowing_add(tmp).0;
    }
    res
}
#[target_feature(enable = "avx2")]
unsafe fn calc_avx2(a: &[i32], b: &[i32]) -> u64 {
    use core::arch::x86_64::*;
    let len = a.len();
    let mut i = 0usize;
    let mut acc0 = _mm256_setzero_si256();
    let mut acc1 = _mm256_setzero_si256();
    let mut acc2 = _mm256_setzero_si256();
    let mut acc3 = _mm256_setzero_si256();
    while i + 16 <= len {
        let va0 = _mm256_loadu_si256(a.as_ptr().add(i) as *const __m256i);
        let vb0 = _mm256_loadu_si256(b.as_ptr().add(i) as *const __m256i);
        let prod0_even = _mm256_mul_epi32(va0, vb0);
        let prod0_odd = _mm256_mul_epi32(
            _mm256_srli_epi64(va0, 32),
            _mm256_srli_epi64(vb0, 32),
        );
        acc0 = _mm256_add_epi64(acc0, prod0_even);
        acc1 = _mm256_add_epi64(acc1, prod0_odd);
        let va1 = _mm256_loadu_si256(a.as_ptr().add(i + 8) as *const __m256i);
        let vb1 = _mm256_loadu_si256(b.as_ptr().add(i + 8) as *const __m256i);
        let prod1_even = _mm256_mul_epi32(va1, vb1);
        let prod1_odd = _mm256_mul_epi32(
            _mm256_srli_epi64(va1, 32),
            _mm256_srli_epi64(vb1, 32),
        );
        acc2 = _mm256_add_epi64(acc2, prod1_even);
        acc3 = _mm256_add_epi64(acc3, prod1_odd);
        i += 16;
    }
    let sum01 = _mm256_add_epi64(acc0, acc1);
    let sum23 = _mm256_add_epi64(acc2, acc3);
    let sum = _mm256_add_epi64(sum01, sum23);
    let mut lanes = [0i64; 4];
    _mm256_storeu_si256(lanes.as_mut_ptr() as *mut __m256i, sum);
    let mut res = 0u64;
    res = res.wrapping_add(lanes[0] as u64);
    res = res.wrapping_add(lanes[1] as u64);
    res = res.wrapping_add(lanes[2] as u64);
    res = res.wrapping_add(lanes[3] as u64);
    while i < len {
        let prod = (*a.get_unchecked(i) as i64) * (*b.get_unchecked(i) as i64);
        res = res.wrapping_add(prod as u64);
        i += 1;
    }
    res
}
#[target_feature(enable = "avx2")]
pub fn calc(a: &[i32], b: &[i32]) -> u64 {
    unsafe { calc_avx2(a, b) }
}
impl SqrtNode for Block {
    type Value = i32;
    fn relax(&mut self, raw_values: &mut [Self::Value]) {
        if let Some(set_to) = self.set_to {
            unsafe {
                assign(raw_values, set_to);
            }
            self.set_to = None;
        }
        if self.add != 0 {
            unsafe {
                add(raw_values, self.add);
            }
            self.add = 0;
        }
    }
    fn rebuild(&mut self, raw_values: &[Self::Value]) {
        let mut cur_min = i32::MAX;
        let mut cur_max = i32::MIN;
        self.mins.resize(raw_values.len(), 0);
        self.maxs.resize(raw_values.len(), 0);
        for i in 0..raw_values.len() {
            cur_min = cur_min.min(raw_values[i]);
            cur_max = cur_max.max(raw_values[i]);
            self.mins[i] = cur_min;
            self.maxs[i] = cur_max;
        }
    }
}
struct Solver {
    sqrt: SqrtDecomposition<Block>,
    sum_ops: usize,
}
impl Solver {
    pub fn new(a: Vec<i32>, block_size: usize) -> Self {
        Self {
            sqrt: SqrtDecomposition::new(a, block_size, Block::default()),
            sum_ops: 0,
        }
    }
    pub fn add(&mut self, l: usize, r: usize, delta: i32) {
        self.sqrt
            .iter_mut(
                l..r,
                |part| match part {
                    crate::algo_lib::collections::sqrt_decomposition::Part::Full(
                        block,
                    ) => {
                        for x in block.mins.iter_mut() {
                            *x += delta;
                        }
                        for x in block.maxs.iter_mut() {
                            *x += delta;
                        }
                        block.add += delta;
                    }
                    crate::algo_lib::collections::sqrt_decomposition::Part::Single(
                        _,
                        value,
                    ) => {
                        *value += delta;
                    }
                },
            );
    }
    pub fn assign(&mut self, l: usize, r: usize, val: i32) {
        self.sqrt
            .iter_mut(
                l..r,
                |part| match part {
                    crate::algo_lib::collections::sqrt_decomposition::Part::Full(
                        block,
                    ) => {
                        for x in block.mins.iter_mut() {
                            *x = val;
                        }
                        for x in block.maxs.iter_mut() {
                            *x = val;
                        }
                        block.set_to = Some(val);
                        block.add = 0;
                    }
                    crate::algo_lib::collections::sqrt_decomposition::Part::Single(
                        _,
                        value,
                    ) => {
                        *value = val;
                    }
                },
            );
    }
    pub fn query(&mut self, l: usize, r: usize) -> u64 {
        let mut cur_min = i32::MAX;
        let mut cur_max = i32::MIN;
        let mut res = 0u64;
        self.sqrt
            .iter_mut(
                l..r,
                |part| match part {
                    crate::algo_lib::collections::sqrt_decomposition::Part::Full(
                        block,
                    ) => {
                        let len = block.len() as u64;
                        let my_min_start = binary_search_first_true(
                            0..len as usize,
                            |pos| block.mins[pos] < cur_min,
                        );
                        let my_max_start = binary_search_first_true(
                            0..len as usize,
                            |pos| block.maxs[pos] > cur_max,
                        );
                        unsafe {
                            let len_global_min_max = my_min_start.min(my_max_start)
                                as u64;
                            res = res
                                .overflowing_add(
                                    len_global_min_max * (cur_min as u64) * (cur_max as u64),
                                )
                                .0;
                            if my_min_start < my_max_start {
                                let tmp = calc_one_set(
                                    &block.mins[my_min_start..my_max_start],
                                    cur_max,
                                );
                                res = res.overflowing_add(tmp).0;
                            } else {
                                let tmp = calc_one_set(
                                    &block.maxs[my_max_start..my_min_start],
                                    cur_min,
                                );
                                res = res.overflowing_add(tmp).0;
                            }
                            {
                                let offset_my = my_min_start.max(my_max_start);
                                let tmp = calc(
                                    &block.mins[offset_my..],
                                    &block.maxs[offset_my..],
                                );
                                res = res.overflowing_add(tmp).0;
                                self.sum_ops += block.len() - offset_my;
                            }
                        }
                        let pos = block.len() - 1;
                        cur_min = cur_min.min(block.mins[pos]);
                        cur_max = cur_max.max(block.maxs[pos]);
                    }
                    crate::algo_lib::collections::sqrt_decomposition::Part::Single(
                        _,
                        value,
                    ) => {
                        let now_min = cur_min.min(*value);
                        let now_max = cur_max.max(*value);
                        res = res.overflowing_add((now_min as u64) * (now_max as u64)).0;
                        cur_min = now_min;
                        cur_max = now_max;
                    }
                },
            );
        res
    }
}
fn test_speed3() {
    let n = 200_000;
    let mut rnd = Random::new(123);
    let mut a = rnd.gen_vec(n, 0..10000000);
    for i in 0..a.len() {
        if i % 2 == 0 {
            a[i] = 10000000 + i as i32;
        } else {
            a[i] = 10000000 - i as i32;
        }
    }
    let mut full_res = 0;
    for _ in 0..n {
        let res = unsafe { calc(&a, &a) };
        full_res += res;
    }
    dbg!(full_res);
}
fn test_speed() {
    let n = 200_000;
    let mut rnd = Random::new(123);
    let mut a = rnd.gen_vec(n, 0..10000000);
    for i in 0..a.len() {
        if i % 2 == 0 {
            a[i] = 10000000 + i as i32;
        } else {
            a[i] = 10000000 - i as i32;
        }
    }
    let mut solver1 = Solver::new(a.clone(), 4096);
    let mut full_res = 0;
    for _ in 0..n {
        let q_type = rnd.gen_range(3..4);
        let l = rnd.gen_range(0..50);
        let r = rnd.gen_range(n - 50..n + 1);
        if q_type == 1 {
            let v = rnd.gen_range(-1000..1000);
            solver1.add(l, r, v);
        } else if q_type == 2 {
            let v = rnd.gen_range(0..10000000);
            solver1.assign(l, r, v);
        } else {
            let res = solver1.query(l, r);
            full_res ^= res;
        }
    }
    dbg!(full_res, solver1.sum_ops);
}
fn solve(input: &mut Input, out: &mut Output) {
    test_speed();
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
pub mod sqrt_decomposition {
use crate::algo_lib::misc::gen_vector::gen_vec;
use crate::algo_lib::misc::range_intersect::range_intersect;
use std::cmp::min;
use std::ops::Range;
pub trait SqrtNode: Clone {
    type Value: Clone;
    fn relax(&mut self, raw_values: &mut [Self::Value]);
    fn rebuild(&mut self, raw_values: &[Self::Value]);
}
pub struct SqrtDecomposition<T>
where
    T: SqrtNode,
{
    raw_values: Vec<T::Value>,
    block_size: usize,
    blocks: Vec<T>,
}
pub enum Part<'a, T>
where
    T: SqrtNode,
{
    Full(&'a mut T),
    Single(&'a mut T, &'a mut T::Value),
}
impl<T> SqrtDecomposition<T>
where
    T: SqrtNode,
{
    pub fn new(raw_values: Vec<T::Value>, block_size: usize, empty_block: T) -> Self {
        assert!(block_size > 0);
        let n = raw_values.len();
        let blocks_num = (n + block_size - 1) / block_size;
        let blocks = gen_vec(
            blocks_num,
            |id| {
                let mut block = empty_block.clone();
                block
                    .rebuild(
                        &raw_values[id * block_size..min((id + 1) * block_size, n)],
                    );
                block
            },
        );
        Self {
            raw_values,
            block_size,
            blocks,
        }
    }
    pub fn iter_mut<F>(&mut self, range: Range<usize>, mut f: F)
    where
        F: FnMut(Part<T>),
    {
        let first_block = range.start / self.block_size;
        let last_block = (range.end + self.block_size - 1) / self.block_size;
        let block_size = self.block_size;
        let handle_side_block = |
            id: usize,
            f: &mut F,
            block: &mut T,
            raw_values: &mut [T::Value]|
        {
            let n = raw_values.len();
            let cur_block = block_size * id..min(n, block_size * (id + 1));
            let range = range_intersect(cur_block.clone(), range.clone());
            if range == cur_block {
                f(Part::Full(block));
            } else {
                block.relax(&mut raw_values[cur_block.clone()]);
                for single in raw_values[range].iter_mut() {
                    f(Part::Single(block, single));
                }
                block.rebuild(&raw_values[cur_block]);
            }
        };
        handle_side_block(
            first_block,
            &mut f,
            &mut self.blocks[first_block],
            &mut self.raw_values,
        );
        if first_block + 1 < last_block {
            for block_id in first_block + 1..last_block - 1 {
                f(Part::Full(&mut self.blocks[block_id]))
            }
            handle_side_block(
                last_block - 1,
                &mut f,
                &mut self.blocks[last_block - 1],
                &mut self.raw_values,
            );
        }
    }
    pub fn iter_mut_only_full<F>(&mut self, range: Range<usize>, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let first_block = range.start / self.block_size;
        let last_block = (range.end + self.block_size - 1) / self.block_size;
        let block_size = self.block_size;
        let handle_side_block = |
            id: usize,
            f: &mut F,
            block: &mut T,
            raw_values: &mut [T::Value]|
        {
            let n = raw_values.len();
            let cur_block = block_size * id..min(n, block_size * (id + 1));
            let range = range_intersect(cur_block.clone(), range.clone());
            if range == cur_block {
                f(block);
            }
        };
        handle_side_block(
            first_block,
            &mut f,
            &mut self.blocks[first_block],
            &mut self.raw_values,
        );
        if first_block + 1 < last_block {
            for block_id in first_block + 1..last_block - 1 {
                if f(&mut self.blocks[block_id]) {
                    return;
                }
            }
            handle_side_block(
                last_block - 1,
                &mut f,
                &mut self.blocks[last_block - 1],
                &mut self.raw_values,
            );
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
pub mod binary_search {
use crate::algo_lib::misc::num_traits::Number;
use std::ops::Range;
pub fn binary_search_first_true<T>(range: Range<T>, mut f: impl FnMut(T) -> bool) -> T
where
    T: Number,
{
    let mut left_plus_one = range.start;
    let mut right = range.end;
    while right > left_plus_one {
        let mid = left_plus_one + (right - left_plus_one) / T::TWO;
        if f(mid) {
            right = mid;
        } else {
            left_plus_one = mid + T::ONE;
        }
    }
    right
}
pub fn binary_search_last_true<T>(
    range: Range<T>,
    mut f: impl FnMut(T) -> bool,
) -> Option<T>
where
    T: Number,
{
    let first_false = binary_search_first_true(range.clone(), |x| !f(x));
    if first_false == range.start { None } else { Some(first_false - T::ONE) }
}

}
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
pub mod range_intersect {
use crate::algo_lib::misc::num_traits::Number;
use std::cmp::{max, min};
use std::ops::Range;
pub fn range_intersect<T>(r1: Range<T>, r2: Range<T>) -> Range<T>
where
    T: Number + Ord,
{
    max(r1.start, r2.start)..min(r1.end, r2.end)
}
pub trait Shift {
    fn shift<T>(self, delta: T) -> Self
    where
        T: Number;
    fn shift_left<T>(self, delta: T) -> Self
    where
        T: Number;
}
impl Shift for Range<usize> {
    fn shift<T>(self, delta: T) -> Self
    where
        T: Number,
    {
        let start = (self.start as i32 + delta.to_i32()) as usize;
        let end = (self.end as i32 + delta.to_i32()) as usize;
        start..end
    }
    fn shift_left<T>(self, delta: T) -> Self
    where
        T: Number,
    {
        self.shift(-delta.to_i32())
    }
}
}
}
}
