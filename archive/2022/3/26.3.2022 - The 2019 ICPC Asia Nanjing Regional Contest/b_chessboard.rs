//{"name":"B. Chessboard","group":"Codeforces - The 2019 ICPC Asia Nanjing Regional Contest","url":"https://codeforces.com/gym/103466/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 3\n3 2\n3 3\n4 4\n","output":"2\n12\n24\n80\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BChessboard"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod7;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve_case(n: usize, m: usize, comb: &CombinationsFact<Mod>) -> Mod {
    if n == 1 && m == 1 {
        return Mod::ONE;
    }
    if n == 1 || m == 1 {
        return Mod::TWO;
    }
    let mut res = Mod::ZERO;
    res += comb.c(n + m - 3, n - 2);
    res += comb.c(n + m - 3, n - 1);
    res * Mod::new(4i32)
}

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    let comb = CombinationsFact::<Mod>::new(2_000_010);
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        out_line!(solve_case(n, m, &comb));
    }
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
}
//END MAIN
