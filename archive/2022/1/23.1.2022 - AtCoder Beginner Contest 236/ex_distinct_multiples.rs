//{"name":"Ex - Distinct Multiples","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_h","interactive":false,"timeLimit":2000,"tests":[{"input":"3 7\n2 3 4\n","output":"3\n"},{"input":"3 3\n1 2 2\n","output":"0\n"},{"input":"6 1000000000000000000\n380214083 420492929 929717250 666796775 209977152 770361643\n","output":"325683519\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ExDistinctMultiples"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::factorials::facts;
use algo_lib::math::gcd::lcm;
use algo_lib::math::modulo::{Mod_998_244_353, ModuloTrait};
use algo_lib::misc::all_submasks_iter::all_submasks_of;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.i128();
    let d = input.read_vec::<i64>(n);

    let prec_ways = gen_vec(1 << n, |mask| -> Mod {
        if mask == 0 {
            return Mod::ZERO;
        }
        let mut cur_lcm = 1i128;
        for i in 0..n {
            if ((1 << i) & mask) != 0 {
                cur_lcm = lcm(cur_lcm, d[i] as i128);
                if cur_lcm > m {
                    return Mod::ZERO;
                }
            }
        }
        let ways = (m / cur_lcm) % (Mod::mod_value() as i128);
        Mod::new(ways as i32)
    });

    let facts = facts::<Mod>(n + 1);

    let mut dp = vec![Mod::ZERO; 1 << n];
    dp[0] = Mod::ONE;
    for mask in 0..(1 << n) - 1 {
        let cur = dp[mask];
        let first = (0..n).filter(|&id| (1 << id) & mask == 0).next().unwrap();
        let can_use = ((1 << n) - 1) ^ mask ^ (1 << first);
        for add_mask in all_submasks_of(can_use) {
            let nmask = mask | (1 << first) | add_mask;
            let comp_size = add_mask.count_ones() as usize + 1;
            let parity_mult = if comp_size % 2 == 1 {
                Mod::ONE
            } else {
                Mod::ZERO - Mod::ONE
            };
            dp[nmask] +=
                cur * prec_ways[add_mask | (1 << first)] * facts[comp_size - 1] * parity_mult;
        }
    }
    let full_mask = (1 << n) - 1;
    out_line!(dp[full_mask]);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("2");
}
//END MAIN
