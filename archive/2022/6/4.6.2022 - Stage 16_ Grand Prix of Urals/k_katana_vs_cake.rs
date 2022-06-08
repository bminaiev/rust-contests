//{"name":"K. Katana vs. Cake","group":"Yandex - Stage 16: Grand Prix of Urals","url":"https://official.contest.yandex.com/opencupXXII/contest/38278/problems/K/","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n1.0 2.0 1.0 0.7\n-0.6 0.0 1.0 0.2\n1.0 -3.0 1.0 0.5\n","output":"1.05139753126103951963\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KKatanaVsCake"}}}

use std::collections::VecDeque;
use std::f32::consts::PI;
use std::ops::Range;
use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type float_type = f32;

struct Plane([float_type; 4]);

#[target_feature(enable = "sse2")]
unsafe fn solve_case3(a: &[Plane]) -> float_type {
    let n = a.len();
    let mut cnt = vec![0.0; 1 << (n + 1)];
    let mut sum_cnt = 0.0;

    const SZ: usize = 247;
    const STEP: float_type = 2.0 / (SZ as float_type + 1.0);
    const START: float_type = -1.0;

    let mut rnd = Random::new(787790);
    let mut gen = || rnd.gen_double() as float_type * STEP;

    const NUM_RANDOMS: usize = 1 << 10;
    let rnds = gen_vec(NUM_RANDOMS, |_| gen());

    let mut it1 = 0;
    let mut it2 = 1;
    let mut it3 = 2;

    let powers = gen_vec(a.len() + 1, |i| (1 << i) as float_type);

    for i in 0..SZ {
        for k in 0..SZ {
            for j in 0..SZ {
                const S: usize = 4;

                let x_start = START + (i as float_type) * STEP;
                let y_start = START + (j as float_type) * STEP;
                let z_start = START + (k as float_type) * STEP;

                let mut xx = [0.0; S];
                let mut yy = [0.0; S];
                let mut zz = [0.0; S];
                for q in 0..S {
                    xx[q] = x_start + rnds.get_unchecked((it1 + q) & (NUM_RANDOMS - 1));
                    yy[q] = y_start + rnds.get_unchecked((it2 + q * 2) & (NUM_RANDOMS - 1));
                    zz[q] = z_start + rnds.get_unchecked((it3 + q * 3) & (NUM_RANDOMS - 1));
                }

                it1 += 1 * S;
                it2 += 2 * S;
                it3 += 3 * S;

                let mut masks = [0.0; S];
                for (((mask, x), y), z) in masks
                    .iter_mut()
                    .zip(xx.iter())
                    .zip(yy.iter())
                    .zip(zz.iter())
                {
                    *mask += (x * x + y * y + z * z > 1.0) as usize as f32 * powers[n];
                    for (pos, p) in a.iter().enumerate() {
                        *mask += (x * p.0[0] + y * p.0[1] + z * p.0[2] + p.0[3] > 0.0) as usize
                            as f32
                            * powers[pos];
                    }
                }
                for &m in masks.iter() {
                    unsafe {
                        *cnt.get_unchecked_mut(m as usize) += 1.0;
                    }
                }
            }
        }
    }

    // dbg!(checks);
    let mut max_id = 0;
    for i in 0..(1 << n) {
        sum_cnt += cnt[i];
        if cnt[i] > cnt[max_id] {
            max_id = i;
        }
    }
    let res = (cnt[max_id]) / (sum_cnt);
    let full = res * 4.0 / 3.0 * PI;
    full
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.matrix::<float_type>(n, 4);
    let a = gen_vec(n, |id| Plane([a[id][0], a[id][1], a[id][2], a[id][3]]));
    let res = unsafe { solve_case3(&a) };
    out_line!(res);
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
