//{"name":"A: Second Hands","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/A","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n3 2\n1 2 2\n5 3\n1 2 3 3 1\n5 2\n1 2 3 4 5\n5 5\n1 1 2 2 1\n1 1\n1\n","output":"Case #1: YES\nCase #2: YES\nCase #3: NO\nCase #4: NO\nCase #5: YES\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_hands_.*input[.]txt"},"output":{"type":"file","fileName":"second_hands_output.txt","pattern":null},"languages":{"java":{"taskClass":"ASecondHands"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let max_in_one = input.usize();
    const M: usize = 101;
    let mut cnt = vec![0; M];
    for _ in 0..n {
        cnt[input.usize()] += 1;
    }
    let ok = n <= max_in_one * 2 && cnt.iter().all(|x| *x <= 2);
    out_line!(format!(
        "Case #{}: {}",
        _test_case,
        if ok { "YES" } else { "NO" }
    ));
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        eprintln!("Running on case {}", i + 1);
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::File("second_hands_output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("2");
    // tester::run_stress(stress);
    // submit();
}
//END MAIN
