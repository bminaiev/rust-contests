//{"name":"D2. Игра на сумму (Сложная версия)","group":"Codeforces - Codeforces Round #767 (Div. 1)","url":"https://codeforces.com/contest/1628/problem/D2","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3 3 2\n2 1 10\n6 3 10\n6 4 10\n100 1 1\n4 4 0\n69 4 20\n","output":"6\n5\n375000012\n500000026\n958557139\n0\n49735962\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2IgraNaSummuSlozhnayaVersiya"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod7;
use algo_lib::{dbg, out, out_line};
use std::cmp::min;

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k: Mod = input.read();
    let mut pow2 = vec![Mod::ONE; n + 1];
    for i in 1..=n {
        pow2[i] = pow2[i - 1] * Mod::TWO;
    }
    let mut res = Mod::ZERO;
    let cnk = CombinationsFact::<Mod>::new(n + 1);
    for x in 1..=m {
        let start = Mod::new(x as i32) * pow2[x];
        let cnt_right = n - x;
        let cnt_down = m - x;
        let ways = if cnt_right == 0 {
            Mod::ONE
        } else {
            cnk.c(cnt_right - 1, cnt_down)
        };
        res += start * ways;
    }
    let res = k * res / pow2[n];
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
