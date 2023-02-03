//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn calc_changes(a: &[i32]) -> usize {
    let mut res = 0;
    for w in a.windows(2) {
        if w[0] != w[1] {
            res += 1;
        }
    }
    res
}

fn split_1d(n: i64) -> Option<Vec<i64>> {
    let total = n * (n + 1) / 2;
    if total % 2 == 1 {
        return None;
    }
    let mut need = total / 2 - n;
    let mut res = vec![];
    let mut left = n;
    while need > 0 && left > 0 {
        let mut cur_len = 1;
        loop {
            cur_len += 1;
            let created = cur_len * (cur_len + 1) / 2 - cur_len;
            if created > need {
                cur_len -= 1;
                break;
            }
        }
        need -= cur_len * (cur_len + 1) / 2 - cur_len;
        res.push(cur_len);
        left -= cur_len;
    }
    assert!(need == 0);
    assert!(left >= 0);
    for _ in 0..left {
        res.push(1);
    }
    Some(res)
}

fn stress() {
    for n in 1..1000_000 {
        dbg!(n);
        if let Some(res) = split_1d(n) {
            // dbg!(n, res);
        }
    }
}

fn stress2() {
    for n in 1..20 {
        let mut ok = false;
        let mut example = vec![];
        for mask in 0..1 << n {
            let mut a = vec![];
            for i in 0..n {
                if ((1 << i) & mask) != 0 {
                    a.push(1);
                } else {
                    a.push(0);
                }
            }
            let mut mixed = 0;
            let mut same = 0;
            for i in 0..n {
                for j in i + 1..=n {
                    let mut sub = a[i..j].to_owned();
                    sub.sort();
                    if sub[0] == *sub.last().unwrap() {
                        same += 1;
                    } else {
                        mixed += 1;
                    }
                }
            }
            if same == mixed {
                ok = true;
                if example.is_empty() || calc_changes(&example) > calc_changes(&a) {
                    example = a;
                }
            }
        }
        dbg!(n, ok, (n * (n + 1) / 2), example);
    }
}

fn print_vec(a: &[Vec<i32>]) {
    for aa in a.iter() {
        for &x in aa.iter() {
            out!(x, "");
        }
        out_line!();
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.i64();
        let m = input.i64();

        if let Some(by_h) = split_1d(n) {
            let mut res = vec![vec![0; m as usize]; n as usize];
            let mut offset = 0;
            let mut xor = 0;
            for &dh in by_h.iter() {
                let dh = dh as usize;
                for x in offset..offset + dh {
                    for y in 0..m as usize {
                        res[x][y] = xor;
                    }
                }
                xor ^= 1;
                offset += dh;
            }
            out_line!("Yes");
            print_vec(&res);
        } else if let Some(by_w) = split_1d(m) {
            let mut res = vec![vec![0; m as usize]; n as usize];
            let mut offset = 0;
            let mut xor = 0;
            for &dh in by_w.iter() {
                let dh = dh as usize;
                for x in offset..offset + dh {
                    for y in 0..n as usize {
                        res[y][x] = xor;
                    }
                }
                xor ^= 1;
                offset += dh;
            }
            out_line!("Yes");
            print_vec(&res);
        } else {
            out_line!("No");
        }
    }
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
