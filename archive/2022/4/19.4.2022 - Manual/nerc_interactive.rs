//{"name":"nerc_interactive","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"nerc_interactive"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::geometry::bounding_box::BoundingBox;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::pairs_iter::PairsIterTrait;
use algo_lib::misc::ordered_pair::OrderedPair;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;
type State = OrderedPair<Point>;

fn solve_case(n: i32, m: i32, interactor: &mut impl Interactor) {
    let mut pts = vec![];
    for x in 0..n {
        for y in 0..m {
            pts.push(Point::new(x, y));
        }
    }
    let states: Vec<_> = pts
        .iter()
        .pairs()
        .map(|(p1, p2)| State::new(*p1, *p2))
        .collect();

    let split = |states: &[State], query: Point| -> Vec<Vec<State>> {
        let mut res = vec![vec![]; (n + m) as usize * 2];
        for s in states.iter() {
            let d = s.min.dist_manh(&query) + s.max.dist_manh(&query);
            res[d as usize].push(s.clone());
        }
        res
    };

    let mut dfs = RecursiveFunction3::new(
        |f, states: Vec<State>, more_queries: usize, use_interactor: bool| -> bool {
            if states.len() == 2 && more_queries >= 1 {
                if use_interactor {
                    let s1 = states[0];
                    let s2 = states[1];
                    if !interactor.dig(s1.min) {
                        assert!(interactor.dig(s2.min));
                        assert!(interactor.dig(s2.max));
                    } else {
                        if !interactor.dig(s1.max) {
                            let next = if s2.min == s1.min { s2.max } else { s2.min };
                            assert!(interactor.dig(next));
                        }
                    }
                }
                return true;
            }
            if states.len() == 0 {
                assert!(!use_interactor);
                return true;
            }
            if states.len() == 1 {
                if use_interactor {
                    let s = states[0];
                    assert!(interactor.dig(s.min));
                    assert!(interactor.dig(s.max));
                }
                return true;
            }
            if more_queries == 0 {
                assert!(!use_interactor);
                return false;
            }

            let mut rnd = Random::new(787788);
            let mut bbox = BoundingBox::new(&states[0].min, &states[0].max);
            for s in states.iter() {
                bbox.add(&s.min);
                bbox.add(&s.max);
            }

            let best_query = if more_queries == 5 {
                Some(Point::ZERO)
            } else {
                let queries = pts
                    .iter()
                    .filter(|p| bbox.contains(p))
                    .map(|&query| {
                        let score = split(&states, query).iter().map(|v| v.len()).max().unwrap();
                        (score, rnd.gen_u64(), query)
                    })
                    .collect::<Vec<_>>()
                    .sorted();
                let mut res = None;
                for (_, _, query) in queries.into_iter().take(4) {
                    let mut good_split = true;

                    let splits = split(&states, query);

                    for sub_states in splits.iter() {
                        if !f.call(sub_states.clone(), more_queries - 1, false) {
                            good_split = false;
                            break;
                        }
                    }
                    if good_split {
                        res = Some(query);
                        break;
                    }
                }
                res
            };
            if let Some(query) = best_query {
                if use_interactor {
                    let dist = interactor.scan(query);
                    let splits = split(&states, query);
                    assert!(f.call(splits[dist].clone(), more_queries - 1, true));
                }
                return true;
            }
            assert!(!use_interactor);
            false
        },
    );
    assert!(dfs.call(states, 5, true));
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let m = input.i32();
    let mut interactor = RealInteractor { input };
    solve_case(n, m, &mut interactor);
}

trait Interactor {
    fn dig(&mut self, p: Point) -> bool;
    fn scan(&mut self, p: Point) -> usize;
}

struct RealInteractor<'a> {
    input: &'a mut Input,
}

impl<'a> Interactor for RealInteractor<'a> {
    fn dig(&mut self, p: Point) -> bool {
        out_line!("DIG", p.x + 1, p.y + 1);
        output().flush();
        self.input.i32() == 1
    }

    fn scan(&mut self, p: Point) -> usize {
        out_line!("SCAN", p.x + 1, p.y + 1);
        output().flush();
        self.input.usize()
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
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_stress(stress);
    // tester::run_single_test("1");
    tester::run_locally();
}
//END MAIN
