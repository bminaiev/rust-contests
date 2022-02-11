//{"name":"C. Минимизируйте расстояние","group":"Codeforces - Технокубок 2022 - Отборочный Раунд 3","url":"http://codeforces.com/contest/1585/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n5 1\n1 2 3 4 5\n9 3\n-5 -10 -15 6 5 8 3 7 4\n5 3\n2 2 3 3 3\n4 2\n1000000000 1000000000 1000000000 1000000000\n","output":"25\n41\n7\n3000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMinimiziruiteRasstoyanie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{collections::last_exn::LastExn, misc::min_max::UpdateMinMax};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.read_vec::<i64>(n);
    let mut res = 0;
    let mut total_max = 0;
    for &sign in [-1, 1].iter() {
        let mut cur: Vec<_> = a
            .iter()
            .filter_map(|&x| {
                if x.signum() == sign {
                    Some(x.abs())
                } else {
                    None
                }
            })
            .collect();
        cur.sort();
        if !cur.is_empty() {
            total_max.update_max(*cur.last_exn());
        }
        while !cur.is_empty() {
            res += *cur.last_exn();
            for _ in 0..k {
                if !cur.is_empty() {
                    cur.pop();
                }
            }
        }
    }
    out_line!(res * 2 - total_max);
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
    // tester::run_single_test("1");
}
//END MAIN
