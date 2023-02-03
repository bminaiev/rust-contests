//{"name":"C. Равные частоты","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, основан на Отборочном раунде VK Cup 2022)","url":"https://codeforces.com/contest/1782/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\nhello\n10\ncodeforces\n5\neevee\n6\nappall\n","output":"1\nhelno\n2\ncodefofced\n1\neeeee\n0\nappall\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRavnieChastoti"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cnt {
    cnt: i32,
    pos: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let s = input.string();
    let mut cnt = (0..26).map(|pos| Cnt { cnt: 0, pos }).collect::<Vec<_>>();
    for &c in s.iter() {
        cnt[(c - b'a') as usize].cnt += 1;
    }
    cnt.sort();
    let mut res = n;
    let mut best_diff = 0;
    for diff in 1..=26 {
        if n % diff == 0 {
            let per_sym = n / diff;
            let mut cur_res = 0;
            for &c in cnt.iter() {
                if c.cnt > per_sym {
                    cur_res += c.cnt - per_sym;
                }
            }
            for i in 0..(26 - diff as usize) {
                cur_res += cnt[i].cnt;
            }
            if cur_res < res {
                res = cur_res;
                best_diff = diff;
            }
        }
    }
    assert_ne!(best_diff, 0);
    out_line!(res);
    {
        let mut t = s.clone();
        let mut delta = vec![0i32; 26];
        let per_sym = n / best_diff;
        for i in 0..(26 - best_diff as usize) {
            delta[cnt[i].pos] = -cnt[i].cnt;
        }
        for i in (26 - best_diff as usize)..26 {
            delta[cnt[i].pos] = per_sym - cnt[i].cnt;
        }
        for i in 0..n as usize {
            let c = (t[i] - b'a') as usize;
            if delta[c] < 0 {
                let mut found = false;
                for j in 0..26 {
                    if delta[j] > 0 {
                        delta[j] -= 1;
                        delta[c] += 1;
                        found = true;
                        t[i] = b'a' + j as u8;
                        break;
                    }
                }
                assert!(found);
            }
        }
        out_line!(vec2str(&t));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
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
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
