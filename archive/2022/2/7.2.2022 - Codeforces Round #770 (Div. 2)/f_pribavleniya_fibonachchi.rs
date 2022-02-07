//{"name":"F. Прибавления Фибоначчи","group":"Codeforces - Codeforces Round #770 (Div. 2)","url":"https://codeforces.com/contest/1634/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5 3\n2 2 1\n0 0 0\nA 1 3\nA 1 3\nB 1 1\nB 2 2\nA 3 3\n","output":"YES\nNO\nNO\nNO\nYES\n"},{"input":"5 3 10\n2 5 0 3 5\n3 5 8 2 5\nB 2 3\nB 3 4\nA 1 2\n","output":"NO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPribavleniyaFibonachchi"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo_runtime::ModRuntime;
use algo_lib::misc::num_traits::ConvI32;
use algo_lib::{dbg, out, out_line};
use std::ops::Range;

type Mod = ModRuntime;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let m = input.i32();
    let a = input.read_vec::<i32>(n);
    let b = input.read_vec::<i32>(n);

    let mut diffs = vec![Mod::new(0, m); n + 2];

    let mut cnt_non_zeros = 0;

    let mut change = |pos: usize, delta: Mod, cnt_non_zeros: &mut i64| {
        if diffs[pos].to_i32() != 0 {
            *cnt_non_zeros -= 1;
        }
        diffs[pos] += delta;
        if diffs[pos].to_i32() != 0 {
            *cnt_non_zeros += 1;
        }
    };

    let mut fibs = vec![Mod::new(1, m); 2];
    for i in 2..=n {
        let next = fibs[i - 1] + fibs[i - 2];
        fibs.push(next);
    }

    let mut add = |range: Range<usize>, mult: Mod, cnt_non_zeros: &mut i64| {
        change(range.start, mult, cnt_non_zeros);
        let len = range.len();
        change(range.end, -mult * fibs[len], cnt_non_zeros);
        change(range.end + 1, -mult * fibs[len - 1], cnt_non_zeros);
    };

    for i in 0..a.len() {
        let need = Mod::new(a[i], m) - Mod::new(b[i], m);
        add(i..i + 1, need, &mut cnt_non_zeros);
    }

    for _ in 0..q {
        let mult = if input.string()[0] == b'A' {
            Mod::new(1, m)
        } else {
            Mod::new(-1, m)
        };
        let left = input.usize() - 1;
        let right = input.usize();
        add(left..right, mult, &mut cnt_non_zeros);
        if cnt_non_zeros == 0 {
            out_line!("YES");
        } else {
            out_line!("NO");
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
