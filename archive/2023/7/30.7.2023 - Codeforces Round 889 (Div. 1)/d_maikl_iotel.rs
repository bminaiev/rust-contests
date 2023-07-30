//{"name":"D. Майкл и отель","group":"Codeforces - Codeforces Round 889 (Div. 1)","url":"https://codeforces.com/contest/1854/problem/D","interactive":true,"timeLimit":2000,"tests":[{"input":"5\n\n0\n\n1\n","output":"? 3 5 2 2 3\n\n? 2 5 2 2 3\n\n! 3 1 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaiklIOtel"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

trait Interactor {
    fn ask(&mut self, start: usize, len: usize, a: &[usize]) -> bool;
    fn answer(&mut self, a: &[usize]);
    fn n(&self) -> usize;
    fn n_queries(&self) -> usize;
}

struct MyInteractor {
    next: Vec<usize>,
    queries: usize,
}

impl MyInteractor {
    fn new(next: Vec<usize>) -> Self {
        Self { next, queries: 0 }
    }
}

struct RealInteractor<'a> {
    n: usize,
    n_queries: usize,
    input: &'a mut Input,
}

impl<'a> RealInteractor<'a> {
    pub fn new(input: &'a mut Input) -> Self {
        let n = input.usize();
        Self {
            input,
            n,
            n_queries: 0,
        }
    }
}

impl<'a> Interactor for RealInteractor<'a> {
    fn ask(&mut self, start: usize, len: usize, a: &[usize]) -> bool {
        out!("?", start + 1, len, a.len(), "");
        for &v in a {
            out!(v + 1, "");
        }
        out_line!();
        output().flush();
        self.n_queries += 1;
        self.input.i32() == 1
    }

    fn answer(&mut self, a: &[usize]) {
        out!("!", a.len(), "");
        for &v in a {
            out!(v + 1, "");
        }
        out_line!();
        output().flush();
    }

    fn n(&self) -> usize {
        self.n
    }

    fn n_queries(&self) -> usize {
        self.n_queries
    }
}

const MAX_QUERIES: usize = 2_000;

impl Interactor for MyInteractor {
    fn ask(&mut self, start: usize, len: usize, a: &[usize]) -> bool {
        assert!(len > 0);
        let mut cur = start;
        for _ in 0..len {
            cur = self.next[cur];
        }
        self.queries += 1;
        assert!(self.queries <= MAX_QUERIES);
        a.contains(&cur)
    }

    fn answer(&mut self, a: &[usize]) {
        let mut base = 0;
        for _ in 0..1000 {
            base = self.next[base];
        }
        let mut res = vec![];
        for i in 0..self.next.len() {
            let mut ok = false;
            let mut now = i;
            for _ in 0..1000 {
                now = self.next[now];
                if now == base {
                    ok = true;
                    break;
                }
            }
            if ok {
                res.push(i);
            }
        }
        assert_eq!(a, res);
    }

    fn n(&self) -> usize {
        self.next.len()
    }

    fn n_queries(&self) -> usize {
        self.queries
    }
}

fn binary_search(
    inter: &mut impl Interactor,
    start: usize,
    len: usize,
    possible: &[usize],
) -> usize {
    assert!(!possible.is_empty());
    if possible.len() == 1 {
        return possible[0];
    }
    let mut check = vec![];
    let mut another = vec![];
    for i in 0..possible.len() {
        if i % 2 == 0 {
            check.push(possible[i]);
        } else {
            another.push(possible[i]);
        }
    }
    if inter.ask(start, len, &check) {
        binary_search(inter, start, len, &check)
    } else {
        binary_search(inter, start, len, &another)
    }
}

fn solver_case(mut inter: impl Interactor) {
    let n = inter.n();
    let full: Vec<usize> = (0..n).collect();

    let mut cur = binary_search(&mut inter, 0, 1000, &full);
    let mut circle = vec![cur];
    for _ in 0..100 {
        let next = binary_search(&mut inter, cur, 1, &full);
        if next == circle[0] {
            break;
        }
        circle.push(next);
        cur = next;
    }
    let mut ask_len = 1234;
    while inter.n_queries() < MAX_QUERIES && circle.len() != n {
        for v in 0..n {
            if circle.contains(&v) {
                continue;
            }
            if inter.ask(v, ask_len, &circle) {
                circle.push(v);
            }
            if inter.n_queries() >= MAX_QUERIES - 5 {
                break;
            }
        }
        ask_len += 99;
    }
    circle.sort();
    inter.answer(&circle);
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(78977);
        let n = 500; //rnd.gen(1..100);
        let mut next = vec![0; n];
        for i in 0..n {
            next[i] = rnd.gen(0..n);
        }
        let circle_len = rnd.gen(1..n);
        for i in 0..circle_len {
            next[i] = (i + 1) % circle_len;
        }
        let mut inter = MyInteractor::new(next);
        solver_case(inter);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let real = RealInteractor::new(input);
    solver_case(real);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: true,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
