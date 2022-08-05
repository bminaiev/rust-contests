//{"name":"Schrödinger and Pavlov","group":"Google Coding Competitions - World Finals 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/000000000087762e/0000000000b9c73a","interactive":false,"timeLimit":10000,"tests":[{"input":"4\n4\n??.C\n2 3 1 3\n4\n????\n2 3 1 3\n6\n?.????\n6 6 6 6 6 5\n34\n????????????????????????????????CC\n2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 33\n","output":"Case #1: 1\nCase #2: 2\nCase #3: 15\nCase #4: 294967268\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SchrdingerAndPavlov"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    const EMPTY: u8 = b'.';
    const CAT: u8 = b'C';
    const UNKNOWN: u8 = b'?';
    let mut s = input.string();
    for _ in 0..10 {
        s.push(EMPTY);
    }
    let jump = input.vec::<usize>(n).sub_from_all(1);
    const C: usize = 5;
    let m = C * 2 + 1;
    let mut ways = vec![Mod::ZERO; 1 << m];
    for mask in 0..(1 << (C + 1)) {
        let mut ok = true;
        for i in 0..=C {
            let c = s[i];
            if ((1 << i) & mask) != 0 && c == EMPTY {
                ok = false;
            }
            if ((1 << i) & mask) == 0 && c == CAT {
                ok = false;
            }
        }
        if ok {
            ways[mask << C] += Mod::ONE;
        }
    }
    for pos in 0..n {
        let mut nways = vec![Mod::ZERO; 1 << m];
        for mask in 0..(1 << m) {
            let mut nmask = mask;
            if ((1 << C) & mask) != 0 {
                let to = jump[pos];
                let rel_to = to + C - pos;
                if ((1 << rel_to) & mask) == 0 {
                    nmask ^= 1 << rel_to;
                    nmask ^= 1 << C;
                }
            }
            for exist in 0..2 {
                if exist == 0 && s[pos + C + 1] == CAT {
                    continue;
                }
                if exist == 1 && s[pos + C + 1] == EMPTY {
                    continue;
                }
                let real_nmask = (nmask >> 1) | (exist << (2 * C));
                nways[real_nmask] += ways[mask];
            }
        }
        ways = nways;
    }
    let mut res = Mod::ZERO;
    for mask in 0..(1 << m) {
        if ((1 << (C - 1)) & mask) != 0 {
            res += ways[mask];
        }
    }
    out_line!(format!("Case #{}: {}", test_case, res));
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
