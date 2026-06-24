//{"name":"A - Min of Sum of XOR","group":"AtCoder - AtCoder Regular Contest 217","url":"https://atcoder.jp/contests/arc217/tasks/arc217_a","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n1\n7\n","output":"1 3 2\n1\n4 5 3 2 6 7 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::collections::BTreeMap;

use algo_lib::collections::permutation::Permutation;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let ans = solve_case(n);
        out.println(ans);
    }
}

fn solve_case(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    }
    if n == 2 {
        return vec![1, 2];
    }
    let mut res = vec![];
    for i in (1..=n).rev() {
        res.push(i);
    }
    if n % 2 == 1 {
        res.swap(0, 1);
        return res;
    } else {
        res.remove(0);
        res.swap(0, 1);
        res.push(n);
    }
    res
}

fn calc_score(a: &[usize]) -> usize {
    let mut ans = 0;
    let mut xor = 0;
    for i in 0..a.len() {
        xor ^= a[i] + 1;
        ans += xor;
    }
    ans
}

fn stress() {
    let n = 7;
    let mut p = Permutation::new(n);
    let mut hm = BTreeMap::<usize, Vec<Vec<usize>>>::new();
    loop {
        let score = calc_score(&p.ids);
        let ids_add_one = p.ids.iter().map(|x| x + 1).collect::<Vec<_>>();
        hm.entry(score).or_default().push(ids_add_one);
        if !p.next() {
            break;
        }
    }
    for (score, v) in hm.iter() {
        dbg!(score);
        for vv in v.iter() {
            dbg!(vv);
        }
        if true {
            break;
        }
    }
    let ans = solve_case(n);
    dbg!(ans);
    let ans_sub_one = ans.iter().map(|x| x - 1).collect::<Vec<_>>();
    dbg!(calc_score(&ans_sub_one));
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a_min_of_sum_of_xor";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
