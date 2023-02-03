//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"a"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Interval {
    fr: i64,
    to: i64,
    prob: f64,
}

impl Interval {
    fn get_prob(&self, pos: i64) -> f64 {
        assert!(self.fr != self.to);
        self.prob * ((pos - self.fr) as f64 / (self.to - self.fr) as f64)
    }
}

fn update_intervals(a: &mut Vec<Interval>) {
    a.sort_by_key(|i| i.fr);
    let mut sum_a = 0;
    for i in a.iter() {
        sum_a += i.to - i.fr;
    }
    if sum_a == 0 {
        for i in 0..a.len() {
            a[i].prob = 1.0 / (a.len() as f64);
        }
    } else {
        for i in 0..a.len() {
            a[i].prob = (a[i].to - a[i].fr) as f64 / (sum_a as f64);
        }
    }
}

fn pick_number(a: &[Interval], rnd: &mut Random) -> f64 {
    let mut p = rnd.gen_double();
    for i in a.iter() {
        p -= i.prob;
        if p <= 0.0 {
            p += i.prob;
            p /= i.prob;
            return (i.fr as f64) * (1.0 - p) + (i.to as f64) * p;
        }
    }
    unreachable!();
}

fn solve_slow(a: &[Interval], b: &[Interval]) -> f64 {
    const ITERS: usize = 1000000;
    let mut rnd = Random::new(788778);
    let mut res = 0.0;
    for _ in 0..ITERS {
        let x = pick_number(a, &mut rnd);
        let y = pick_number(b, &mut rnd);
        res += (x - y).abs();
    }
    res /= ITERS as f64;
    res
}

fn gen_probs(a: &[Interval], all_coords: &[i64]) -> (Vec<f64>, Vec<bool>) {
    let mut res = vec![];
    let mut it = 0;
    let mut sum = 0.0;
    let mut prob_here = 0.0;
    let mut present = vec![];
    for i in 0..all_coords.len() {
        let x = all_coords[i];
        let mut is_present = false;
        while it != a.len() && a[it].to <= x {
            sum += a[it].prob - prob_here;
            if i + 1 != all_coords.len()
                && a[it].fr <= all_coords[i]
                && a[it].to >= all_coords[i + 1]
            {
                is_present = true;
            }
            prob_here = 0.0;
            it += 1;
        }
        if it != a.len() && a[it].to >= x && a[it].fr <= x {
            let more = a[it].get_prob(x) - prob_here;
            sum += more;
            prob_here += more;
            if i + 1 != all_coords.len()
                && a[it].fr <= all_coords[i]
                && a[it].to >= all_coords[i + 1]
            {
                is_present = true;
            }
        }
        res.push(sum);
        present.push(is_present);
    }

    (res, present)
}

fn solve_fast(a: &[Interval], b: &[Interval]) -> f64 {
    let mut all_coords = vec![];
    for i in a.iter() {
        all_coords.push(i.fr);
        all_coords.push(i.to);
    }
    for i in b.iter() {
        all_coords.push(i.fr);
        all_coords.push(i.to);
    }
    all_coords.sort();
    if all_coords[0] == *all_coords.last().unwrap() {
        return 0.0;
    }
    all_coords.dedup();
    // dbg!(all_coords);
    let mut res = 0.0;
    for &(left, right) in [(a, b)].iter() {
        let (p1, pr1) = gen_probs(&left, &all_coords);
        let (p2, pr2) = gen_probs(&right, &all_coords);

        // dbg!(p1, pr1);
        // dbg!(p2, pr2);

        for i in 0..all_coords.len() - 1 {
            let p1_left = p1[i];
            let mut p1_mid = p1[i + 1] - p1[i];
            let mut p1_right = 1.0 - p1_left - p1_mid;
            if !pr1[i] {
                p1_right += p1_mid;
                p1_mid = 0.0;
            }

            let p2_left = p2[i];
            let mut p2_mid = p2[i + 1] - p2[i];
            let mut p2_right = 1.0 - p2_left - p2_mid;
            if !pr2[i] {
                p2_right += p2_mid;
                p2_mid = 0.0;
            }

            // dbg!(
            //     all_coords[i],
            //     all_coords[i + 1],
            //     p1_left,
            //     p1_mid,
            //     p1_right,
            //     p2_left,
            //     p2_mid,
            //     p2_right
            // );

            let mut cur_coef = p1_left * p2_right + p1_right * p2_left;
            cur_coef +=
                0.5 * (p1_left * p2_mid + p2_left * p1_mid + p2_mid * p1_right + p2_right * p1_mid);
            cur_coef += p1_mid * p2_mid / 3.0;
            res += cur_coef * (all_coords[i + 1] - all_coords[i]) as f64;
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut a = gen_vec(n, |_| Interval {
        fr: input.read(),
        to: input.read(),
        prob: 0.0,
    });
    let mut b = gen_vec(m, |_| Interval {
        fr: input.read(),
        to: input.read(),
        prob: 0.0,
    });
    update_intervals(&mut a);
    update_intervals(&mut b);
    // dbg!(solve_slow(&a, &b));
    out_line!(solve_fast(&a, &b));
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
    // tester::run_single_test("3");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
