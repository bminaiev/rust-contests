//{"name":"B1: Second Friend","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/B1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n1 3\n.^.\n3 1\n.\n.\n.\n4 4\n..^.\n..^.\n....\n...^\n","output":"Case #1: Impossible\nCase #2: Possible\n.\n.\n.\nCase #3: Possible\n^^^.\n^.^.\n^^^^\n..^^\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_friend_.*input[.]txt"},"output":{"type":"file","fileName":"second_friend_output.txt","pattern":null},"languages":{"java":{"taskClass":"B1SecondFriend"}}}

use std::fmt::format;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut s = gen_vec(n, |_| input.string());
    const TREE: u8 = b'^';
    let any_tree = s.iter().any(|row| row.iter().any(|&x| x == TREE));
    let ok = if !any_tree {
        true
    } else {
        if n > 1 && m > 1 {
            for i in 0..n {
                for j in 0..m {
                    s[i][j] = TREE;
                }
            }
            true
        } else {
            false
        }
    };
    out_line!(format!(
        "Case #{}: {}",
        _test_case,
        if ok { "Possible" } else { "Impossible" }
    ));
    if ok {
        for i in 0..n {
            out_line!(vec2str(&s[i]));
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
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
        output: TaskIoType::File("second_friend_output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
