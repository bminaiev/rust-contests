//{"name":"D - Dance","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_d","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 0 1\n5 3\n2\n","output":"6\n"},{"input":"1\n5\n","output":"5\n"},{"input":"5\n900606388 317329110 665451442 1045743214 260775845 726039763 57365372 741277060 944347467\n369646735 642395945 599952146 86221147 523579390 591944369 911198494 695097136\n138172503 571268336 111747377 595746631 934427285 840101927 757856472\n655483844 580613112 445614713 607825444 252585196 725229185\n827291247 105489451 58628521 1032791417 152042357\n919691140 703307785 100772330 370415195\n666350287 691977663 987658020\n1039679956 218233643\n70938785\n","output":"1073289207\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDance"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize() * 2;
    let mut res = 0;
    let mut a = Array2D::new(0, n, n);
    for i in 0..n {
        for j in i + 1..n {
            let v = input.i32();
            a[i][j] = v;
            a[j][i] = v;
        }
    }
    let mut seen = vec![false; n];
    RecursiveFunction::new(|f, xor| {
        let first = (0..n).filter(|&id| !seen[id]).next();
        match first {
            None => {
                res.update_max(xor);
            }
            Some(first) => {
                for next in first + 1..n {
                    if !seen[next] {
                        seen[next] = true;
                        seen[first] = true;
                        f.call(xor ^ (a[first][next]));
                        seen[next] = false;
                        seen[first] = false;
                    }
                }
            }
        }
    })
    .call(0);
    out_line!(res);
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
