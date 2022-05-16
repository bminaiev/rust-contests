//{"name":"I, O Bot","group":"Google Coding Competitions - Round 2 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008778ec/0000000000b15167","interactive":false,"timeLimit":40000,"tests":[{"input":"4\n5 0\n3 0\n6 0\n8 0\n10 1\n15 1\n5 10\n3 0\n6 0\n8 0\n10 1\n15 1\n5 1\n3 0\n6 0\n8 0\n10 1\n15 1\n2 0\n1000000000 0\n-1000000000 1\n","output":"Case #1: 52\nCase #2: 56\nCase #3: 54\nCase #4: 4000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IOBot"}}}

use std::cmp::{max, min};
use std::time::Instant;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
use algo_lib::misc::sse_utils::print_available_sse_extensions;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Elem {
    pos: i64,
    value: i32,
}

#[inline(never)]
#[target_feature(enable = "avx2")]
unsafe fn res_to_optimize1(
    ones: &[i64],
    dp0: &mut [i64],
    dp1: &[i64],
    dp2: &[i64],
    prev_z2: i64,
    convert: i64,
) {
    let len = ones.len() + 1;

    let ones = &ones[..len - 1];
    let dp0 = &mut dp0[..len];
    let dp1 = &dp1[..len];
    let dp2 = &dp2[..len];

    for o in 2..len {
        let prev_o2 = ones[o - 1] * 2;
        let f1 = dp1[o] + prev_z2;
        let f2 = dp2[o] + prev_z2 + convert;
        let f3 = dp1[o - 1] + max(prev_z2, prev_o2);
        dp0[o] = min(f1, min(f2, f3));
    }
}

#[inline(never)]
#[target_feature(enable = "avx2")]
unsafe fn res_to_optimize2(ones: &[i64], dp0: &mut [i64], convert: i64) {
    let len = dp0.len();
    let ones = &ones[..len - 1];
    for o in 2..len {
        let prev_o2 = ones[o - 1] * 2;
        let f1 = dp0[o];
        let f2 = dp0[o - 1] + prev_o2;
        let f3 = dp0[o - 2] + prev_o2 + convert;
        dp0[o] = min(f1, min(f2, f3));
    }
}

fn res(a: &mut Vec<Elem>, convert: i64) -> i64 {
    a.sort();
    let mut zeros = vec![];
    let mut ones = vec![];
    for e in a.iter() {
        if e.value == 0 {
            zeros.push(e.pos);
        } else {
            assert_eq!(e.value, 1);
            ones.push(e.pos);
        }
    }

    const MAX: i64 = std::i64::MAX / 5;

    let ones_size = ones.len() + 1;

    let mut dp2 = vec![MAX; ones_size];
    let mut dp1 = vec![MAX; ones_size];
    let mut dp0 = vec![MAX; ones_size];
    dp0[0] = 0;
    for z in 0..=zeros.len() {
        let prev_z2 = if z == 0 { MAX } else { zeros[z - 1] * 2 };
        for o in 0..std::cmp::min(2, ones.len() + 1) {
            let prev_o2 = if o == 0 { MAX } else { ones[o - 1] * 2 };
            let mut cur = dp0[o];
            cur.update_min(dp1[o] + prev_z2);
            cur.update_min(dp2[o] + prev_z2 + convert);
            if o >= 1 {
                cur.update_min(dp1[o - 1] + max(prev_z2, prev_o2));
                cur.update_min(dp0[o - 1] + prev_o2);
            }
            if o >= 2 {
                cur.update_min(dp0[o - 2] + prev_o2 + convert);
            }
            dp0[o] = cur;
        }
        unsafe {
            res_to_optimize1(&ones, &mut dp0, &dp1, &dp2, prev_z2, convert);
            res_to_optimize2(&ones, &mut dp0, convert);
        }
        let tmp = dp2;
        dp2 = dp1;
        dp1 = dp0;
        dp0 = tmp;
        for x in dp0.iter_mut() {
            *x = MAX;
        }
    }
    dp1[ones.len()]
}

fn solve(input: &mut Input, test_case: usize, start: &Instant) {
    let n = input.usize();
    let convert = input.i64();

    let all = gen_vec(n, |_| Elem {
        pos: input.read(),
        value: input.read(),
    });

    let mut positive = vec![];
    let mut negative = vec![];
    for e in all.into_iter() {
        if e.pos > 0 {
            positive.push(e.clone());
        } else {
            negative.push(Elem {
                pos: e.pos * -1,
                value: e.value,
            })
        }
    }

    out_line!(format!(
        "Case #{}: {}",
        test_case,
        res(&mut positive, convert) + res(&mut negative, convert)
    ));
    dbg!(test_case, start.elapsed());
}

fn stress() {
    print_available_sse_extensions();
    let mut rnd = Random::new(787788);
    let start = Instant::now();
    let mut elems = gen_vec(100_000, |_| Elem {
        pos: rnd.gen(0..1_000_000_000),
        value: rnd.gen(0..2),
    });
    let result = res(&mut elems, 123456);
    out_line!(result);
    out_line!(format!("Elapsed: {}ms", start.elapsed().as_millis()));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    let start = Instant::now();
    for i in 0usize..t {
        solve(&mut input, i + 1, &start);
    }
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
    // tester::run_tests();
    // tester::run_single_test("2");
    tester::run_locally();
}
//END MAIN
