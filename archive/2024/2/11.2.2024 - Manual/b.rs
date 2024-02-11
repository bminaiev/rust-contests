//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

use algo_lib::collections::bit_set::BitSet;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    const MAX_C: usize = 250_001;
    let mut by_color = vec![vec![]; n];
    let mut a = vec![usize::MAX; MAX_C];
    let mut full = BitSet::new(MAX_C);
    let mut positions = vec![];
    for _ in 0..n {
        let pos = input.usize() - 1;
        let color = input.usize() - 1;
        positions.push(pos);
        by_color[color].push(pos);
        a[pos] = color;
        full.set(pos, true);
    }
    let mut bitsets = vec![None; n];
    const BIG: usize = 787;
    for color in 0..n {
        if by_color[color].len() > BIG {
            let mut bitset = BitSet::new(MAX_C);
            for &pos in &by_color[color] {
                bitset.set(pos, true);
            }
            bitsets[color] = Some(bitset);
        }
    }
    let mut res = vec![usize::MAX; MAX_C];
    let mut need = BitSet::new(MAX_C);
    for i in 0..MAX_C {
        need.set(i, true);
    }
    for i in 0..n {
        let pos = positions[i];
        let my_color = a[pos];
        if my_color == usize::MAX {
            continue;
        }
        let mut cur_bs = full.clone();
        if let Some(bs) = &bitsets[my_color] {
            cur_bs ^= bs;
        } else {
            for &pos in &by_color[my_color] {
                cur_bs.set(pos, false);
            }
        }

        let mut cur_bs = cur_bs.shift_lower(pos);
        cur_bs &= &need;
        let mut cur_pos = 1;
        while let Some(d) = cur_bs.first_set(cur_pos) {
            assert!(res[d] == usize::MAX);
            res[d] = i;
            need.set(d, false);
            cur_pos = d + 1;
        }
    }
    for _ in 0..q {
        let d = input.usize();
        if res[d] == usize::MAX {
            out.println(0);
        } else {
            out.println(res[d] + 1);
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
