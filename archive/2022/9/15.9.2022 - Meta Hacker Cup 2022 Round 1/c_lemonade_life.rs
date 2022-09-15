//{"name":"C: Lemonade Life","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 1","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-1/problems/C","interactive":false,"timeLimit":360000,"tests":[{"input":"5\n9 25 8\n0 5\n1 6\n6 3\n6 7\n3 4\n9 2\n2 1\n1 2\n11 8\n3 100 7\n0 0\n4 1\n7 2\n3 100 7\n0 0\n4 1\n8 2\n6 0 1000000000\n0 10\n2 5\n1 7\n7 4\n8 1\n10 0\n12 1600 2000\n0 30\n16 48\n36 57\n951 45\n397 63\n447 63\n185 16\n362 10\n432 9\n507 11\n643 16\n1000 30\n","output":"Case #1: 115\nCase #2: 200\nCase #3: -1\nCase #4: 56\nCase #5: 184654\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"lemonade_life_.*input[.]txt"},"output":{"type":"file","fileName":"lemonade_life_output.txt","pattern":null},"languages":{"java":{"taskClass":"CLemonadeLife"}}}

use std::cmp::max;

use algo_lib::collections::index_of::IndexOf;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::geometry::convex_hull::convex_hull;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type Point = PointT<i64>;

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        k: i64,
        d: i64,
        a: Vec<Point>,
        res: i64,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            let n = input.usize();
            let k = input.i64();
            let d = input.i64();
            let a = gen_vec(n, |_| Point::new(input.read(), input.read()));
            *self = Job {
                k,
                d,
                a,
                res: std::i64::MAX,
            };
        }

        fn solve(&mut self) {
            let first = self.a[0];
            let last = *self.a.last_exn();
            let hull = convex_hull(&self.a);
            let first_pos = hull.index_of(&first).unwrap();
            let last_pos = hull.index_of(&last).unwrap();
            let mut dp = vec![std::i64::MAX; hull.len()];
            dp[first_pos] = 0;
            let mut seen = vec![false; hull.len()];
            loop {
                let next_id = (0..hull.len())
                    .filter(|pos| !seen[*pos] && dp[*pos] != std::i64::MAX)
                    .min_by_key(|pos| dp[*pos]);

                if let Some(v) = next_id {
                    seen[v] = true;
                    for to in 0..hull.len() {
                        if !seen[to] {
                            let d2 = hull[to].dist2(&hull[v]);
                            if d2 <= self.d * self.d {
                                let ndist = dp[v] + max(self.k, d2);
                                assert!(ndist >= 0);
                                dp[to].update_min(ndist);
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            self.res = dp[last_pos];
            assert!(self.res >= 0);
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!(
                "Case #{}: {}",
                test_case,
                if self.res == std::i64::MAX {
                    -1
                } else {
                    self.res
                }
            ));
        }
    }

    run_parallel::<Job>(input, None);
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
        output: TaskIoType::File("lemonade_life_output.txt".to_string()),
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
