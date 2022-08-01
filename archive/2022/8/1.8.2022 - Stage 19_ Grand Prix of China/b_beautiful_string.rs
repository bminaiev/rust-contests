//{"name":"B. Beautiful String","group":"Yandex - Stage 19: Grand Prix of China","url":"https://official.contest.yandex.com/opencupXXII/contest/39025/problems/B/","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n114514\n0000000\n","output":"1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBeautifulString"}}}

use std::cmp::min;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_smart(s: &[u8]) -> i64 {
    let n = s.len();
    let mut le = Array2D::new(0, n + 1, n + 1);
    for s1 in (0..n).rev() {
        for s2 in s1 + 1..n {
            if s[s1] == s[s2] {
                le[s1][s2] = 1 + le[s1 + 1][s2 + 1];
            } else {
                le[s1][s2] = 0;
            }
        }
    }
    let mut res = 0i64;
    for start in 0..n {
        let mut good_end_1 = vec![false; n + 1];
        for prev in 0..start {
            let len = start - prev;
            if le[prev][start] >= len && start + len <= n {
                good_end_1[start + len] = true;
            }
        }
        let mut end_suf = vec![0; n + 1];
        for next in start + 3..n {
            let len = min(le[start][next], next - start - 1);
            end_suf[start + len] += 1;
        }
        for i in (1..end_suf.len()).rev() {
            end_suf[i - 1] += end_suf[i];
        }
        let mut ways1 = 0;
        for end in start + 2..=n {
            if good_end_1[end - 1] {
                ways1 += 1;
            }
            let ways2 = end_suf[end];
            res += ways1 * ways2;
        }
    }
    res
}

fn stress() {
    for it in 25413.. {
        dbg!(it);
        let mut rnd = Random::new(787788 + it);
        let n = rnd.gen(1..32);
        let s = gen_vec(n, |_| rnd.gen(b'0'..b'8'));
        let smart = solve_smart(&s);
        let stupid = solve_stupid(&s);
        if smart != stupid {
            dbg!(vec2str(&s));
            dbg!(smart);
            dbg!(stupid);
            assert!(false);
        }
    }
}

fn solve_stupid(s: &[u8]) -> i64 {
    let mut res = 0;
    let n = s.len();
    for i0 in 0..n {
        for i1 in i0 + 1..n {
            let i2 = i1 - i0 + i1;
            for i3 in i2 + 1..n {
                for i4 in i3 + 1..n {
                    let i5 = i4 + i1 - i0;
                    let i6 = i5 + i3 - i2;
                    if i6 <= n {
                        if s[i0..i1] == s[i1..i2]
                            && s[i1..i2] == s[i4..i5]
                            && s[i2..i3] == s[i5..i6]
                        {
                            res += 1;
                        }
                    }
                }
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.string();
    // let res = solve_stupid(&s);
    let res = solve_smart(&s);
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

fn main() {
    tester::run_tests();
    // tester::run_stress(stress);
    // tester::run_single_test("2");
    // tester::run_stress(stress);
}
//END MAIN
