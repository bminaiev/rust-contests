//{"name":"B: Third Trie","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n3\n3\n1 a\n1 b\n3\n1 a\n2 a\n2\n1 c\n4\n2\n1 a\n2\n1 a\n2\n1 a\n4\n1 a\n2 a\n3 a\n4\n2\n1 a\n2\n1 b\n2\n1 c\n4\n1 a\n2 b\n3 c\n","output":"Case #1: 5\nCase #2: 14\nCase #3: 20\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"third_trie_.*input[.]txt"},"output":{"type":"file","fileName":"third_trie_output.txt","pattern":null},"languages":{"java":{"taskClass":"BThirdTrie"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    let tc = input.usize();
    for test_case in 1..=tc {
        dbg!(test_case);
        let n = input.usize();
        let mut tot = vec![n as i64];
        const MX: usize = std::usize::MAX;
        let mut next = vec![vec![MX; 26]];
        for _ in 0..n {
            let sz = input.usize();
            let mut mapping = vec![0; sz];
            for i in 1..sz {
                let prev = input.usize() - 1;
                let c = (input.string()[0] - b'a') as usize;
                let all_v = mapping[prev];
                if next[all_v][c] == MX {
                    next[all_v][c] = tot.len();
                    tot.push(0);
                    next.push(vec![MX; 26]);
                }
                mapping[i] = next[all_v][c];
                tot[mapping[i]] += 1;
            }
        }
        let mut res = 0;
        let n = n as i64;
        let full = n * (n - 1) * (n - 2) / 6;
        for i in 0..tot.len() {
            let cnt_ok = tot[i];
            let cnt_bad = (n - cnt_ok) as i64;
            let full_bad = cnt_bad * (cnt_bad - 1) * (cnt_bad - 2) / 6;
            res += full - full_bad;
        }
        out_line!(format!("Case #{}: {}", test_case, res));
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
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::File("third_trie_output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    tester::run_with_last_downloaded_file();
}
//END MAIN
