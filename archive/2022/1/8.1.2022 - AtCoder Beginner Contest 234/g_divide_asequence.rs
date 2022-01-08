//{"name":"G - Divide a Sequence","group":"AtCoder - AtCoder Beginner Contest 234","url":"https://atcoder.jp/contests/abc234/tasks/abc234_g","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 3\n","output":"2\n"},{"input":"4\n1 10 1 10\n","output":"90\n"},{"input":"10\n699498050 759726383 769395239 707559733 72435093 537050110 880264078 699299140 418322627 134917794\n","output":"877646588\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GDivideASequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

struct Elem {
    extreme_elem: i32,
    sum_f: Mod,
}

impl Elem {
    fn value(&self) -> Mod {
        Mod::new(self.extreme_elem) * self.sum_f
    }
}

struct Stack {
    sum: Mod,
    values: Vec<Elem>,
}

impl Stack {
    fn new() -> Self {
        Self {
            sum: Mod::ZERO,
            values: vec![],
        }
    }

    fn add(&mut self, ai: i32, sum_f: Mod) {
        let mut new_max_elem = Elem {
            extreme_elem: ai,
            sum_f,
        };
        while !self.values.is_empty()
            && self.values.last().unwrap().extreme_elem <= new_max_elem.extreme_elem
        {
            let last = self.values.pop().unwrap();
            self.sum -= last.value();
            new_max_elem.sum_f += last.sum_f;
        }
        self.sum += new_max_elem.value();
        self.values.push(new_max_elem);
    }
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let a: Vec<i32> = input.read_vec(n);
    let mut f = vec![Mod::ZERO; n + 1];
    f[0] = Mod::ONE;

    let mut max_stack = Stack::new();
    let mut min_stack = Stack::new();

    for i in 0..n {
        max_stack.add(a[i], f[i]);
        min_stack.add(-a[i], f[i]);

        f[i + 1] += max_stack.sum + min_stack.sum;
    }
    out_line!(f[n]);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
