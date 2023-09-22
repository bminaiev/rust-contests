//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Seg {
    color: usize,
    len: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut segs = vec![];
    let k = input.usize();
    for color in 1..=k {
        let cnt = input.usize();
        for _ in 0..cnt {
            let len = input.i64();
            segs.push(Seg { color, len })
        }
    }
    segs.sort_by_key(|s| s.len);
    let mut best_three: Vec<Seg> = vec![];
    for &seg in segs.iter() {
        for s1 in best_three.iter() {
            for s2 in best_three.iter() {
                if s1.color != s2.color && s1.color != seg.color && s2.color != seg.color {
                    if s1.len + s2.len > seg.len {
                        out_line!(s1.color, s1.len, s2.color, s2.len, seg.color, seg.len);
                        return;
                    }
                }
            }
        }
        best_three.push(seg);
        best_three.sort_by_key(|s| -s.len);
        for i in 0..best_three.len() {
            for j in i + 1..best_three.len() {
                if best_three[i].color == best_three[j].color {
                    best_three.remove(j);
                    break;
                }
            }
        }
        best_three.truncate(3);
    }
    out_line!(-1);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
