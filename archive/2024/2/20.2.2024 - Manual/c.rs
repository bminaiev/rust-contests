//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use std::cmp::min;
use std::ops::Range;
use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};

type Mod = Mod_998_244_353;

#[derive(Clone)]
struct Ways {
    ways: Vec<Mod>,
    offset: usize,
}

impl Ways {
    pub fn new() -> Ways {
        Self {
            ways: vec![Mod::ONE],
            offset: 0,
        }
    }

    pub fn add(&self, left: usize, right: usize) -> Self {
        let mut new_ways = vec![Mod::ZERO; self.ways.len() + right + 1];
        for i in 0..self.ways.len() {
            new_ways[i + left] += self.ways[i];
            new_ways[i + right + 1] -= self.ways[i];
        }
        for i in 1..new_ways.len() {
            let tmp = new_ways[i - 1];
            new_ways[i] += tmp;
        }
        Self {
            ways: new_ways,
            offset: self.offset,
        }
    }

    pub fn remove(&self, left: usize, right: usize) -> Self {
        let mut res = self.add(0, right - left);
        res.offset += right;
        res
    }

    pub fn get_ans(&self) -> Mod {
        if self.offset < self.ways.len() {
            self.ways[self.offset]
        } else {
            Mod::ZERO
        }
    }

    pub fn truncate(&mut self, len: usize) {
        self.ways.truncate(len);
        while self.ways.last() == Some(&Mod::ZERO) {
            self.ways.pop();
        }
    }

    pub fn truncate_first(&mut self, len: usize) {
        self.ways.drain(0..len);
        self.offset -= len;
        let mut first_non_zero = 0;
        while first_non_zero < self.ways.len() && self.ways[first_non_zero] == Mod::ZERO {
            first_non_zero += 1;
        }
        self.offset += first_non_zero;
        self.ways.drain(0..first_non_zero);
    }
}

fn stress() {
    const MAX_N: usize = 5;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(7878 + it);
        let n = rnd.gen(1..MAX_N);
        let mut a = vec![];
        for _ in 0..n {
            let mut l = rnd.gen(1..MAX_N);
            let mut r = rnd.gen(1..MAX_N);
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            a.push((l, r));
        }
        let m = rnd.gen(1..MAX_N);
        let mut b = vec![];
        for _ in 0..m {
            let mut l = rnd.gen(1..MAX_N);
            let mut r = rnd.gen(1..MAX_N);
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            b.push((l, r));
        }
        let s1 = solve_case(&a, &b, false);
        let s2 = solve_case(&a, &b, true);
        assert_eq!(s1, s2);
    }
}

fn stress2() {
    const MAX_N: usize = 500;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(7878 + it);
        let n = MAX_N; //rnd.gen(1..MAX_N);
        let mut a = vec![];
        for _ in 0..n {
            let mut l = 1; //rnd.gen(1..MAX_N);
            let mut r = MAX_N; //rnd.gen(1..MAX_N);
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            a.push((l, r));
        }
        let m = MAX_N; //rnd.gen(1..MAX_N);
        let mut b = vec![];
        for _ in 0..m {
            let mut l = 1; //rnd.gen(1..MAX_N);
            let mut r = MAX_N; //rnd.gen(1..MAX_N);
            if l > r {
                std::mem::swap(&mut l, &mut r);
            }
            b.push((l, r));
        }
        let mut start = Instant::now();
        let s2 = solve_case(&a, &b, true);
        dbg!(start.elapsed());
    }
}

fn solve_case(a: &[(usize, usize)], b: &[(usize, usize)], rem: bool) -> Array2D<Mod> {
    let n = a.len();
    let m = b.len();
    let mut res = Array2D::new(Mod::ZERO, n + 1, m + 1);

    let mut zz = 0;
    RecursiveFunction3::new(
        |f, mut cur_ways: Ways, r1: Range<usize>, r2: Range<usize>| {
            if r1.len() == 1 && r2.len() == 1 {
                res[r1.start][r2.start] = cur_ways.get_ans();
                return;
            }
            let mut sum_add = 0;
            let mut sum_sub = 0;
            for i in r1.start..r1.end - 1 {
                sum_add += a[i].1;
            }
            for i in r2.start..r2.end - 1 {
                sum_sub += b[i].1;
            }
            zz += cur_ways.ways.len();
            if rem {
                cur_ways.truncate(cur_ways.offset + sum_sub + 1);
                if cur_ways.offset > sum_add {
                    let sub = min(cur_ways.ways.len(), cur_ways.offset - sum_add);
                    cur_ways.truncate_first(sub);
                }
            }
            if r1.len() > r2.len() {
                let mid = (r1.start + r1.end) / 2;
                f.call(cur_ways.clone(), r1.start..mid, r2.clone());
                let mut new_ways = cur_ways.clone();
                for i in r1.start..mid {
                    new_ways = new_ways.add(a[i].0, a[i].1);
                }
                f.call(new_ways, mid..r1.end, r2);
            } else {
                let mid = (r2.start + r2.end) / 2;
                f.call(cur_ways.clone(), r1.clone(), r2.start..mid);
                let mut new_ways = cur_ways.clone();
                for i in r2.start..mid {
                    new_ways = new_ways.remove(b[i].0, b[i].1);
                }
                f.call(new_ways, r1, mid..r2.end);
            }
        },
    )
    .call(Ways::new(), 0..n + 1, 0..m + 1);
    dbg!(zz);
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = vec![];
    for _ in 0..n {
        a.push((input.usize(), input.usize()));
    }
    let mut b = vec![];
    for _ in 0..m {
        b.push((input.usize(), input.usize()));
    }
    let res = solve_case(&a, &b, true);

    for i in 1..=n {
        for j in 1..=m {
            out.print(res[i][j]);
            out.print(" ");
        }
        out.println("");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress2);
    // run_locally(run);
}
//END MAIN
