//{"name":"M. Monetary Reform","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/M/","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n10\n3\n5\n1\n","output":"INF\n17\n7\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MMonetaryReform"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let sz = a[0];
    let mut smallest_possible = vec![std::usize::MAX; sz];
    let mut sorted_positions = vec![];
    let mut seen_id = vec![n; sz];
    for (id, &new_val) in a.iter().enumerate() {
        let shift = new_val % sz;
        if smallest_possible[shift] > new_val {
            smallest_possible[shift] = new_val;
            let mut nqueue = vec![shift];
            let mut it1 = 0;
            let mut it2 = 0;
            while it1 < sorted_positions.len() || it2 < nqueue.len() {
                let use_first = if it1 == sorted_positions.len() {
                    false
                } else if it2 == nqueue.len() {
                    true
                } else {
                    smallest_possible[sorted_positions[it1]] < smallest_possible[nqueue[it2]]
                };
                let pos = if use_first {
                    it1 += 1;
                    sorted_positions[it1 - 1]
                } else {
                    it2 += 1;
                    nqueue[it2 - 1]
                };
                let mut npos = pos + shift;
                if npos >= sz {
                    npos -= sz;
                }
                let ncost = smallest_possible[pos] + new_val;
                if ncost < smallest_possible[npos] {
                    smallest_possible[npos] = ncost;
                    nqueue.push(npos);
                }
            }
            let mut merged = vec![];
            {
                it1 = 0;
                it2 = 0;
                while it1 < sorted_positions.len() || it2 < nqueue.len() {
                    if it1 < sorted_positions.len() && seen_id[sorted_positions[it1]] == id {
                        it1 += 1;
                        continue;
                    }
                    let use_first = if it1 == sorted_positions.len() {
                        false
                    } else if it2 == nqueue.len() {
                        true
                    } else {
                        smallest_possible[sorted_positions[it1]] < smallest_possible[nqueue[it2]]
                    };
                    let pos = if use_first {
                        it1 += 1;
                        sorted_positions[it1 - 1]
                    } else {
                        it2 += 1;
                        nqueue[it2 - 1]
                    };
                    merged.push(pos);
                    seen_id[pos] = id;
                }
                sorted_positions = merged;
            }
        }
        let max_bad = *smallest_possible.iter().max().unwrap();
        if max_bad == std::usize::MAX {
            out_line!("INF");
        } else {
            if max_bad <= sz {
                out_line!(-1);
            } else {
                out_line!(max_bad - sz);
            }
        }
    }
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
