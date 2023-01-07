//{"name":"H. Chaos & Chaos & Chaos","group":"TLX - TLX Regular Open Contest #30","url":"https://tlx.toki.id/contests/troc-30/problems/H","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1 2 4 3 4\n","output":"14\n"},{"input":"4\n9 13 15 7\n","output":"0\n"},{"input":"12\n69 1273 727 420 265 666 1337 0 619 58008 69 265\n","output":"3294\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HChaosChaosChaos"}}}

use std::collections::VecDeque;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<usize>(n);
    let mut ways = Mod::ZERO;
    let pow2 = Mod::gen_powers(Mod::TWO, n + 1);
    let mut max_bit = 0;
    for &v in a.iter() {
        for bit in 0..=20 {
            if ((1 << bit) & v) != 0 {
                max_bit.update_max(bit);
            }
        }
    }
    max_bit += 1;
    let mut graph = Array2D::new(false, max_bit, max_bit);
    for &v in a.iter() {
        for b1 in 0..max_bit {
            for b2 in 0..max_bit {
                if ((1 << b1) & v) == 0 {
                    if ((1 << b2) & v) == 0 {
                        graph[b1][b2] = true;
                    }
                }
            }
        }
    }
    let mut base_mask = usize::MAX;
    for &v in a.iter() {
        base_mask &= v;
    }
    let mut over_masks = vec![0; 1 << max_bit];
    for &v in a.iter() {
        over_masks[v] += 1;
    }
    for bit in 0..max_bit {
        for mask in 0..1 << max_bit {
            if ((1 << bit) & mask) != 0 {
                over_masks[mask ^ (1 << bit)] += over_masks[mask];
            }
        }
    }
    let mut queue = VecDeque::new();
    for mask in 0..(1 << max_bit) {
        if (mask & base_mask) != 0 {
            continue;
        }
        let mut seen = vec![false; max_bit];
        let mut cnt_groups = 0;
        for bit in 0..max_bit {
            if ((1 << bit) & mask) != 0 && !seen[bit] {
                seen[bit] = true;
                cnt_groups += 1;
                queue.push_back(bit);
                while let Some(cur_bit) = queue.pop_front() {
                    for next in 0..max_bit {
                        if ((1 << next) & mask) != 0 && graph[cur_bit][next] && !seen[next] {
                            seen[next] = true;
                            queue.push_back(next);
                        }
                    }
                }
            }
        }
        let cnt_zeros = n - over_masks[mask];
        // for &v in a.iter() {
        //     if (v & mask) != mask {
        //         cnt_zeros += 1;
        //     }
        // }
        let mut cur_ways = pow2[n - cnt_zeros];
        cur_ways *= pow2[cnt_groups];
        if mask.count_ones() % 2 == 1 {
            cur_ways = Mod::ZERO - cur_ways;
        }
        ways += cur_ways;
    }
    out_line!(ways);
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
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
