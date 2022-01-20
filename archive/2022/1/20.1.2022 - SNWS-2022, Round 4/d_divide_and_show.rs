//{"name":"D. Divide And Show","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/D/","interactive":false,"timeLimit":2000,"tests":[{"input":"7 4\n4\n0 2 5 4\n0 2 7 2\n0 1 5 4\n2 0 7 1\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDivideAndShow"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::fenwick::Fenwick;
use algo_lib::{dbg, out, out_line};
use std::cmp::{max, min};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize();
    let mut read_pos = || -> usize {
        let x = input.usize();
        let y = input.usize();
        if x == 0 {
            return y;
        }
        if y == m {
            return m + x;
        }
        if x == n {
            return n + m + m - y;
        }
        return (n + m) * 2 - x;
    };
    let total_max = (n + m) * 2;
    let mut fenw = Fenwick::new(total_max);
    let all = gen_vec(k, |_| (read_pos(), read_pos()));
    for &(fr, to) in all.iter() {
        fenw.add(fr, 1);
        fenw.add(to, 1);
    }
    for &(fr, to) in all.iter() {
        let min = min(fr, to);
        let max = max(fr, to);
        assert_ne!(min, max);
        if fenw.get_range_sum(min + 1..max) == 0 {
            out_line!("YES");
            return;
        }
        if fenw.get_range_sum(0..min) + fenw.get_range_sum(max + 1..total_max) == 0 {
            out_line!("YES");
            return;
        }
    }
    out_line!("NO");
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
}
//END MAIN
