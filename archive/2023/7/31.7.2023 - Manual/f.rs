//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::fft::fft_multiply;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn calc(s: &[u8], ok_chars: &[u8]) -> Vec<i64> {
    let mut a = vec![0.0; s.len()];
    for i in 0..s.len() {
        if ok_chars.contains(&s[i]) {
            a[i] = 1.0;
        }
    }
    let res = fft_multiply(a.clone(), a.clone());
    let mut res = res.iter().map(|x| x.round() as i64).collect::<Vec<_>>();
    for i in 0..s.len() {
        if ok_chars.contains(&s[i]) {
            res[i + i] -= 1;
        }
    }
    for i in 0..res.len() {
        assert!(res[i] % 2 == 0);
        res[i] /= 2;
    }
    res
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(78778 + it);
        let n = rnd.next_in_range(1, 20);
        let mut s = vec![];
        for _ in 0..n {
            s.push([b'W', b'B', b'?'][rnd.gen(0..3)]);
        }
        let cnt_q = s.iter().filter(|&&x| x == b'?').count() as i64;
        let max_w = rnd.gen(0..10);
        let max_b = rnd.gen(0..10);
        if cnt_q > max_w + max_b {
            continue;
        }
        let slow = solve_slow(&s, max_w, max_b);
        let fast = solve_fast(&s, max_w, max_b);
        if fast != slow {
            let ss = s.iter().map(|&x| x as char).collect::<String>();
            dbg!(ss, max_w, max_b);
        }
        assert_eq!(slow, fast);
        dbg!(fast, slow);
    }
}

fn calc_slow(s: &[u8]) -> i64 {
    let mut res = vec![0; s.len() * 2 + 1];
    for i in 0..s.len() {
        for j in i + 1..s.len() {
            if s[i] != s[j] {
                res[i + j] += 1;
            }
        }
    }
    res.iter().copied().max().unwrap()
}

fn solve_slow(s: &[u8], max_w: i64, max_b: i64) -> i64 {
    let mut res = 0;
    for mask in 0..1 << s.len() {
        let mut check_s = vec![];
        let mut used_w = 0;
        let mut used_b = 0;
        for i in 0..s.len() {
            if s[i] == b'?' {
                if (1 << i) & mask != 0 {
                    check_s.push(b'W');
                    used_w += 1;
                } else {
                    check_s.push(b'B');
                    used_b += 1;
                }
            } else {
                check_s.push(s[i]);
            }
        }
        if used_b <= max_b && used_w <= max_w {
            res.update_max(calc_slow(&check_s));
        }
    }
    res
}

fn solve_fast(s: &[u8], max_w: i64, max_b: i64) -> i64 {
    let same_w = calc(s, &[b'W']);
    let same_b = calc(s, &[b'B']);
    let w_or_q = calc(s, &[b'W', b'?']);
    let b_or_q = calc(s, &[b'B', b'?']);
    let same_q = calc(s, &[b'?']);
    let mut res = 0;
    for i in 0..same_w.len() {
        let w = same_w[i];
        let b = same_b[i];
        let q = same_q[i];
        let w_or_q = w_or_q[i] - w - q;
        let b_or_q = b_or_q[i] - b - q;
        assert!(w_or_q >= 0);
        assert!(b_or_q >= 0);

        let mut total_pairs = if i < s.len() {
            (i + 1) as i64
        } else {
            let from = i - s.len() + 1;
            let to = s.len();
            if to < from {
                0
            } else {
                (to - from) as i64
            }
        };
        total_pairs /= 2;

        let wb = total_pairs - w - b - w_or_q - b_or_q - q;
        assert!(wb >= 0);

        let mut cur_res = wb;
        let mut more_w = max_w;
        let mut more_b = max_b;
        {
            let use_w = min(more_w, b_or_q);
            more_w -= use_w;
            cur_res += use_w;
        }
        {
            let use_b = min(more_b, w_or_q);
            more_b -= use_b;
            cur_res += use_b;
        }
        {
            let use_more = min(more_b, min(more_w, q));
            cur_res += use_more;
        }
        res.update_max(cur_res);
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.usize();
    let max_w = input.i64();
    let max_b = input.i64();
    let s = input.string();
    let res = solve_fast(&s, max_w, max_b);
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
