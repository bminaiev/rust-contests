//{"name":"petr_10_a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"art.in","pattern":null},"output":{"type":"file","fileName":"art.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_a"}}}

use std::ops::Not;
use std::time::Instant;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_n(n: usize) -> u128 {
    let mut best_cnt = 0;
    let mut best_res = 0u128;
    let mut go = RecursiveFunction2::new(|f, mask_used: u128, mask_can: u128| {
        let potentially_max_cnt = mask_used.count_ones() + mask_can.count_ones();
        if potentially_max_cnt <= best_cnt {
            return;
        }
        if mask_can == 0 {
            best_cnt = mask_used.count_ones();
            best_res = mask_used;
            return;
        }
        let bit = mask_can.trailing_zeros();
        {
            let mut next_can = mask_can ^ (1 << bit);
            for prev in (0..bit).rev() {
                let dist = bit - prev;
                let remove = bit + dist * 2;
                if remove < n as u32 {
                    if ((1 << prev) & mask_used) != 0 {
                        next_can &= (1u128 << remove).not();
                    }
                } else {
                    break;
                }
            }
            f.call(mask_used | (1 << bit), next_can);
        }
        f.call(mask_used, mask_can ^ (1 << bit));
    });
    go.call(1, (1u128 << n) - 1);
    best_res
}

fn solve_n_prec(n: usize) -> u128 {
    [
        0,
        1,
        3,
        7,
        7,
        29,
        39,
        115,
        167,
        423,
        935,
        935,
        935,
        5031,
        5031,
        5031,
        54151,
        119687,
        119687,
        119687,
        853799,
        853799,
        2575879,
        6902311,
        10884391,
        30608019,
        44438823,
        44438823,
        245118983,
        444928423,
        980475933,
        980475933,
        3917824167,
        7143163047,
        15590491815,
        31374311719,
        42203414695,
        106409820583,
        250994492019,
        457163412135,
        656400515495,
        1813351039783,
        2701019579047,
        7269514479773,
        14535588055507,
        23332564631667,
        40928935150887,
        128507300549543,
        183663807501223,
        443799415296423,
        691457253970855,
        1872479308026791,
        4112414127686883,
        8224828255371687,
        14823077284221863,
        32899313021489575,
        65798625975866279,
        65798625975866279,
        158999562327954343,
        524111501271831463,
        1051682916796470183,
        1051682916796470183,
        2814192461545542567,
        8422224008946182595,
        12043809588147913639,
        30040382025316176807,
        56370435215130170279,
        56370435215130170279,
        191603187786385986471,
        472152918341481206695,
        472152918341481206695,
    ][n]
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let res = solve_n_prec(n);
    let cnt = res.count_ones();
    out_line!(cnt);
    for bit in 0..n {
        if ((1u128 << bit) & res) != 0 {
            out!(bit + 1, "");
        }
    }
    out_line!();
}

fn stress() {
    let mut prec = vec![0];
    for n in 1..=70 {
        let start = Instant::now();
        let res = solve_n(n);
        dbg!(n, res, res.count_ones(), start.elapsed().as_millis());
        prec.push(res);
    }
    dbg!(prec);
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
        input: TaskIoType::File("art.in".to_string()),
        output: TaskIoType::File("art.out".to_string()),
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
