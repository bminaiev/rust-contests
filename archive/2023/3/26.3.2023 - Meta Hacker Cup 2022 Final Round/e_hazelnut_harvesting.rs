//{"name":"E: Hazelnut Harvesting","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Final Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/final-round/problems/E","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n4\n0 0\n2 1\n5 0\n0 6\n5\n10 10\n12 10\n8 10\n5 10\n8 8\n4\n1 1\n3 3\n0 4\n4 6\n","output":"Case #1: 20\nCase #2: 28\nCase #3: 42\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"hazelnut_harvesting_.*input[.]txt"},"output":{"type":"file","fileName":"hazelnut_harvesting_output.txt","pattern":null},"languages":{"java":{"taskClass":"EHazelnutHarvesting"}}}

use std::cmp::{max, min};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    // let tc = input.usize();
    // for test_case in 1..=tc {
    //     dbg!(test_case);

    //     out_line!(format!("Case #{}: ", test_case));
    // }
    run_parallel::<Job>(input, Some(8), &());
}

#[derive(Clone, Debug, Copy)]
struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn intersect_one(x1: i32, x2: i32, x3: i32, x4: i32) -> bool {
    max(x1, x3) <= min(x2, x4)
}

impl Rect {
    pub fn intersects(&self, other: &Self) -> bool {
        intersect_one(self.x1, self.x2, other.x1, other.x2)
            && intersect_one(self.y1, self.y2, other.y1, other.y2)
    }

    pub fn join(&self, other: &Self) -> Self {
        Self {
            x1: min(self.x1, other.x1),
            y1: min(self.y1, other.y1),
            x2: max(self.x2, other.x2),
            y2: max(self.y2, other.y2),
        }
    }

    pub fn area(&self) -> i64 {
        let dx = (self.x2 - self.x1) as i64;
        let dy = (self.y2 - self.y1) as i64;
        dx * dy
    }
}

#[derive(Clone, Default)]
struct Job {
    rects: Vec<Rect>,
    res: i64,
}

impl ParallelJob for Job {
    type Context = ();

    fn read_input(&mut self, input: &mut Input) {
        let n = input.usize();
        for _ in 0..n {
            let x = input.i32();
            let y = input.i32();
            self.rects.push(Rect {
                x1: x - 1,
                y1: y - 1,
                x2: x + 1,
                y2: y + 1,
            });
        }
    }

    fn solve(&mut self, context: &Self::Context) {
        let mut rnd = Random::new(787788);
        rnd.shuffle(&mut self.rects);
        let mut cur_rects: Vec<Rect> = vec![];
        let mut to_remove = vec![];
        for r in self.rects.iter() {
            let mut r = *r;
            loop {
                for (pos, existing) in cur_rects.iter().enumerate() {
                    if r.intersects(existing) {
                        to_remove.push(pos);
                        r = r.join(existing);
                    }
                }
                if to_remove.is_empty() {
                    break;
                }
                while let Some(p) = to_remove.pop() {
                    cur_rects.swap_remove(p);
                }
            }
            cur_rects.push(r);
        }
        for r in cur_rects.iter() {
            self.res += r.area();
        }
    }

    fn write_output(&mut self, test_case: usize) {
        out_line!(format!("Case #{}: {}", test_case, self.res));
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
        output: TaskIoType::File("hazelnut_harvesting_output.txt".to_string()),
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
