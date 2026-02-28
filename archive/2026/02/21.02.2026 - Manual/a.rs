//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use algo_lib::collections::fx_hash_map::FxHashMap;
use algo_lib::collections::sparse_table_min::SparseTableMin;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
use algo_lib::misc::vec_apply_delta::ApplyDelta;

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let a = input.vec::<usize>(n).sub_from_all(1);
    let sparse = SparseTableMin::new(&a);

    let mut cache = FxHashMap::default();
    let mut answer = vec![0u64; n + 1];
    RecursiveFunction2::new(|f, l: usize, r: usize| -> usize {
        let len = r - l;
        if len < 2 {
            return 0;
        }
        if let Some(&res) = cache.get(&(l, r)) {
            return res;
        }
        let min_pos1 = sparse.find_min_pos(l..r);
        let min_pos2 = if l < min_pos1 {
            Some(sparse.find_min_pos(l..min_pos1))
        } else {
            None
        };
        let min_pos3 = if min_pos1 + 1 < r {
            Some(sparse.find_min_pos(min_pos1 + 1..r))
        } else {
            None
        };
        let actual_min_pos = if let Some(pos2) = min_pos2 {
            if let Some(pos3) = min_pos3 {
                if a[pos2] < a[pos3] {
                    pos2
                } else {
                    pos3
                }
            } else {
                pos2
            }
        } else if let Some(pos3) = min_pos3 {
            pos3
        } else {
            panic!();
        };
        let left = min_pos1.min(actual_min_pos);
        let right = min_pos1.max(actual_min_pos);
        let tmp = f.call(left + 1, right) + 1;
        cache.insert((l, r), tmp);
        let ways = (left - l + 1) as u64 * (r - right) as u64;
        // dbg!(l, r, left, right, ways, tmp);
        answer[tmp] += ways;
        {
            // dbg!("calling", l, right);
            f.call(l, right);
            // dbg!("calling", left + 1, r);
            f.call(left + 1, r);
        }
        tmp
    })
    .call(0, n);
    for k in 1..=n {
        out.println(answer[k]);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "2");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
