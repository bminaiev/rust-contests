//{"name":"C. Репрезентативные края","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"http://codeforces.com/contest/1616/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4\n1 2 3 4\n4\n1 1 2 2\n2\n0 -1\n6\n3 -2 4 -1 -4 0\n1\n-100\n","output":"0\n2\n0\n3\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CReprezentativnieKraya"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a: Vec<_> = (0..n)
        .map(|id| Point::new(id as i32, input.i32()))
        .collect();
    let mut res = n - 1;
    for i in 0..n {
        for j in i + 1..n {
            let mut ok_already = 0;
            for p in a.iter() {
                if Point::vect_mul(&a[i], &a[j], p) == 0 {
                    ok_already += 1;
                }
            }
            res.update_min(n - ok_already);
        }
    }
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
