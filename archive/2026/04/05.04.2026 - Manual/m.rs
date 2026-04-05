//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::BTreeSet;

use algo_lib::collections::fx_hash_map::FxHashMap;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Query {
    x: i64,
    id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct LinearFunction {
    a: i64,
    b: i64,
}

impl LinearFunction {
    fn eval(&self, x: i64) -> i64 {
        self.a * x + self.b
    }
}

fn when_second_better(f1: LinearFunction, f2: LinearFunction) -> i64 {
    if f1.a == f2.a {
        if f1.b <= f2.b {
            i64::MAX
        } else {
            i64::MIN
        }
    } else {
        // if f1.a > f2
        // TODO: ???
        let start = (f1.b - f2.b) / (f2.a - f1.a) + 1;
        let mut x = start;
        while f1.eval(x) >= f2.eval(x) {
            x -= 1;
        }
        assert!(start - x <= 3);
        x
    }
}

impl std::ops::Add<LinearFunction> for &LinearFunction {
    type Output = LinearFunction;

    fn add(self, rhs: LinearFunction) -> Self::Output {
        LinearFunction {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

struct Store {
    funs: BTreeSet<LinearFunction>,
    final_fun: LinearFunction,
    extra: LinearFunction,
}

impl Store {
    fn new(value: i64) -> Self {
        let fun = if value == -1 {
            LinearFunction { a: -1, b: 0 }
        } else {
            LinearFunction { a: 0, b: value }
        };
        let mut funs = BTreeSet::new();
        funs.insert(fun);
        Self {
            funs,
            final_fun: fun,
            extra: LinearFunction { a: 0, b: 0 },
        }
    }

    fn merge(mut a: Store, mut b: Store, cur_x: i64) -> Self {
        let first_b = b.funs.iter().next().unwrap() + b.extra;
        while a.funs.len() > 0 {
            let f = a.funs.iter().next_back().unwrap() + a.extra;
            if when_second_better(f, first_b) <= cur_x {
                break;
            }
        }
        while let Some(f) = a.funs.iter().next_back() {
            if when_second_better(f + a.extra, first_b) < cur_x {
                break;
            }
            a.funs.remove(f);
        }

        if a.funs.len() > b.funs.len() {
            // add all of b to a

            a
        } else {
            b
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let q = input.usize();
    let a = input.vec::<i64>(n);
    let mut queries = vec![];
    for i in 0..q {
        let x = input.i64();
        queries.push(Query { x, id: i });
    }
    queries.sort_by_key(|q| -q.x);
    let mut dsu = Dsu::new(n);
    let mut stores = FxHashMap::default();
    for i in 0..n {
        stores.insert(i, Store::new(a[i]));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "m";
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
