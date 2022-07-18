//{"name":"A. Adobe Flash Games","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/A/","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5 3\n1 3 5\n1 1\n1 2\n1 3\n","output":"6\n"},{"input":"4 3 2\n3 3 2 1\n2 1 2\n2 2 3\n","output":"7\n"},{"input":"4 2 2\n1 2 2 1\n2 1 2\n2 3 4\n","output":"-1\n"},{"input":"2 10 0\n1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAdobeFlashGames"}}}

use std::collections::VecDeque;
use std::ops::Not;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let num_regions = input.usize();
    let colors = input.usize();
    let num_eq = input.usize();
    let need_colors = input.vec::<usize>(num_regions).sub_from_all(1);
    let eq_covers = gen_vec(num_eq, |_| {
        let cnt = input.usize();
        let mut mask = 0;
        for _ in 0..cnt {
            mask |= 1 << (input.usize() - 1)
        }
        mask
    });
    let mut dist = vec![std::i32::MAX; 1 << (num_regions + num_eq)];
    let mut init_mask = 0;
    for i in 0..num_regions {
        if need_colors[i] == 0 {
            init_mask |= 1 << i;
        }
    }
    let mut queue = VecDeque::new();
    queue.push_back(init_mask);
    dist[init_mask] = 0;
    while let Some(cur_mask) = queue.pop_front() {
        let cur_dist = dist[cur_mask];
        let mut cur_forb = 0;
        for i in 0..num_eq {
            if (1 << (i + num_regions)) & cur_mask != 0 {
                cur_forb |= eq_covers[i];
            }
        }
        for next_color in 0..colors {
            let mut next_mask = cur_mask;
            for i in 0..num_regions {
                if (1 << i) & cur_forb == 0 {
                    if need_colors[i] == next_color {
                        next_mask |= 1 << i;
                    } else {
                        next_mask &= (1usize << i).not();
                    }
                }
            }
            if dist[next_mask] > cur_dist + 1 {
                dist[next_mask] = cur_dist + 1;
                queue.push_back(next_mask);
            }
        }
        for i in 0..num_eq {
            let next_mask = cur_mask ^ (1 << (num_regions + i));
            if dist[next_mask] > cur_dist + 1 {
                dist[next_mask] = cur_dist + 1;
                queue.push_back(next_mask);
            }
        }
    }
    let need_mask = (1 << num_regions) - 1;
    if dist[need_mask] == std::i32::MAX {
        out_line!(-1);
    } else {
        out_line!(dist[need_mask]);
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
    // tester::run_stress(stress);
}
//END MAIN
