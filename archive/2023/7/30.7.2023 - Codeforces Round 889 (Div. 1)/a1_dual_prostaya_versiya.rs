//{"name":"A1. Dual (простая версия)","group":"Codeforces - Codeforces Round 889 (Div. 1)","url":"https://codeforces.com/contest/1854/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n2\n2 1\n4\n1 2 -10 3\n5\n2 1 1 1 1\n8\n0 0 0 0 0 0 0 0\n5\n1 2 -4 3 -10\n10\n11 12 13 14 15 -15 -16 -17 -18 -19\n7\n1 9 3 -4 -3 -2 -1\n3\n10 9 8\n20\n1 -14 2 -10 6 -5 10 -13 10 7 -14 19 -5 19 1 18 -16 -7 12 8\n20\n-15 -17 -13 8 14 -13 10 -4 11 -4 -16 -6 15 -4 -2 7 -9 5 -5 17\n","output":"1\n2 1\n3\n4 4\n4 4\n3 4\n4\n2 1\n3 1\n4 1\n5 1\n0\n7\n3 4\n3 4\n5 4\n5 4\n5 4\n5 4\n5 4\n15\n6 1\n6 1\n6 1\n7 2\n7 2\n7 2\n8 3\n8 3\n8 3\n9 4\n9 4\n9 4\n10 5\n10 5\n10 5\n8\n3 4\n3 4\n2 4\n2 4\n2 4\n2 4\n1 4\n1 4\n3\n2 1\n3 1\n3 1\n31\n14 1\n18 7\n13 11\n15 11\n6 4\n5 17\n19 6\n19 12\n10 5\n11 12\n1 17\n15 19\n16 10\n14 2\n16 11\n20 7\n7 6\n9 5\n3 6\n6 14\n17 18\n18 14\n12 3\n17 16\n8 18\n13 16\n9 8\n14 8\n16 2\n11 8\n12 7\n31\n5 12\n19 13\n9 1\n5 17\n18 19\n6 16\n15 8\n6 9\n15 14\n7 10\n19 7\n17 20\n14 4\n15 20\n4 3\n1 8\n16 12\n16 15\n5 6\n12 10\n11 15\n20 3\n20 19\n13 14\n11 14\n18 10\n7 3\n12 17\n4 7\n13 2\n11 13\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A1DualProstayaVersiya"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn max_pos(a: &[i32]) -> usize {
    let mut max = a[0];
    let mut max_pos = 0;
    for i in 1..a.len() {
        if a[i] > max {
            max = a[i];
            max_pos = i;
        }
    }
    max_pos
}

fn min_pos(a: &[i32]) -> usize {
    let mut min = a[0];
    let mut min_pos = 0;
    for i in 1..a.len() {
        if a[i] < min {
            min = a[i];
            min_pos = i;
        }
    }
    min_pos
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let mx = rnd.gen(1..20);
        let n = rnd.gen(1..20);
        let a = rnd.gen_vec(n, -mx..mx);
        dbg!(a);
        solve_case(a.clone());
    }
}

fn solve_case(mut a: Vec<i32>) -> Vec<(usize, usize)> {
    let mut res = vec![];
    let max = *a.iter().max().unwrap();
    let min = *a.iter().min().unwrap();
    if min == 0 && max == 0 {
        return res;
    }
    if max.abs() > min.abs() || (max.abs() == min.abs() && max > 0) {
        while a[0] <= 0 || max_pos(&a) != 0 {
            let max_pos = max_pos(&a);
            res.push((0, max_pos));
            a[0] += a[max_pos];
        }
        for i in 1..a.len() {
            while a[i] < a[i - 1] {
                let max_pos = max_pos(&a);
                res.push((i, max_pos));
                a[i] += a[max_pos];
            }
        }
    } else {
        let n = a.len();
        while a[n - 1] >= 0 || min_pos(&a) != n - 1 {
            let min_pos = min_pos(&a);
            res.push((n - 1, min_pos));
            a[n - 1] += a[min_pos];
        }
        for i in (0..n - 1).rev() {
            while a[i] > a[i + 1] {
                let min_pos = min_pos(&a);
                res.push((i, min_pos));
                a[i] += a[min_pos];
            }
        }
    }
    for w in a.windows(2) {
        assert!(w[0] <= w[1]);
    }
    dbg!("got", a);
    if res.len() > 31 {
        dbg!(res.len());
        dbg!(a);
        dbg!(res);
    }
    assert!(res.len() <= 31);
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i32>(n);
    let res = solve_case(a);
    out_line!(res.len());
    for &(x, y) in res.iter() {
        out_line!(x + 1, y + 1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
