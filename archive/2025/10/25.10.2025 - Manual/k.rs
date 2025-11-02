//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::fenwick::Fenwick;

#[derive(Clone, Copy)]
struct Query {
    left: usize,
    right: usize,
    ops: i64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct MidQuery {
    val: i64,
    id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Value {
    value: i64,
    pos: usize,
}

type Fenw = Fenwick<i64>;

type Mod = Mod_998_244_353;

#[derive(Clone)]
pub struct FenwickProd {
    values: Vec<Mod>,
}

impl FenwickProd {
    #[allow(dead_code)]
    pub fn get_sum(&self, mut pos: usize) -> Mod {
        let mut res = Mod::ONE;
        loop {
            res *= self.values[pos];
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, mut pos: usize, change: Mod) {
        while pos < self.values.len() {
            self.values[pos] *= change;
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let values = vec![Mod::ONE; n];
        FenwickProd { values }
    }
}

fn solve_case(a: &[i64], queries: &[Query]) -> Vec<Mod> {
    let mut values = vec![];
    let n = a.len();
    let q = queries.len();
    for i in 0..n {
        values.push(Value {
            value: a[i],
            pos: i,
        });
    }
    values.sort();

    const MX: i64 = 998244999;
    let mut left = vec![0; q];
    let mut extra_left = vec![0; q];
    for i in 0..q {
        extra_left[i] = queries[i].ops;
    }
    let mut right = vec![MX; q];
    // invariant:
    // we can change all values <= left[q]

    for _iter in 0..31 {
        let mut mids = vec![];
        for id in 0..q {
            let mid = (left[id] + right[id]) / 2;
            mids.push(MidQuery { val: mid, id });
        }
        mids.sort();
        let mut it = 0;
        let mut f = Fenw::new(n);
        for mid in mids.iter() {
            while it < values.len() && values[it].value <= mid.val {
                f.add(values[it].pos, values[it].value - 1);
                it += 1;
            }
            let sum = f.get_range_sum(queries[mid.id].left..queries[mid.id].right);
            if sum <= queries[mid.id].ops {
                left[mid.id] = mid.val;
                extra_left[mid.id] = queries[mid.id].ops - sum;
            } else {
                right[mid.id] = mid.val;
            }
        }
    }
    for i in 0..q {
        assert_eq!(left[i] + 1, right[i]);
    }
    values.reverse();
    let mut results = vec![Mod::ZERO; q];
    {
        let mut f = FenwickProd::new(n);
        let mut mids = vec![];
        for id in 0..q {
            mids.push(MidQuery { val: left[id], id });
        }
        mids.sort();
        mids.reverse();
        let mut it = 0;
        for mid in mids.iter() {
            while it < values.len() && values[it].value > mid.val {
                f.add(values[it].pos, Mod::new(values[it].value));
                it += 1;
            }
            let mut prod = f.get_sum(queries[mid.id].right - 1)
                * if queries[mid.id].left == 0 {
                    Mod::ONE
                } else {
                    f.get_sum(queries[mid.id].left - 1).inv()
                };
            let extra = extra_left[mid.id];
            let value = left[mid.id] + 1;
            if value != MX {
                let cnt_full = extra / (value - 1);
                prod /= Mod::new(value).pown((cnt_full + 1) as usize);
                let part_extra = extra - cnt_full * (value - 1);
                assert!(value - part_extra > 1);
                prod *= Mod::new(value - part_extra);
            }
            results[mid.id] = prod;
        }
    }
    results
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i64>(n);
    let mut queries = vec![];
    for _ in 0..q {
        let left = input.usize() - 1;
        let right = input.usize();
        let ops = input.i64();
        queries.push(Query { left, right, ops });
    }
    let results = solve_case(&a, &queries);
    for &r in results.iter() {
        out.println(r);
    }
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rng = Random::new(it + 123);
        let n = rng.gen_range(1..10);
        let q = rng.gen_range(1..10);
        let a = rng.gen_vec(n, 0..100);
        let mut queries = vec![];
        for _ in 0..q {
            let left = rng.gen_range(0..n);
            let right = rng.gen_range(left + 1..n + 1);
            let ops = rng.gen_range(0..10000);
            queries.push(Query { left, right, ops });
        }
        let res1 = solve_case(&a, &queries);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "k";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
