//{"name":"C. Аля и перестановка","group":"Codeforces - Codeforces Global Round 27","url":"https://codeforces.com/contest/2035/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\n6\n7\n8\n9\n10\n","output":"5\n2 1 3 4 5\n7\n1 2 4 6 5 3\n7\n2 4 5 1 3 6 7\n15\n2 4 5 1 3 6 7 8\n9\n2 4 5 6 7 1 3 8 9\n15\n1 2 3 4 5 6 8 10 9 7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAlyaIPerestanovka"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve_case(n: usize) -> Vec<usize> {
    let mut rnd = Random::new(234234);
    let mut res = vec![];
    for i in 1..=n {
        res.push(i);
    }
    let high2 = (n - 2).next_power_of_two() / 2 - 1;
    res.swap(n - 3, high2);
    res.swap(n - 4, high2 - 1);
    let mut best = (res.clone(), calc_score(&res));
    for _it in 0..500 {
        rnd.shuffle(&mut res[n - 5..]);
        let nscore = calc_score(&res);
        if nscore > best.1 {
            best = (res.clone(), nscore);
        }
    }
    best.0
}

fn calc_score(a: &[usize]) -> usize {
    let mut res = 0;
    for i in 0..a.len() {
        if i % 2 == 0 {
            res = res & a[i];
        } else {
            res = res | a[i];
        }
    }
    res
}

fn stress() {
    for n in 5..2000 {
        let res = solve_case(n);
        let score = calc_score(&res);
        dbg!(n);
        if n % 2 == 1 {
            assert_eq!(score, n);
        } else {
            let expected_score = (n + 1).next_power_of_two() - 1;
            assert_eq!(score, expected_score);
        }
        dbg!(n, calc_score(&res));
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let res = solve_case(n);
    out.println(calc_score(&res));
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "c_alya_iperestanovka";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
