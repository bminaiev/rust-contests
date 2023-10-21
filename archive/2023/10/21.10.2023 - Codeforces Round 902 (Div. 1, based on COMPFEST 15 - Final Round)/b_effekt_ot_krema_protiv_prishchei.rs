//{"name":"B. Эффект от крема против прыщей","group":"Codeforces - Codeforces Round 902 (Div. 1, based on COMPFEST 15 - Final Round)","url":"https://codeforces.com/contest/1876/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n19 14 19 9\n","output":"265\n"},{"input":"1\n0\n","output":"0\n"},{"input":"15\n90000 9000 99000 900 90900 9900 99900 90 90090 9090 99090 990 90990 9990 99990\n","output":"266012571\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BEffektOtKremaProtivPrishchei"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Elem {
    value: i32,
    pos: usize,
}

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let values = input.vec::<i32>(n);
    let mut a = gen_vec(n, |i| Elem {
        value: values[i],
        pos: i,
    });
    a.sort();
    let mut prev = vec![vec![]; n];
    for i in 0..n {
        for to in (i + 1..=n).step_by(i + 1) {
            let to = to - 1;
            prev[to].push(i);
        }
    }
    let mut seen = vec![false; n];
    let mut new_here = vec![0; n];
    for i in (0..n).rev() {
        let e = a[i];
        for p in prev[e.pos].iter().rev() {
            if !seen[*p] {
                seen[*p] = true;
                new_here[i] += 1;
            }
        }
    }
    let pow2 = Mod::gen_powers(Mod::TWO, n + 1);
    let mut tot_used = 0;
    let mut already_ways = Mod::ZERO;
    let mut res = Mod::ZERO;
    for i in 0..n {
        let e = a[i];
        tot_used += new_here[i];
        let ways = pow2[tot_used] - already_ways - Mod::ONE;
        res += ways * Mod::new(e.value);
        already_ways += ways;
    }
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
