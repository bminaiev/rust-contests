//{"name":"F - Delete 1, 4, 7, ...","group":"AtCoder - AtCoder Regular Contest 135","url":"https://atcoder.jp/contests/arc135/tasks/arc135_f","interactive":false,"timeLimit":2000,"tests":[{"input":"10 2\n","output":"25\n"},{"input":"10 10\n","output":"0\n"},{"input":"10000 10\n","output":"862816\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FDelete147"}}}

use std::cmp::min;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Seq {
    add: i64,
    starts: Vec<i64>,
}

const BIG_NUM: i64 = 1e15 as i64;

impl Seq {
    fn new(add: i64, starts: Vec<i64>) -> Self {
        Self { add, starts }
    }

    fn get(&self, pos: i64) -> i64 {
        self.starts[(pos % (self.starts.len() as i64)) as usize]
            + self.add * (pos / self.starts.len() as i64)
    }

    fn next(&self) -> Self {
        let mut starts = vec![];
        for i in 0..self.starts.len() {
            for delta in 1..=2 {
                let value = self.get((i * 3 + delta) as i64);
                starts.push(value);
            }
        }
        Self {
            starts,
            add: min(BIG_NUM, self.add * 3),
        }
    }

    fn multiply(s1: &Self, s2: &Self, n: i64) -> Self {
        let l2 = s2.starts.len();
        let mut starts = vec![];
        for it in 0..l2 {
            let mut breaked = false;
            for &index in s1.starts.iter() {
                let pos = index + s1.add * (it as i64);
                let value = s2.get(pos);
                if value > n {
                    breaked = true;
                    break;
                }
                starts.push(value);
            }
            if breaked {
                break;
            }
        }
        let add = if (s1.add as f64) * (s2.add as f64) > 1e15 {
            BIG_NUM
        } else {
            min(BIG_NUM, s1.add * s2.add)
        };
        Self { starts, add }
    }

    fn remove_big(&mut self, n: i64) {
        while !self.starts.is_empty() && *self.starts.last_exn() > n {
            self.starts.pop();
        }
    }
}

fn solve(input: &mut Input) {
    let n = input.i64();
    let tot_steps = input.usize();
    let mut by_lvl = vec![Seq::new(1, vec![0i64])];
    for cur_step in 0..20 {
        dbg!(cur_step);
        let mut next = by_lvl.last_exn().next();
        next.remove_big(n);
        dbg!(next.starts.len());
        by_lvl.push(next);
    }
    let mut start_index = min(tot_steps, by_lvl.len() - 1);
    let mut cur = by_lvl[start_index].clone();
    while start_index != tot_steps {
        let can_add = min(tot_steps - start_index, by_lvl.len() - 1);
        cur = Seq::multiply(&cur, &by_lvl[can_add], n);
        start_index += can_add;
    }
    const MOD_VAL: i64 = 998_244_353;
    let mut res = Mod::ZERO;
    for &start in cur.starts.iter() {
        let start = start + 1;
        if start > n {
            break;
        }
        let cnt = 1 + (n - start) / cur.add;
        let last = start + (cnt - 1) * cur.add;
        let sum = (last + start) % MOD_VAL;
        res += Mod::new(sum as i32) * Mod::new((cnt % MOD_VAL) as i32) / Mod::TWO;
    }
    let test = Seq::multiply(&by_lvl[3], &by_lvl[5], n);
    let expect = by_lvl[8].clone();
    assert_eq!(test, expect);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("4");
}
//END MAIN
