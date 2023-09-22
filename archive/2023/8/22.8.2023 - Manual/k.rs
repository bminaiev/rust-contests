//{"name":"k","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"k"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn floyd_warshall(dist: &mut [Vec<i32>]) {
    let n = dist.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                dist[j][k] = min(dist[j][k], dist[j][i] + dist[i][k]);
            }
        }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn calc_delta(pos_of_next: &[usize], fr: usize, to: usize, change_mult: &[i32]) -> i32 {
    let mut res = 0;
    for i in fr..to {
        let inside = pos_of_next[i] >= fr && pos_of_next[i] < to;
        res += (inside as i32) * change_mult[i];
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let p = input.vec::<usize>(n).sub_from_all(1);
    let mut position = vec![0; n];
    for i in 0..n {
        position[p[i]] = i;
    }
    let change_mult: Vec<i32> = (0..n)
        .map(|i| {
            if p[i] == n - 1 {
                0
            } else if position[p[i]] < position[p[i] + 1] {
                1
            } else {
                -1
            }
        })
        .collect();
    let pos_of_next: Vec<usize> = (0..n)
        .map(|i| if p[i] == n - 1 { n } else { position[p[i] + 1] })
        .collect();
    let base_rounds = change_mult.iter().filter(|&&change| change == -1).count();
    out_line!(base_rounds * n + position[n - 1] + 1);
    output().flush();
    for _ in 0..k {
        let (fr, to) = (input.usize() - 1, input.usize());
        let nth_pos = if position[n - 1] >= fr && position[n - 1] < to {
            to - (position[n - 1] - fr) - 1
        } else {
            position[n - 1]
        };
        let delta = unsafe { calc_delta(&pos_of_next, fr, to, &change_mult) };
        let rounds = ((base_rounds as i32) + delta) as usize;
        out_line!(rounds * n + nth_pos + 1);
        output().flush();
    }
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

fn stress() {
    let n = 50_000;
    let mut rnd = Random::new(787788);
    let p = rnd.gen_permutation(n);
    let mut position = vec![0; n];
    for i in 0..n {
        position[p[i]] = i;
    }
    let change_mult: Vec<i32> = (0..n)
        .map(|i| {
            if p[i] == n - 1 {
                0
            } else if position[p[i]] < position[p[i] + 1] {
                1
            } else {
                -1
            }
        })
        .collect();
    let pos_of_next: Vec<usize> = (0..n)
        .map(|i| if p[i] == n - 1 { n } else { position[p[i] + 1] })
        .collect();
    let mut xor = 0;
    for _ in 0..n {
        let fr = rnd.gen(0..64);
        let to = n - rnd.gen(1..64);
        let delta = unsafe { calc_delta(&pos_of_next, fr, to, &change_mult) };
        xor ^= delta;
    }
    dbg!(xor);
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
