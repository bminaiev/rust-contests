//{"name":"C: Balance Scale","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"6\n5 1\n1 3000\n1 2000\n1 1000\n1 2000\n1 1000\n5 2\n1 3000\n1 2000\n1 1000\n1 2000\n1 1000\n2 10\n10 1\n10 2\n5 2\n2 50\n1 40\n1 50\n1 60\n3 50\n4 2993\n3000 999999999\n2995 1000000000\n1552 888888888\n1336 999999999\n3 1\n1 10\n2 9\n1 11\n","output":"Case #1: 800000006\nCase #2: 200000002\nCase #3: 0\nCase #4: 208333335\nCase #5: 590307096\nCase #6: 333333336\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"balance_scale_.*input[.]txt"},"output":{"type":"file","fileName":"balance_scale_output.txt","pattern":null},"languages":{"java":{"taskClass":"CBalanceScale"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

use algo_lib::misc::gen_vector::gen_vec;
type Mod = Mod7;
use algo_lib::math::combinations::Combinations;
use algo_lib::math::combinations::CombinationsFact;

#[derive(Copy, Clone)]
struct Group {
    cnt: usize,
    w: i32,
}

static mut cnk_static: Option<CombinationsFact<Mod>> = None;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        k: usize,
        groups: Vec<Group>,
        res: Mod,
    }

    const MAX_SUM: usize = 3000 * 3000;
    unsafe {
        cnk_static = Some(CombinationsFact::<Mod>::new(MAX_SUM + 10));
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            let n = input.usize();
            self.k = input.usize();
            self.groups = gen_vec(n, |_| Group {
                cnt: input.usize(),
                w: input.read(),
            })
        }

        fn solve(&mut self) {
            let mut sum = 0;
            for group in self.groups.iter() {
                sum += group.cnt;
            }

            let cnk = unsafe {
                match &cnk_static {
                    Some(x) => x,
                    None => todo!(),
                }
            };
            let total = cnk.c(sum, self.k + 1);
            let mut ways = Mod::ZERO;
            let mut tot_same = 0;
            let mut tot_less = 0;
            for group in self.groups.iter() {
                if group.w < self.groups[0].w {
                    tot_less += group.cnt;
                } else if group.w == self.groups[0].w {
                    tot_same += group.cnt;
                }
            }
            tot_same -= self.groups[0].cnt;
            for my_weight in 1..=self.groups[0].cnt + tot_same {
                if my_weight > self.k + 1 {
                    continue;
                }
                let more = self.k + 1 - my_weight;
                if more > tot_less {
                    continue;
                }
                ways += cnk.c(self.groups[0].cnt + tot_same, my_weight)
                    * cnk.c(tot_less, more)
                    * Mod::new(self.groups[0].cnt)
                    / Mod::new(self.groups[0].cnt + tot_same);
            }
            self.res = ways / total;
        }

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
        output: TaskIoType::File("balance_scale_output.txt".to_string()),
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
