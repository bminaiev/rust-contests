//{"name":"C1: Second Meaning","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Qualification Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/qualification-round/problems/C1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n3\n.-.\n4\n-\n3\n..\n","output":"Case #1:\n...\n---\nCase #2:\n...\n.-\n..-\nCase #3:\n-\n.-\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"second_meaning_.*input[.]txt"},"output":{"type":"file","fileName":"second_meaning_output.txt","pattern":null},"languages":{"java":{"taskClass":"C1SecondMeaning"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    let tc = input.usize();
    for test_case in 1..=tc {
        dbg!(test_case);
        let n = input.usize();
        let first = input.string();
        let last = *first.last_exn();
        let prev_last = last ^ b'-' ^ b'.';
        out_line!(format!("Case #{}: ", test_case));
        for cnt in 1..n {
            let res: Vec<_> = first[..first.len() - 1]
                .iter()
                .chain(vec![prev_last; cnt].iter())
                .chain([last].iter())
                .cloned()
                .collect();
            out_line!(vec2str(&res));
        }
    }
    // run_parallel::<Job>(input, Some(1), &());
}

#[derive(Clone, Default)]
struct Job {}

impl ParallelJob for Job {
    type Context = ();

    fn read_input(&mut self, input: &mut Input) {}

    fn solve(&mut self, context: &Self::Context) {}

    fn write_output(&mut self, test_case: usize) {
        out_line!(format!("Case #{}: ", test_case));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::File("second_meaning_output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
