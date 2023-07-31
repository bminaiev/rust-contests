//{"name":"g","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"g"}}}

use std::cmp::min;
use std::time::Instant;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn stress() {
    let n = 50_000;
    let mut rnd = Random::new(787788);
    let mut objects = vec![];
    for _ in 0..1_000_000 {
        objects.push((rnd.gen(0..301), rnd.gen(0..1_000_000_000)));
    }
    let res = solve_fast(objects, n);
}

#[target_feature(enable = "avx2")]
pub unsafe fn max_sum(aa: &[i64], bb: &[i64]) -> i64 {
    aa.iter()
        .zip(bb.iter())
        .map(|(&a, &b)| a + b)
        .max()
        .unwrap_or(0)
}

fn solve_fast(objects: Vec<(usize, i64)>, n: usize) -> Vec<i64> {
    let mut sum = vec![0; n + 1];
    const MAX_PRICE: usize = 300;
    let mut by_price = vec![vec![]; MAX_PRICE + 1];
    let mut base_res = 0;
    for (price, val) in objects {
        if price == 0 {
            base_res += val;
            continue;
        }
        by_price[price].push(val);
    }
    for price in 1..=MAX_PRICE {
        let cur_price = &mut by_price[price];
        cur_price.sort();
        cur_price.reverse();
        for i in 1..cur_price.len() {
            cur_price[i] += cur_price[i - 1];
        }
        cur_price.reverse();
        let mut aux = vec![];
        for rem in 0..price {
            aux.clear();
            for i in (rem..sum.len()).step_by(price) {
                aux.push(sum[i]);
            }
            for pos in (0..aux.len()).rev() {
                let cnt = min(pos, cur_price.len());
                let mut best = aux[pos];
                let aa = &aux[pos - cnt..pos];
                let bb = &cur_price[cur_price.len() - cnt..];
                best.update_max(unsafe { max_sum(aa, bb) });
                aux[pos] = best;
            }
            for i in (rem..sum.len()).step_by(price) {
                sum[i] = aux[i / price];
            }
        }
    }
    for i in 0..=n {
        sum[i] += base_res;
    }
    sum[1..].to_vec()
}

fn solve(input: &mut Input, _test_case: usize) {
    // let start = Instant::now();
    // stress();
    // if true {
    //     out_line!(format!("{:?}", start.elapsed()));
    //     return;
    // }
    let n_types = input.usize();
    let n = input.usize();
    let objects = (0..n_types)
        .map(|_| (input.usize(), input.i64()))
        .collect::<Vec<_>>();
    let res = solve_fast(objects, n);
    out_line!(res);
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
    // tester::run_tests();
    tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
