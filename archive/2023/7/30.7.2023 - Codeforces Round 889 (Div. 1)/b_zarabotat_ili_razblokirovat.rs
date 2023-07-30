//{"name":"B. Заработать или разблокировать","group":"Codeforces - Codeforces Round 889 (Div. 1)","url":"https://codeforces.com/contest/1854/problem/B","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n1 2\n","output":"2\n"},{"input":"5\n2 4 5 0 1\n","output":"9\n"},{"input":"4\n0 4 4 4\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BZarabotatIliRazblokirovat"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let mut cur = BitSet::new(2 * n + 5);
    cur.set(0, true);
    let mut pref_sum = 0;
    let mut res = 0;
    for pos in 0..=n {
        if pos > 0 {
            pref_sum += a[pos - 1] as i64;
            if cur.get(pos - 1) {
                cur.set(pos - 1, false);
                res.update_max(pref_sum - pos as i64 + 1);
            }
        }
        if pos == n {
            break;
        }
        if a[pos] != 0 {
            let next = cur.shift_right(a[pos]);
            cur |= &next;
        }
    }
    for pos in n + 1..2 * n + 4 {
        if cur.get(pos - 1) {
            res.update_max(pref_sum - pos as i64 + 1);
        }
    }
    out_line!(res);
}

#[test]
fn test() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(787788 + it);
        let n = 1000;
        let mut a = BitSet::new(n);
        let mut b = vec![false; n];
        for i in 0..n {
            if rnd.gen_bool() {
                a.set(i, true);
                b[i] = true;
            }
        }
        let shift = rnd.gen(0..n);
        let a = a.shift_right(shift);
        let mut c = vec![false; n];
        for i in 0..n {
            if b[i] && i + shift < n {
                c[i + shift] = true;
            }
        }
        for i in 0..n {
            assert_eq!(c[i], a.get(i));
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
