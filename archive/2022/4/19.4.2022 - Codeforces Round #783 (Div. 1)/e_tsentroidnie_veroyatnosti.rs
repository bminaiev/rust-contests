//{"name":"E. Центроидные вероятности","group":"Codeforces - Codeforces Round #783 (Div. 1)","url":"https://codeforces.com/contest/1667/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n","output":"1 1 0\n"},{"input":"5\n","output":"10 10 4 0 0\n"},{"input":"7\n","output":"276 276 132 36 0 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETsentroidnieVeroyatnosti"}}}

use std::cmp::max;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::math::ntt::NTT;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let big_size = (n + 1) / 2;
    let mut ways_big_root = vec![Mod::ZERO; n];

    let comb = CombinationsFact::<Mod>::new(n + 1);

    let mut f = vec![Mod::ZERO; n + 1];
    for subtree_size in big_size - 1..n {
        let mut ways = comb.fact(subtree_size);

        let left_vertices = n - subtree_size - 1;
        if left_vertices > 0 {
            ways *= comb.fact(left_vertices - 1);
        }
        f[subtree_size] += ways;
        f[subtree_size] /= comb.fact(subtree_size);
    }

    let mut g = vec![Mod::ZERO; n + 1];
    for i in 0..g.len() {
        g[i] = Mod::ONE / comb.fact(i);
    }

    // let mut fg = vec![Mod::ZERO; 2 * n + 2];
    // for i in 0..=n {
    //     for j in 0..=n {
    //         fg[i + j] += f[i] * g[j];
    //     }
    // }

    let fg = NTT::new().multiply(f, g);

    for root_pos in (1..n).rev() {
        let max_subtree_size = n - root_pos - 1;
        // for subtree_size in 0..=max_subtree_size {
        //     ways_big_root[root_pos] += f[subtree_size] / comb.fact(max_subtree_size - subtree_size);
        // }
        // assert_eq!(ways_big_root[root_pos], fg[max_subtree_size]);
        ways_big_root[root_pos] =
            fg[max_subtree_size] * Mod::new(root_pos) * comb.fact(max_subtree_size);
    }
    for root_pos in (0..1).rev() {
        let max_subtree_size = n - root_pos - 1;
        for subtree_size in big_size - 1..=max_subtree_size {
            let mut ways = comb.fact(subtree_size);

            let left_vertices = n - subtree_size - 1;
            if left_vertices > 0 {
                if root_pos == 0 {
                    continue;
                }
                ways *= comb.fact(left_vertices - 1);
            }
            ways *= comb.c(max_subtree_size, subtree_size);
            ways_big_root[root_pos] += ways;
        }
        if root_pos != 0 {
            ways_big_root[root_pos] *= Mod::new(root_pos);
        }
    }
    let mut pref_sum = Mod::ZERO;
    for i in (0..n).rev() {
        let next_sum = pref_sum + ways_big_root[i] / Mod::new(max(1, i));
        ways_big_root[i] -= pref_sum;
        pref_sum = next_sum;
    }

    out_line!(ways_big_root);
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
