//{"name":"B: Balance Sheet","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 2","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-2/problems/B","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n8 10\n1 2 10 20\n2 4 25 30\n1 4 45 40\n4 5 35 20\n1 5 10 15\n5 6 45 30\n6 7 30 40\n8 9 80 90\n8 6\n1 2 10 20\n2 4 25 30\n1 4 45 40\n4 5 35 20\n1 5 10 15\n5 6 45 30\n6 7 30 40\n8 9 80 90\n2 1\n1 2 10 20\n3 4 30 40\n2 1\n2 3 30 40\n1 2 10 20\n","output":"Case #1: 140\nCase #2: 135\nCase #3: 0\nCase #4: 10\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"balance_sheet_.*input[.]txt"},"output":{"type":"file","fileName":"balance_sheet_output.txt","pattern":null},"languages":{"java":{"taskClass":"BBalanceSheet"}}}

use std::cmp::max;
use std::collections::{BTreeSet, BinaryHeap};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
use algo_lib::seg_trees::lazy_seg_tree_max::SegTreeMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::seg_trees::lazy_seg_tree_max::MaxValNode;

#[derive(Clone, Copy, Debug)]
struct Client {
    start: usize,
    end: usize,
    buy: i64,
    sell: i64,
}

type Mod = Mod7;
type SegTree = SegTreeMax<i64>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Path {
    expected_max_ans: i64,
    cur_ans: i64,
    cur_id: usize,
    fr: usize,
    to: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Result {
    result: i64,
    id: usize,
}

fn solve(input: &mut Input) {
    #[derive(Clone, Default)]
    struct Job {
        clients: Vec<Client>,
        k: usize,
        res: Mod,
    }

    impl ParallelJob for Job {
        fn read_input(&mut self, input: &mut Input) {
            let n = input.usize();
            self.k = input.usize();
            self.clients = gen_vec(n, |_| Client {
                start: input.read(),
                end: input.read(),
                buy: input.read(),
                sell: input.read(),
            });
        }

        fn solve(&mut self) {
            let mut times = vec![];
            for c in self.clients.iter() {
                times.push(c.start);
                times.push(c.end);
            }
            times.sort();
            times.dedup();
            for c in self.clients.iter_mut() {
                c.start = times.binary_search(&c.start).unwrap();
                c.end = times.binary_search(&c.end).unwrap();
            }
            let mut starts_here = vec![vec![]; times.len()];
            for i in 0..self.clients.len() {
                starts_here[self.clients[i].start].push(i);
            }
            for i in 0..starts_here.len() {
                starts_here[i].sort_by_key(|&id| self.clients[id].buy);
            }

            let from_time = gen_vec(self.clients.len(), |id| {
                let time_to = self.clients[id].end;
                binary_search_first_true(0..starts_here[time_to].len(), |check_x| {
                    self.clients[starts_here[time_to][check_x]].buy > self.clients[id].sell
                })
            });
            let mut seg_trees = gen_vec(starts_here.len(), |time| {
                SegTree::new_f(max(1, starts_here[time].len()), &|pos| MaxValNode {
                    max_val: -1,
                    pos,
                })
            });
            let get_best =
                |seg_trees: &mut [SegTree], time: usize, fr: usize, to: usize| -> MaxValNode<i64> {
                    if fr == to {
                        return MaxValNode {
                            max_val: -1,
                            pos: 0,
                        };
                    }
                    seg_trees[time].get(fr..to)
                };
            let mut dp = vec![0; self.clients.len()];
            for time in (0..starts_here.len()).rev() {
                for i in 0..starts_here[time].len() {
                    let id = starts_here[time][i];
                    let time_to = self.clients[id].end;
                    let best = get_best(
                        &mut seg_trees,
                        time_to,
                        from_time[id],
                        starts_here[time_to].len(),
                    );
                    if best.max_val == -1 {
                        dp[id] = 0;
                    } else {
                        dp[id] = best.max_val - self.clients[id].sell;
                    }
                    seg_trees[time].update(i..i + 1, dp[id] + self.clients[id].buy);
                }
            }
            let mut pq = BinaryHeap::<Path>::new();
            for id in 0..self.clients.len() {
                pq.push(Path {
                    cur_id: id,
                    expected_max_ans: dp[id],
                    fr: from_time[id],
                    to: starts_here[self.clients[id].end].len(),
                    cur_ans: 0,
                });
            }
            let mut all_results: BTreeSet<Result> = BTreeSet::new();
            let mut bs_iter = 0;
            while let Some(path) = pq.pop() {
                if all_results.len() == self.k
                    && all_results.iter().next().unwrap().result > path.expected_max_ans
                {
                    break;
                }
                let best = get_best(
                    &mut seg_trees,
                    self.clients[path.cur_id].end,
                    path.fr,
                    path.to,
                );
                if best.max_val != -1 {
                    let next_id = starts_here[self.clients[path.cur_id].end][best.pos];
                    let earned_here = self.clients[next_id].buy - self.clients[path.cur_id].sell;
                    let cur_ans = earned_here + path.cur_ans;
                    all_results.insert(Result {
                        result: cur_ans,
                        id: bs_iter,
                    });
                    if all_results.len() > self.k {
                        let key = all_results.iter().next().unwrap().clone();
                        all_results.remove(&key);
                    }
                    bs_iter += 1;
                    pq.push(Path {
                        cur_ans,
                        cur_id: next_id,
                        fr: from_time[next_id],
                        to: starts_here[self.clients[next_id].end].len(),
                        expected_max_ans: path.expected_max_ans,
                    });
                    for &left in [false, true].iter() {
                        let mut fr = path.fr;
                        let mut to = path.to;
                        if left {
                            to = best.pos;
                        } else {
                            fr = best.pos + 1;
                        }
                        assert!(fr != path.fr || to != path.to);
                        if fr < to {
                            let best_here =
                                get_best(&mut seg_trees, self.clients[path.cur_id].end, fr, to);
                            if best_here.max_val != -1 {
                                let delta = best.max_val - best_here.max_val;
                                pq.push(Path {
                                    cur_ans: path.cur_ans,
                                    cur_id: path.cur_id,
                                    fr,
                                    to,
                                    expected_max_ans: path.expected_max_ans - delta,
                                });
                            }
                        }
                    }
                }
            }
            for res in all_results.iter() {
                let res_mod = res.result % 1_000_000_007;
                self.res += Mod::new(res_mod);
            }
        }

        fn write_output(&mut self, test_case: usize) {
            out_line!(format!("Case #{}: {}", test_case, self.res));
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
        output: TaskIoType::File("balance_sheet_output.txt".to_string()),
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
