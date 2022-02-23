//{"name":"F. Базис","group":"Codeforces - Educational Codeforces Round 123 (рейтинговый для Div. 2)","url":"https://codeforces.com/contest/1644/problem/F","interactive":false,"timeLimit":6000,"tests":[{"input":"3 2\n","output":"2\n"},{"input":"4 10\n","output":"12\n"},{"input":"13 37\n","output":"27643508\n"},{"input":"1337 42\n","output":"211887828\n"},{"input":"198756 123456\n","output":"159489391\n"},{"input":"123456 198756\n","output":"460526614\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBazis"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::factorials::facts;
use algo_lib::math::modulo::{Mod_998_244_353, ModuloTrait};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn calc_first_prime_divisor(up_to: usize) -> Vec<usize> {
    let mut res = gen_vec(up_to + 1, id);
    for x in 2..res.len() {
        if res[x] == x {
            for y in (x * 2..res.len()).step_by(x) {
                res[y] = x;
            }
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let mut res = Mod::ZERO;

    let first_prime = calc_first_prime_divisor(n);

    let facts = facts::<Mod>(n + 1);

    for copies in 1..=n {
        let mut mult = Mod::ONE;
        {
            let mut cur = copies;
            let mut ignore = false;
            while cur != 1 {
                let prime = first_prime[cur];
                let mut pw = 0;
                while cur % prime == 0 {
                    pw += 1;
                    cur /= prime;
                }
                if pw > 1 {
                    ignore = true;
                    break;
                }
                mult = Mod::ZERO - mult;
            }
            if ignore {
                continue;
            }
        }
        let len = (n + copies - 1) / copies;
        let mut prev_ways = Mod::ZERO;
        for at_most_colors in 1..=min(len, k) {
            let mut ways = Mod::new(at_most_colors).pown(len);
            {
                let next_ways = ways * Mod::new(at_most_colors + 1);
                ways -= prev_ways;
                prev_ways = next_ways;
            }
            ways /= facts[at_most_colors];
            dbg!(copies, at_most_colors, mult, ways);
            if at_most_colors != 1 {
                res += mult * ways;
            }
        }
    }
    if n == 1 {
        res += Mod::ONE;
    }
    out_line!(res);
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
    // tester::run_tests();
    tester::run_single_test("2");
}
//END MAIN
