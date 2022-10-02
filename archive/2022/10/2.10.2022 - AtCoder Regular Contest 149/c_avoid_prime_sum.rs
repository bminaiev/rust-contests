//{"name":"C - Avoid Prime Sum","group":"AtCoder - AtCoder Regular Contest 149","url":"https://atcoder.jp/contests/arc149/tasks/arc149_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n","output":"15 11 16 12\n13 3 6 9\n14 7 8 1\n4 2 10 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAvoidPrimeSum"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::iters::shifts_iter::ShiftsIterator;
use algo_lib::math::primes::gen_primes_table;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn gen(n: usize) -> Array2D<usize> {
    let primes = gen_primes_table(n * n * 2 + 100);
    let mut res = Array2D::new(0, n, n);
    let mut vals = vec![];
    for x in (1..=n * n).step_by(2) {
        vals.push(x);
    }
    for x in (2..=n * n).step_by(2) {
        vals.push(x);
    }
    for sum in 0..2 * n + 5 {
        for row in 0..n {
            if row <= sum {
                let col = sum - row;
                if col < n {
                    res[row][col] = vals.pop().unwrap();
                }
            }
        }
    }
    assert!(vals.is_empty());
    let si = ShiftsIterator::new(&SHIFTS_4, n, n);

    let mut cnt_bads = 0;

    let is_bad = |x: usize, y: usize, res: &Array2D<usize>| -> bool {
        for (nx, ny) in si.iter(x, y) {
            let sum = res[x][y] + res[nx][ny];
            if primes[sum] {
                return true;
            }
        }
        false
    };

    let mut rnd = Random::new(787788);
    let mut start_bad = vec![];
    for x in 0..n {
        for y in 0..n {
            if is_bad(x, y, &res) {
                start_bad.push((x, y));
            }
        }
    }

    loop {
        let mut bad = vec![];
        for &(x, y) in start_bad.iter() {
            if is_bad(x, y, &res) {
                bad.push((x, y));
            }
        }
        if bad.is_empty() {
            break;
        }
        if bad.len() <= 3 {
            for _it in 0..4 {
                let x = rnd.gen(0..n);
                let y = rnd.gen(0..n);
                if !bad.contains(&(x, y)) {
                    bad.push((x, y));
                    break;
                }
            }
        }

        let mut vals = vec![];
        for &(x, y) in bad.iter() {
            vals.push(res[x][y]);
        }
        rnd.shuffle(&mut vals);

        for i in 0..bad.len() {
            let (x, y) = bad[i];
            res[x][y] = vals[i];
        }

        start_bad = bad.clone();
    }

    for x in 0..n {
        for y in 0..n {
            if is_bad(x, y, &res) {
                cnt_bads += 1;
            }
        }
    }

    assert_eq!(cnt_bads, 0);

    res
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let res = gen(n);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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

fn stress() {
    for n in 3..1001 {
        let _ = gen(n);
        dbg!(n);
    }
}

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
