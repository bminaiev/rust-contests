//{"name":"A. Area and Perimeter","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/A/","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1/4\n3/8\n2/7\n1/2\n3/2\n","output":"YES\n1 1\n#\nYES\n3 4\n....\n.##.\n.#..\nNO\nYES\n3 3\n###\n#.#\n###\nYES\n8 8\n...####.\n.######.\n#######.\n#######.\n########\n.#######\n...#####\n....####\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAreaAndPerimeter"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_case(exp_s: usize, exp_p: usize, print: bool) -> bool {
    const MAX: usize = 100;
    for l1 in 1..MAX {
        for l2 in 1..MAX {
            let tot_p = (l1 + l2) * 2;
            let min_s = l1 + l2 - 1;
            let max_s = l1 * l2;
            for s in min_s..=max_s {
                if s * exp_p == exp_s * tot_p {
                    if print {
                        out_line!("YES");
                        out_line!(l1, l2);
                    }
                    let mut res = vec![vec![b'.'; l2]; l1];
                    const FILLED: u8 = b'#';
                    for i in 0..l1 {
                        res[i][0] = FILLED;
                    }
                    for j in 0..l2 {
                        res[0][j] = FILLED;
                    }
                    let mut more = s - min_s;
                    for i in 0..l1 {
                        for j in 0..l2 {
                            if more > 0 && res[i][j] != FILLED {
                                res[i][j] = FILLED;
                                more -= 1;
                            }
                        }
                    }
                    assert!(more == 0);
                    if print {
                        for i in 0..l1 {
                            out_line!(vec2str(&res[i]));
                        }
                    }
                    return true;
                }
            }
        }
    }
    if print {
        out_line!("NO");
    }
    return false;
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string_as_string();
    let parts: Vec<_> = s.split('/').collect();
    let exp_s: usize = parts[0].parse().unwrap();
    let exp_p: usize = parts[1].parse().unwrap();
    solve_case(exp_s, exp_p, true);
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

fn stress() {
    let mut sum = 0;
    for s in 1..=20 {
        for q in 1..=20 {
            if gcd(s, q) == 1 {
                if !solve_case(s, q, false) {
                    dbg!(s, q);
                    sum += 1;
                }
            }
        }
    }
    dbg!(sum);
}

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
