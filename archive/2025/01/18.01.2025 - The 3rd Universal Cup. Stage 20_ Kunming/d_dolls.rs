//{"name":"D. Dolls","group":"Universal Cup - The 3rd Universal Cup. Stage 20: Kunming","url":"https://contest.ucup.ac/contest/1871/problem/9865","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n4\n2 1 4 3\n4\n1 4 2 3\n4\n3 1 4 2\n5\n1 3 5 2 4\n5\n1 4 2 5 3\n5\n2 5 3 1 4\n6\n1 3 6 5 2 4\n6\n2 5 1 3 6 4\n","output":"3\n3\n2\n3\n3\n3\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDolls"}}}

use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;

fn can_split(a: &[usize], from: usize) -> bool {
    let n = a.len();
    if n <= 2 {
        return true;
    }
    let to = from + a.len();
    let mut prefix_min = usize::MAX;
    let mut suffix_min = usize::MAX;
    let mut prefix_max = 0;
    let mut suffix_max = 0;
    for len in 1..a.len() {
        prefix_min = prefix_min.min(a[len - 1] - from);
        prefix_max = prefix_max.max(a[len - 1] - from);
        suffix_min = suffix_min.min(a[n - len] - from);
        suffix_max = suffix_max.max(a[n - len] - from);
        if prefix_max == len - 1 {
            return can_split(&a[..len], from) && can_split(&a[len..], from + len);
        }
        if prefix_min + len == to {
            return can_split(&a[..len], to - len) && can_split(&a[len..], from);
        }
        if suffix_max == len - 1 {
            return can_split(&a[..n - len], from + len) && can_split(&a[n - len..], from);
        }
        if suffix_min + len == to {
            return can_split(&a[..n - len], from) && can_split(&a[n - len..], to - len);
        }
    }
    false
}

fn is_good(a: &[usize]) -> bool {
    let mut all_vals = a.to_vec();
    all_vals.sort();
    let mut a = a.to_vec();
    for i in 0..a.len() {
        a[i] = all_vals.binary_search(&a[i]).unwrap();
    }
    can_split(&a, 0)
}

fn solve_case(mut a: &[usize]) -> usize {
    let n = a.len();
    let mut n_comps = 0;
    while !a.is_empty() {
        let mut ok_len = 1;
        while ok_len < a.len() {
            let test_len = (ok_len * 2).min(a.len());
            if is_good(&a[..test_len]) {
                ok_len = test_len;
            } else {
                break;
            }
        }
        if ok_len != a.len() {
            let bad_len = a.len().min(ok_len * 2);
            ok_len =
                binary_search_first_true(ok_len..bad_len, |check_len| !is_good(&a[..check_len]))
                    - 1;
        }
        n_comps += 1;
        a = &a[ok_len..];
    }
    n - n_comps
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = 100_000;
        let perm = rnd.gen_permutation(n);
        let start = Instant::now();
        let res = solve_case(&perm);
        dbg!(res);
        dbg!(start.elapsed());
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let a = input.vec::<usize>(n);
        let res = solve_case(&a);
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "d_dolls";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
