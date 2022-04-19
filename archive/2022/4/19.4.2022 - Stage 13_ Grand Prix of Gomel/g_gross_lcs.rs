//{"name":"G. Gross LCS","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/G/","interactive":false,"timeLimit":1000,"tests":[{"input":"3 4\n5 5 8\n3 6 3 6\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GGrossLCS"}}}

use algo_lib::collections::min_priority_queue::MinPriorityQueue;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::num_traits::Number;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct Point {
    b_sub_a: i32,
    i: usize,
    m_sub_j: usize,
}

#[derive(Clone)]
pub struct Fenwick<T: Number> {
    values: Vec<T>,
    last_changed: Vec<i32>,
    timestamp: i32,
}

impl<T: Number> Fenwick<T> {
    #[allow(dead_code)]
    pub fn get_max(&self, mut pos: usize) -> T {
        pos = self.values.len() - 1 - pos;

        let mut res = T::ZERO;
        while pos < self.values.len() {
            if self.last_changed[pos] == self.timestamp {
                res.update_max(self.values[pos]);
            }
            pos |= pos + 1;
        }
        res
    }

    #[allow(dead_code)]
    pub fn add(&mut self, mut pos: usize, change: T) {
        pos = self.values.len() - 1 - pos;

        loop {
            if self.last_changed[pos] != self.timestamp {
                self.last_changed[pos] = self.timestamp;
                self.values[pos] = T::ZERO;
            }
            self.values[pos].update_max(change);
            pos = pos & (pos + 1);
            if pos == 0 {
                return;
            }
            pos -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        let values = vec![T::ZERO; n];
        let last_changed = vec![0; n];
        Fenwick {
            values,
            last_changed,
            timestamp: 0,
        }
    }

    pub fn clear(&mut self) {
        for x in self.values.iter_mut() {
            *x = T::ZERO;
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<i32>(n);
    let b = input.vec::<i32>(m);
    let mut b_ids = gen_vec(m, id);
    b_ids.sort_by_key(|&id| (b[id], (m - id)));

    let mut heap = MinPriorityQueue::new();
    let mut next_idx = vec![0; n];
    let mut gen_next = |i: usize| -> Option<Point> {
        if next_idx[i] == m {
            return None;
        }
        next_idx[i] += 1;
        let j = b_ids[next_idx[i] - 1];
        Some(Point {
            b_sub_a: b[j] - a[i],
            i,
            m_sub_j: m - j,
        })
    };
    for i in 0..n {
        heap.push(gen_next(i).unwrap());
    }
    let mut res = 0i64;
    let mut cur_delta = std::i32::MIN;
    let mut fenw = Fenwick::<i32>::new(m);
    while let Some(p) = heap.peek() {
        let p = *p;
        if p.b_sub_a != cur_delta {
            res += fenw.get_max(m - 1) as i64;
            fenw.timestamp += 1;
            cur_delta = p.b_sub_a;
            continue;
        }
        heap.pop();
        if let Some(next) = gen_next(p.i) {
            heap.push(next);
        }
        let j = m - p.m_sub_j;
        let cur = if j == 0 { 0 } else { fenw.get_max(j - 1) };
        fenw.add(j, cur + 1);
    }
    res += fenw.get_max(m - 1) as i64;
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    // tester::run_single_test("1");
}
//END MAIN
