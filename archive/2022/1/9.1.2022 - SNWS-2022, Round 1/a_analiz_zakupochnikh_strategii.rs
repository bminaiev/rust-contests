//{"name":"A. Анализ закупочных стратегий","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/?nc=7hvEsgcY","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n9 3\n-2 -1 0 1 3 5 7 9 11\n1 3\n3 4\n3 3\n9 9\n3 9\n3 9\n","output":"0\n5.5\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAnalizZakupochnikhStrategii"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let a: Vec<i64> = input.read_vec(n);
    for _ in 0..q {
        let l1 = input.usize() - 1;
        let r1 = input.usize() - 1;
        let l2 = input.usize() - 1;
        let r2 = input.usize() - 1;
        let total_cnt = (r1 - l1 + 1) + (r2 - l2 + 1);
        fn sum(l: usize, r: usize, x: usize) -> usize {
            if x < l {
                0
            } else if x >= r {
                r - l + 1
            } else {
                x - l + 1
            }
        };
        let cnt_less_or_eq = |x: usize| sum(l1, r1, x) + sum(l2, r2, x);
        if total_cnt % 2 == 1 {
            let res =
                a[binary_search_first_true(0..n, |pos| cnt_less_or_eq(pos) >= (total_cnt + 1) / 2)];
            out_line!(res);
        } else {
            let res1 =
                a[binary_search_first_true(0..n, |pos| cnt_less_or_eq(pos) >= (total_cnt) / 2)];
            let res2 =
                a[binary_search_first_true(0..n, |pos| cnt_less_or_eq(pos) >= (total_cnt) / 2 + 1)];
            let sum = res1 + res2;
            if sum % 2 == 0 {
                out_line!(sum / 2);
            } else {
                out_line!(format!("{}.5", sum / 2));
            }
        }
    }
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
