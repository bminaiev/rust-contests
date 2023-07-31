//{"name":"d","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"d"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn need_numbers(n: usize) -> usize {
    ((n as f64).sqrt() / 2.0).ceil() as usize
}

fn find_sol(max_n: usize, need: usize, rnd: &mut Random) -> Option<(Vec<usize>, Vec<usize>)> {
    if need == 1 {
        return Some((vec![1], vec![0]));
    }
    let shift = max_n / (need + 1);

    let mut a = vec![0; need];
    for i in 0..a.len() {
        a[i] = 1 + i * shift;
    }
    let mut delta = vec![0; need];
    let mut seen = vec![false; max_n * 2 + 1];
    for i in 0..need {
        let mut tries = 0;
        loop {
            tries += 1;
            let mut ok = true;
            delta[i] = rnd.gen(0..shift);
            for j in 0..i {
                let sum = a[i] + a[j] + delta[i] + delta[j];
                if seen[sum] {
                    ok = false;
                }
            }
            if ok {
                for j in 0..i {
                    let sum = a[i] + a[j] + delta[i] + delta[j];
                    seen[sum] = true;
                }
                break;
            }
            // if tries % 10 == 0 {
            // dbg!(i, tries);
            // }
            if tries > 100 {
                return None;
            }
        }
    }
    for i in 0..a.len() {
        a[i] += delta[i];
    }
    for &x in a.iter() {
        assert!(x <= max_n);
    }
    Some((a, delta))
}

fn is_good_sol(a: &[usize]) -> bool {
    let mut all = vec![];
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            all.push(a[i] + a[j]);
        }
    }
    all.sort();
    for w in all.windows(2) {
        if w[0] == w[1] {
            return false;
        }
    }
    true
}

fn stress() {
    for n in 1..1_000_000 {
        if need_numbers(n) > need_numbers(n - 1) {
            let need = need_numbers(n);
            dbg!(n, need);
            let mut rnd = Random::new(78788);
            for it in 1.. {
                if let Some((sol, delta)) = find_sol(n, need, &mut rnd) {
                    if is_good_sol(&sol) {
                        dbg!(it);
                        dbg!(delta);
                        break;
                    } else {
                        panic!("bad sol");
                    }
                }
            }
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {}

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
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
