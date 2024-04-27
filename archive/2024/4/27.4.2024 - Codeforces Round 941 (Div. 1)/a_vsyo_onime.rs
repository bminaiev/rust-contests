//{"name":"A. Всё о Ниме","group":"Codeforces - Codeforces Round 941 (Div. 1)","url":"https://codeforces.com/contest/1965/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5\n3 3 3 3 3\n2\n1 7\n7\n1 3 9 7 4 2 100\n3\n1 2 3\n6\n2 1 3 4 2 4\n8\n5 7 2 9 6 3 3 2\n1\n1000000000\n","output":"Alice\nBob\nAlice\nAlice\nBob\nAlice\nAlice\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AVsyoONime"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut a = input.vec::<i64>(n);
    a.sort();
    a.dedup();
    let mut b = vec![a[0]];
    for i in 1..a.len() {
        b.push(a[i] - a[i - 1]);
    }
    let mut first_non_one = b.len();
    for i in 0..b.len() {
        if b[i] != 1 {
            first_non_one = i;
            break;
        }
    }
    let alice_win = if first_non_one == b.len() {
        b.len() % 2 == 1
    } else {
        first_non_one % 2 == 0
    };
    if alice_win {
        out.println("Alice");
    } else {
        out.println("Bob");
    }
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
    const PROBLEM_NAME: &str = "a_vsyo_onime";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
