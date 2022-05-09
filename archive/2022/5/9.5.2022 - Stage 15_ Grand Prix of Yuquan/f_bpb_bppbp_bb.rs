//{"name":"F. BpbBppbpBB","group":"Yandex - Stage 15: Grand Prix of Yuquan","url":"https://official.contest.yandex.com/opencupXXII/contest/37831/problems/F/","interactive":false,"timeLimit":1000,"tests":[{"input":"10 17\n#################\n#################\n#################\n####..#####..####\n###....###....###\n###....###....###\n####..#####..####\n#################\n#################\n#################\n","output":"1 0\n"},{"input":"14 11\n.##########\n.##########\n.##########\n.####..####\n.###....###\n.###....###\n.####..####\n.##########\n.##########\n.##########\n.###.......\n.###.......\n.###.......\n.###.......\n","output":"0 1\n"},{"input":"20 14\n.##########...\n.##########...\n.##########...\n.####..####...\n.###....###...\n.###....###...\n.####..####...\n.##########...\n.##########...\n.##########...\n.#############\n.#############\n.#############\n.#######..####\n....###....###\n....###....###\n....####..####\n##############\n##############\n##############\n","output":"0 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBpbBppbpBB"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let s = gen_vec(n, |_| input.string());
    let mut expect = vec![];
    expect.push(String::from("######").into_bytes());
    expect.push(String::from("##..##").into_bytes());
    expect.push(String::from("#....#").into_bytes());
    expect.push(String::from("#....#").into_bytes());
    expect.push(String::from("##..##").into_bytes());
    expect.push(String::from("######").into_bytes());
    let mut is_center = Array2D::new(false, n, m);
    let mut tot_centers = 0;
    for i in 0..n {
        for j in 0..m {
            if i + expect.len() <= n && j + expect.len() <= m {
                let mut ok = true;
                for di in 0..expect.len() {
                    for dj in 0..expect.len() {
                        if expect[di][dj] != s[i + di][j + dj] {
                            ok = false;
                            break;
                        }
                    }
                }
                is_center[i][j] = ok;
                if ok {
                    tot_centers += 1;
                }
            }
        }
    }
    let mut bs = 0;
    for i in 0..n {
        for j in 0..m {
            if is_center[i][j] {
                if i + 7 < n && is_center[i + 7][j] {
                    bs += 1;
                }
                if j + 7 < m && is_center[i][j + 7] {
                    bs += 1;
                }
            }
        }
    }
    out_line!(bs, tot_centers - bs * 2);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    // tester::run_single_test("1");
}
//END MAIN
