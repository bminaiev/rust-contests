//{"name":"B1: Watering Well - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-1/problems/B1","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n2\n2 2\n5 5\n2\n2 5\n6 6\n4\n1 1\n4 3\n6 3\n6 5\n3\n3 1\n5 2\n6 5\n8\n2837 745\n62 1162\n2634 1112\n1746 2618\n847 127\n986 1993\n732 1273\n2003 1998\n4\n1276 2231\n1234 1234\n287 2371\n3000 3000\n","output":"Case #1: 52\nCase #2: 131\nCase #3: 110090622\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"watering_well_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"watering_well_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"B1WateringWellChapter1"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type Mod = Mod7;

#[derive(Clone, Copy, Default)]
struct ByCoord {
    cnt: usize,
    sum: Mod,
    sum_sq: Mod,
}

impl ByCoord {
    pub fn add(&mut self, val: i32) {
        let val = Mod::new(val);
        self.cnt += 1;
        self.sum += val;
        self.sum_sq += val * val;
    }

    pub fn calc_val(&self, coord: i32) -> Mod {
        let coord = Mod::new(coord);
        Mod::new(self.cnt) * coord * coord + self.sum_sq - Mod::TWO * self.sum * coord
    }
}

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        res: Mod,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            let n = input.usize();
            let mut xx = ByCoord::default();
            let mut yy = ByCoord::default();
            for _ in 0..n {
                xx.add(input.read());
                yy.add(input.read());
            }
            let mut res = Mod::ZERO;
            let q = input.usize();
            for _ in 0..q {
                res += xx.calc_val(input.read()) + yy.calc_val(input.read());
            }
            self.res = res;
        }

        fn solve(&mut self) {}

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}: {}", test_case, self.res));
        }
    }

    run_parallel::<Job>(input, Some(1));
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
        output: TaskIoType::File("watering_well_chapter__output.txt".to_string()),
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
