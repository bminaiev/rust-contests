use std::collections::VecDeque;
use std::time::Instant;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Node {
    weight: i64,
    cnt_items: i64,
}

struct MyQueue {
    queues: Vec<VecDeque<Node>>,
}

impl MyQueue {
    fn new(mut v: Vec<Node>) -> Self {
        v.sort();
        let main_queue = VecDeque::from(v);
        Self {
            queues: vec![main_queue, VecDeque::new()],
        }
    }

    fn assert_good(&self) {
        // for id in 0..2 {
        //     for i in 1..self.queues[id].len() {
        //         assert!(self.queues[id][i - 1].weight <= self.queues[id][i].weight);
        //     }
        // }
    }

    fn pop(&mut self) -> Option<Node> {
        self.assert_good();

        self.relax();
        if self.queues[1].is_empty() {
            return self.queues[0].pop_front();
        }
        let first = self.queues[0].front().unwrap();
        let second = self.queues[1].front().unwrap();
        if first <= second {
            self.queues[0].pop_front()
        } else {
            self.queues[1].pop_front()
        }
    }

    fn return_back(&mut self, node: Node) {
        self.assert_good();

        for id in 0..2 {
            if self.queues[id].is_empty() || self.queues[id].front().unwrap().weight >= node.weight
            {
                self.queues[id].push_front(node);
                return;
            }
        }
        unreachable!();
    }

    fn push(&mut self, node: Node) {
        // dbg!("push", node);
        self.assert_good();

        self.relax();
        if !self.queues[1].is_empty() {
            assert!(self.queues[1].back().unwrap().weight <= node.weight);
        }
        self.queues[1].push_back(node);
    }

    fn relax(&mut self) {
        if self.queues[0].is_empty() {
            // dbg!("Swap");
            self.queues.swap(0, 1);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.i64();
    let m = input.usize();
    let mut left = vec![];
    let mut right = vec![];
    for _ in 0..m {
        left.push(input.i64());
        right.push(input.i64() + 1);
    }
    let res = solve_case(n, &left, &right);
    out.println(res);
}

fn solve_case(n: i64, left: &Vec<i64>, right: &Vec<i64>) -> i64 {
    let mut all_positions = left.clone();
    all_positions.extend(right.iter());
    all_positions.push(n + 1);
    let mut start = Instant::now();
    {
        let mut x = 1;
        while x <= n {
            all_positions.push(x);
            let next = binary_search_first_true(x + 1..n + 1, |next| n / next != n / x);
            x = next;
        }
    }
    all_positions.sort();
    all_positions.dedup();
    let mut sum = vec![0; all_positions.len()];
    for i in 0..left.len() {
        let fr = all_positions.binary_search(&left[i]).unwrap();
        let to = all_positions.binary_search(&right[i]).unwrap();
        sum[fr] += 1;
        sum[to] -= 1;
    }
    let mut ss = 0;
    let mut heap = vec![];
    let mut exist_non_zero = false;
    for i in 0..all_positions.len() - 1 {
        ss += sum[i];
        if ss != 0 {
            let fr = all_positions[i];
            let to = all_positions[i + 1];
            assert_eq!(n / fr, n / (to - 1));
            heap.push(Node {
                weight: ss * (n / fr),
                cnt_items: to - fr,
            })
        } else {
            exist_non_zero = true;
        }
    }
    if n == 1 {
        return sum[0];
    }
    if exist_non_zero {
        heap.push(Node {
            weight: 0,
            cnt_items: 1,
        })
    }
    let mut heap = MyQueue::new(heap);
    let mut res = 0;
    while let Some(top) = heap.pop() {
        if top.cnt_items > 1 {
            if top.cnt_items % 2 == 1 {
                heap.return_back(Node {
                    weight: top.weight,
                    cnt_items: 1,
                });
            }
            let half = top.cnt_items / 2;
            res += top.weight * half * 2;
            heap.push(Node {
                weight: top.weight * 2,
                cnt_items: half,
            });
        } else {
            if let Some(top2) = heap.pop() {
                if top2.cnt_items > 1 {
                    heap.return_back(Node {
                        weight: top2.weight,
                        cnt_items: top2.cnt_items - 1,
                    })
                }
                res += top.weight + top2.weight;
                heap.push(Node {
                    weight: top.weight + top2.weight,
                    cnt_items: 1,
                })
            } else {
                break;
            }
        }
    }
    res
}

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let start = Instant::now();
        // const N: i64 = 1_00000_00000;
        // const M: usize = 100_000;
        let n = rnd.gen_range(1..1000);
        let m = rnd.gen_range(1..1000);
        let mut left = vec![];
        let mut right = vec![];
        for _ in 0..m {
            let l = 1 + (rnd.gen_u64() % (n as u64)) as i64;
            let r = 1 + (rnd.gen_u64() % (n as u64)) as i64;
            if l < r {
                left.push(l);
                right.push(r + 1);
            } else {
                left.push(r);
                right.push(l + 1);
            }
        }
        let res = solve_case(n, &left, &right);
        dbg!(res, start.elapsed());
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "l_logging_compression";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
