//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::flows::hungarian_algorithm::hungarian_algorithm;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let b = input.vec::<usize>(n).sub_from_all(1);

    let offset = n as i64 + 10;
    let mut dist = Array2D::new(offset, m, m);
    for i in 0..n {
        dist[a[i]][b[i]] -= 1;
    }

    let best_cost = hungarian_algorithm(&dist).unwrap().min_cost;
    let mut mapping = vec![None; m];
    let mut rev_mapping = vec![None; m];

    let is_good = |mapping: &[Option<usize>], rev_mapping: &[Option<usize>]| {
        let mut new_dist = Array2D::new(offset, m, m);
        for i in 0..m {
            for j in 0..m {
                if (mapping[i].is_none() && rev_mapping[j].is_none()) || mapping[i] == Some(j) {
                    new_dist[i][j] = dist[i][j];
                }
            }
        }
        hungarian_algorithm(&new_dist).unwrap().min_cost == best_cost
    };

    for i in 0..n {
        let x = a[i];

        if mapping[x].is_some() {
            continue;
        }
        // dbg!(i, y, rev_mapping[y]);
        let mut found = false;
        for y in 0..m {
            if rev_mapping[y].is_some() {
                continue;
            }
            mapping[x] = Some(y);
            rev_mapping[y] = Some(x);
            if is_good(&mapping, &rev_mapping) {
                found = true;
                break;
            }
            mapping[x] = None;
            rev_mapping[y] = None;
        }
        assert!(found);
    }
    let mut free_y = vec![];
    for i in 0..m {
        if rev_mapping[i].is_none() {
            free_y.push(i);
        }
    }
    for i in 0..m {
        if mapping[i].is_none() {
            mapping[i] = free_y.pop();
        }
    }
    for &x in a.iter() {
        out!(mapping[x].unwrap() + 1, "");
    }
    out_line!();
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
