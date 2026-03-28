// 
use crate::algo_lib::collections::array_2d::Array2D;
use crate::algo_lib::collections::bit_set::BitSet;

use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::misc::rand::Random;
fn gauss(a: &mut [BitSet]) -> bool {
    let n = a.len();
    let mut row_from = 0;
    for i in 0..n {
        let mut use_row = n;
        for j in row_from..n {
            if a[j].get(i) {
                use_row = j;
                break;
            }
        }
        if use_row == n {
            continue;
        }
        a.swap(row_from, use_row);
        for j in 0..n {
            if j != row_from && a[j].get(i) {
                a[j] ^= &a[row_from].clone();
            }
        }
        row_from += 1;
    }
    true
}
fn solve_gauss(s: &[Vec<u8>]) -> Option<Vec<Vec<bool>>> {
    let n = s.len();
    let m = s[0].len();
    let mut click = Array2D::new(BitSet::new(m + 1), n, m);
    for j in 0..m {
        click[0][j].set(j, true);
    }
    for row in 0..n - 1 {
        for col in 0..m {
            let mut my_value = BitSet::new(m + 1);
            if s[row][col] == b'1' {
                my_value.set(m, true);
            }
            if row > 0 {
                my_value ^= &click[row - 1][col];
            }
            if col > 0 {
                my_value ^= &click[row][col - 1];
            }
            if col + 1 < m {
                my_value ^= &click[row][col + 1];
            }
            my_value ^= &click[row][col];
            click[row + 1][col] = my_value;
        }
    }
    let mut a = vec![BitSet::new(m + 1); m];
    for col in 0..m {
        let mut my_value = BitSet::new(m + 1);
        if s[n - 1][col] == b'1' {
            my_value.set(m, true);
        }
        if n > 1 {
            my_value ^= &click[n - 2][col];
        }
        if col > 0 {
            my_value ^= &click[n - 1][col - 1];
        }
        if col + 1 < m {
            my_value ^= &click[n - 1][col + 1];
        }
        my_value ^= &click[n - 1][col];
        a[col] = my_value;
    }
    if !gauss(&mut a) {
        return None;
    }
    let mut vars_values = vec![false; m];
    let mut var_id = 0;
    for i in 0..m {
        while var_id < m && !a[var_id].get(i) {
            var_id += 1;
        }
        if var_id == m {
            break;
        }
        vars_values[var_id] = a[i].get(m);
        var_id += 1;
    }
    let mut ans = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            let mut res = click[i][j].get(m);
            for k in 0..m {
                if click[i][j].get(k) && vars_values[k] {
                    res = !res;
                }
            }
            ans[i][j] = res;
        }
    }
    let mut final_board = vec![vec![false; m]; n];
    for i in 0..n {
        for j in 0..m {
            if ans[i][j] {
                if i > 0 {
                    final_board[i - 1][j] = !final_board[i - 1][j];
                }
                if j > 0 {
                    final_board[i][j - 1] = !final_board[i][j - 1];
                }
                if i + 1 < n {
                    final_board[i + 1][j] = !final_board[i + 1][j];
                }
                if j + 1 < m {
                    final_board[i][j + 1] = !final_board[i][j + 1];
                }
                final_board[i][j] = !final_board[i][j];
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            let expected = s[i][j] == b'1';
            if final_board[i][j] != expected {
                return None;
            }
            assert_eq!(final_board[i] [j], expected);
        }
    }
    Some(ans)
}
fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let m = input.usize();
    let mut s = vec![];
    for _ in 0..n {
        let ss = input.string();
        s.push(ss);
    }
    if let Some(ans) = solve_gauss(&s) {
        out.println("YES");
        for i in 0..n {
            for j in 0..m {
                if ans[i][j] {
                    out.print("1");
                } else {
                    out.print("0");
                }
            }
            out.println("");
        }
    } else {
        out.println("NO");
    }
}
fn solve_slow(s: &[Vec<u8>]) -> Option<Vec<Vec<bool>>> {
    let n = s.len();
    let m = s[0].len();
    let total = n * m;
    let mut ids = Array2D::new(0, n, m);
    let mut need_mask = 0;
    for i in 0..n {
        for j in 0..m {
            ids[i][j] = i * m + j;
            if s[i][j] == b'1' {
                need_mask |= 1 << ids[i][j];
            }
        }
    }
    let mut apply = Array2D::new(0, n, m);
    for i in 0..n {
        for j in 0..m {
            for i2 in 0..n {
                for j2 in 0..m {
                    let d = i.abs_diff(i2) + j.abs_diff(j2);
                    if d <= 1 {
                        apply[i][j] |= 1 << ids[i2][j2];
                    }
                }
            }
        }
    }
    for mask in 0..1 << total {
        let mut xor_mask = 0;
        for i in 0..n {
            for j in 0..m {
                if (mask >> ids[i][j]) & 1 == 1 {
                    xor_mask ^= apply[i][j];
                }
            }
        }
        if xor_mask == need_mask {
            let mut ans = vec![vec![false; m]; n];
            for i in 0..n {
                for j in 0..m {
                    if (mask >> ids[i][j]) & 1 == 1 {
                        ans[i][j] = true;
                    }
                }
            }
            return Some(ans);
        }
    }
    None
}
fn stress() {
    const N: usize = 5;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..N);
        let m = rnd.gen_range(1..N);
        let mut s = vec![vec![b'0'; m]; n];
        for i in 0..n {
            for j in 0..m {
                if rnd.gen_bool() {
                    s[i][j] = b'1';
                }
            }
        }
        let ans_slow = solve_slow(&s);
        let ans_fast = solve_gauss(&s);
        if ans_fast != ans_slow {
            if ans_fast.is_some() && ans_slow.is_some() {
                continue;
            }
            let s_str = s
                .iter()
                .map(|row| String::from_utf8_lossy(row))
                .collect::<Vec<_>>()
                .join("\n");
            dbg!(s_str);
            dbg!(ans_slow);
            dbg!(ans_fast);
        }
        assert_eq!(ans_slow, ans_fast);
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
pub mod bit_set {
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, Not};
#[derive(Hash, Clone, Eq, PartialOrd, PartialEq, Debug)]
pub struct BitSet {
    values: Vec<u64>,
}
impl BitSet {
    pub fn calc_len(n: usize) -> usize {
        (n + 63) / 64
    }
    #[allow(unused)]
    pub fn new(n: usize) -> Self {
        Self {
            values: vec![0u64; BitSet::calc_len(n)],
        }
    }
    #[allow(unused)]
    pub fn get(&self, pos: usize) -> bool {
        (self.values[pos >> 6] >> (pos & 63)) & 1 == 1
    }
    pub fn get_u64(&self, from_pos: usize) -> u64 {
        if from_pos >= self.bit_len() {
            return 0;
        }
        if from_pos & 63 == 0 {
            self.values[from_pos >> 6]
        } else {
            let mut res = self.values[from_pos >> 6] >> (from_pos & 63);
            if from_pos + 64 < self.bit_len() {
                res |= self.values[(from_pos >> 6) + 1] << (64 - (from_pos & 63));
            }
            res
        }
    }
    pub fn shift_higher(&self, shift: usize) -> Self {
        let mut res = Self::new(self.bit_len());
        let whole = shift / 64;
        let offset = shift % 64;
        for i in 0..self.values.len() {
            if i + whole >= res.values.len() {
                break;
            }
            res.values[i + whole] |= self.values[i] << offset;
            if offset != 0 && i + whole + 1 < res.values.len() {
                res.values[i + whole + 1] |= self.values[i] >> (64 - offset);
            }
        }
        res
    }
    pub fn shift_lower(&self, shift: usize) -> Self {
        let mut res = Self::new(self.bit_len());
        let whole = shift / 64;
        let offset = shift % 64;
        for i in 0..self.values.len() {
            if i < whole {
                continue;
            }
            res.values[i - whole] |= self.values[i] >> offset;
            if offset != 0 && i - whole != 0 {
                res.values[i - whole - 1] |= self.values[i] << (64 - offset);
            }
        }
        res
    }
    #[allow(unused)]
    pub fn set(&mut self, pos: usize, val: bool) {
        if val {
            self.values[pos >> 6] |= 1u64 << (pos & 63);
        } else {
            self.values[pos >> 6] &= (1u64 << (pos & 63)).not();
        }
    }
    pub fn set_true(&mut self, pos: usize) {
        self.values[pos >> 6] |= 1u64 << (pos & 63);
    }
    #[allow(unused)]
    pub fn clear(&mut self) {
        for x in self.values.iter_mut() {
            *x = 0;
        }
    }
    fn ensure_length(&mut self, bit_len: usize) {
        let i64_len = Self::calc_len(bit_len);
        if i64_len > self.values.len() {
            self.values.resize(i64_len, 0);
        }
    }
    fn bit_len(&self) -> usize {
        self.values.len() << 6
    }
    pub fn first_not_set(&self, mut pos: usize) -> usize {
        if pos >= self.bit_len() {
            return pos;
        }
        while (pos & 63) != 0 {
            if !self.get(pos) {
                return pos;
            }
            pos += 1;
        }
        match self.values[pos >> 6..].iter().position(|x| *x != u64::MAX) {
            None => self.values.len() << 6,
            Some(idx) => {
                pos += idx * 64;
                while self.get(pos) {
                    pos += 1;
                }
                pos
            }
        }
    }
    pub fn first_set(&self, mut pos: usize) -> Option<usize> {
        if pos >= self.bit_len() {
            return None;
        }
        if (pos & 63) != 0 {
            let part = self.values[pos >> 6] >> (pos & 63);
            if part != 0 {
                return Some(pos + part.trailing_zeros() as usize);
            }
            pos = (pos | 63) + 1;
        }
        match self.values[pos >> 6..].iter().position(|x| *x != 0) {
            None => None,
            Some(idx) => {
                pos += idx * 64;
                pos += self.values[pos >> 6].trailing_zeros() as usize;
                assert!(self.get(pos));
                Some(pos)
            }
        }
    }
    #[target_feature(enable = "avx2")]
    unsafe fn bitor_assign_avx2(&mut self, rhs: &Self) {
        for (x, y) in self.values.iter_mut().zip(rhs.values.iter()) {
            *x |= *y;
        }
    }
    #[target_feature(enable = "ssse3")]
    unsafe fn bitor_assign_ssse3(&mut self, rhs: &Self) {
        for (x, y) in self.values.iter_mut().zip(rhs.values.iter()) {
            *x |= *y;
        }
    }
    pub fn count_ones(&self) -> usize {
        self.values.iter().map(|x| x.count_ones() as usize).sum()
    }
}
impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &Self) {
        self.ensure_length(rhs.bit_len());
        if is_x86_feature_detected!("avx2") {
            unsafe {
                self.bitor_assign_avx2(rhs);
            }
        } else if is_x86_feature_detected!("ssse3") {
            unsafe {
                self.bitor_assign_ssse3(rhs);
            }
        } else {
            for (x, y) in self.values.iter_mut().zip(rhs.values.iter()) {
                *x |= *y;
            }
        }
    }
}
impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &BitSet) {
        self.ensure_length(rhs.bit_len());
        let len = rhs.values.len();
        for (x, y) in self.values[0..len].iter_mut().zip(rhs.values[0..len].iter()) {
            *x &= *y;
        }
    }
}
impl BitXorAssign<&BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &BitSet) {
        self.ensure_length(rhs.bit_len());
        let len = rhs.values.len();
        for (x, y) in self.values[0..len].iter_mut().zip(rhs.values[0..len].iter()) {
            *x ^= *y;
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
}
}
