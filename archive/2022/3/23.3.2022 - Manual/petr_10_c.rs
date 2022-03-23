//{"name":"petr_10_c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"file","fileName":"curiosity.in","pattern":null},"output":{"type":"file","fileName":"curiosity.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_c"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::{output, set_global_output_to_file, set_global_output_to_stdout};
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::{shift_by_uldr, SHIFTS_4};
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::{SearchFor, SimulatedAnnealing};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

const UNKNOWN: u8 = b'?';

fn calc_score(matrix: &Array2D<i32>, s: &[u8]) -> usize {
    let n = 6;
    let mut dp = Array2D::new(true, n, n);
    let mut ndp = Array2D::new(false, n, n);
    for idx in (0..s.len() - 1).step_by(2) {
        let color = s[idx];
        if color != UNKNOWN {
            let need_color = (color - b'0') as i32;
            for i in 0..n {
                for j in 0..n {
                    if matrix[i][j] != need_color {
                        dp[i][j] = false;
                    }
                }
            }
        }
        for i in 0..n {
            for j in 0..n {
                ndp[i][j] = false;
            }
        }

        let mut any = false;
        for x in 0..n {
            for y in 0..n {
                if dp[x][y] {
                    any = true;
                    let p = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    let expect_shift = if s[idx + 1] == UNKNOWN {
                        None
                    } else {
                        Some(shift_by_uldr(s[idx + 1]))
                    };
                    for shift in SHIFTS_4.iter() {
                        if let Some(expected_shift) = expect_shift {
                            if expected_shift != *shift {
                                continue;
                            }
                        }
                        let np = p.apply_shift(shift);
                        if np.index_arr2d(matrix).is_some() {
                            ndp[np.x as usize][np.y as usize] = true;
                        }
                    }
                }
            }
        }
        if !any {
            return idx;
        }

        {
            let tmp = dp;
            dp = ndp;
            ndp = tmp;
        }
    }
    s.len()
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize() - 1;
    let results = vec![
        "010111000001010011101110001110011110",
        "100101000110000000001111010110111000",
        "100000011001010100110100001001010101",
        "011111101000001010000010100101001000",
        "110011101100011010010100100100000100",
        "100111001101101001100110101110010101",
        "010011111110001110110000100110111101",
        "111101000001001011000001010011110011",
        "000000100101011011001100000001111101",
        "111100010100010111111101101100000100",
    ];
    let sz = 6;
    let need_str = (&results[n]).as_bytes();
    for x in 0..sz {
        for y in 0..sz {
            out!(need_str[x * sz + y] - b'0');
        }
        out_line!();
    }
}

fn solve2(input: &mut Input, _test_case: usize) {
    let s = input.string();
    let mut rnd = Random::new(782788);
    for iter in 0.. {
        dbg!(_test_case, iter);
        let n = 6;
        let mut a = Array2D::new(0, n, n);
        for i in 0..n {
            for j in 0..n {
                a[i][j] = rnd.gen_in_range(0..2i32);
            }
        }
        let mut sa = SimulatedAnnealing::new(0.1, SearchFor::MaximumScore, 100.0, 1.0);
        let mut prev_score = calc_score(&a, &s);
        while sa.should_continue() {
            let x = rnd.gen_in_range(0..n);
            let y = rnd.gen_in_range(0..n);
            a[x][y] ^= 1;
            let new_score = calc_score(&a, &s);
            if sa.should_go(prev_score, new_score) {
                prev_score = new_score;
            } else {
                a[x][y] ^= 1;
            }
        }
        if prev_score >= s.len() {
            //s.len() {
            out!("\"");
            for x in 0..n {
                for y in 0..n {
                    out!(a[x][y]);
                }
            }
            out_line!("\",");
            break;
        }
    }
    // dbg!(s);
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
        input: TaskIoType::File("curiosity.in".to_string()),
        output: TaskIoType::File("curiosity.out".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // set_global_output_to_file("petr_10_c/res.txt");
    // for test in 1..=10 {
    //     let mut input = Input::new_file(format!("petr_10_c/tests/{}.lab", test));
    //     solve(&mut input, test);
    // }
    // output().flush();
    tester::run_locally();
}
//END MAIN
