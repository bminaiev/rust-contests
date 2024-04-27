//{"name":"C. Запас билетов","group":"Codeforces - Codeforces Global Round 25","url":"https://codeforces.com/contest/1951/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 2 3\n8 6 4 2\n4 2 8\n8 6 4 2\n5 100 1\n10000 1 100 10 1000\n6 3 9\n5 5 5 5 5 5\n","output":"10\n64\n1\n72\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CZapasBiletov"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let max_per_day = input.i64();
    let mut need = input.i64();
    let mut res = need * (need - 1) / 2;
    let mut a = input.vec::<i64>(n);
    a.sort();
    for &price in a.iter() {
        let use_here = max_per_day.min(need);
        res += use_here * price;
        res -= use_here * (use_here - 1) / 2;
        need -= use_here;
    }
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
    const PROBLEM_NAME: &str = "c_zapas_biletov";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
