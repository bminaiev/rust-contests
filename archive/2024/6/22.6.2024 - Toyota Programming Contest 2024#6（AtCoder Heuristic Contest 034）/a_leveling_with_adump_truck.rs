//{"name":"A - Leveling with a Dump Truck","group":"AtCoder - Toyota Programming Contest 2024#6（AtCoder Heuristic Contest 034）","url":"https://atcoder.jp/contests/ahc034/tasks/ahc034_a","interactive":false,"timeLimit":2000,"tests":[{"input":"20\n10 4 0 0 4 10 16 20 21 16 11 2 -7 -16 -23 -25 -23 -16 -7 2\n16 10 6 5 7 10 13 15 14 10 4 -4 -13 -21 -29 -32 -29 -22 -13 -3\n20 14 10 8 9 10 11 12 10 6 0 -7 -15 -24 -31 -33 -31 -25 -16 -8\n20 15 12 10 10 10 10 11 8 5 0 -5 -12 -20 -26 -29 -28 -23 -15 -7\n16 13 12 10 10 10 10 10 9 7 4 1 -4 -10 -16 -20 -20 -17 -10 -3\n10 10 10 10 11 10 10 10 10 10 10 9 7 2 -3 -8 -9 -8 -3 3\n4 7 9 10 10 10 10 10 11 14 16 18 17 14 9 5 1 1 4 9\n0 5 8 10 10 10 10 10 12 15 20 24 25 24 19 14 9 7 9 13\n0 6 10 12 11 11 9 9 10 14 20 26 29 28 24 18 13 10 10 14\n4 10 14 16 13 10 7 5 6 10 16 23 26 26 23 16 10 8 6 10\n11 16 20 21 16 10 4 0 0 4 10 16 21 20 16 10 4 0 1 4\n16 23 26 24 18 9 1 -4 -7 -4 2 9 12 12 8 3 -3 -6 -7 -3\n20 26 27 24 17 7 -3 -11 -14 -12 -7 -1 2 2 -1 -5 -10 -12 -11 -7\n20 24 24 20 13 3 -8 -17 -21 -20 -16 -12 -9 -9 -11 -14 -15 -15 -12 -6\n16 18 17 14 6 -3 -13 -20 -24 -25 -23 -21 -20 -19 -21 -21 -18 -15 -10 -3\n10 10 8 4 -1 -8 -14 -20 -23 -25 -25 -26 -27 -28 -28 -25 -21 -14 -5 3\n4 2 -1 -4 -7 -9 -12 -15 -18 -19 -23 -26 -29 -31 -31 -28 -21 -11 -1 8\n0 -4 -7 -9 -9 -8 -7 -7 -8 -11 -16 -20 -26 -30 -31 -28 -20 -8 2 12\n0 -6 -9 -10 -7 -3 1 3 3 -1 -7 -13 -20 -26 -29 -27 -20 -9 2 13\n4 -3 -6 -6 -3 3 9 12 12 9 3 -5 -12 -20 -25 -26 -21 -12 -1 9\n","output":"+10\nR\n+4\nR\nR\nR\nR\nR\nR\nR\n+50\nR\nR\nR\nR\n-7\nD\n-13\nL\n-44\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALevelingWithADumpTruck"}}}

use std::io::Write;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

const BASE_COST: i32 = 100;
type Point = PointT<usize>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Upload(i32),
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

impl Operation {
    fn new(from: Point, to: Point) -> Self {
        if from.x == to.x {
            if from.y < to.y {
                Operation::MoveRight
            } else {
                Operation::MoveLeft
            }
        } else if from.x < to.x {
            Operation::MoveDown
        } else {
            Operation::MoveUp
        }
    }
}

#[derive(Default, Clone)]
struct Solution {
    cost: i32,
    ops: Vec<Operation>,
}

impl Solution {
    fn apply(&self, a: &mut [i32], start: usize) -> i32 {
        let mut pos = start;
        let mut cost = 0;
        let mut cur_balance = 0;
        for op in &self.ops {
            match op {
                Operation::Upload(x) => {
                    cur_balance += x;
                    cost += x.abs();
                    a[pos] -= x;
                    assert!(cur_balance >= 0);
                }
                Operation::MoveLeft => {
                    pos -= 1;
                    cost += cur_balance + BASE_COST;
                }
                Operation::MoveRight => {
                    cost += cur_balance + BASE_COST;
                    pos += 1;
                }
                Operation::MoveUp | Operation::MoveDown => {
                    unreachable!();
                }
            }
        }
        cost
    }

    fn check(&self, a: &[i32], start: usize) {
        let mut a = a.to_vec();
        let cost = self.apply(&mut a, start);
        assert_eq!(cost, self.cost);
        for x in a {
            assert_eq!(x, 0);
        }
    }

    fn convert(&self, mut pos: usize, positions: &[Point]) -> Self {
        let mut new_ops = vec![];
        for op in &self.ops {
            match op {
                Operation::Upload(x) => {
                    new_ops.push(Operation::Upload(*x));
                }
                Operation::MoveLeft => {
                    pos -= 1;
                    new_ops.push(Operation::new(positions[pos + 1], positions[pos]));
                }
                Operation::MoveRight => {
                    pos += 1;
                    new_ops.push(Operation::new(positions[pos - 1], positions[pos]));
                }
                Operation::MoveUp | Operation::MoveDown => {
                    unreachable!();
                }
            }
        }
        Solution {
            cost: self.cost,
            ops: new_ops,
        }
    }

    fn print(&self, out: &mut Output) {
        for &op in self.ops.iter() {
            match op {
                Operation::Upload(x) => {
                    out.println(format!("{}{}", if x >= 0 { "+" } else { "" }, x));
                }
                Operation::MoveLeft => {
                    out.println("L");
                }
                Operation::MoveRight => {
                    out.println("R");
                }
                Operation::MoveUp => {
                    out.println("U");
                }
                Operation::MoveDown => {
                    out.println("D");
                }
            }
        }
    }

    fn pop_unused_ops(&mut self) {
        while !self.ops.is_empty() {
            let last = self.ops.last().unwrap();
            if matches!(last, Operation::Upload(_)) {
                break;
            }
            self.ops.pop();
            self.cost -= BASE_COST;
        }
    }

    fn get_final_pos(&self, start: usize) -> usize {
        let mut pos = start;
        for op in &self.ops {
            match op {
                Operation::MoveLeft => {
                    pos -= 1;
                }
                Operation::MoveRight => {
                    pos += 1;
                }
                Operation::Upload(_) => {}
                Operation::MoveUp | Operation::MoveDown => {
                    unreachable!();
                }
            }
        }
        pos
    }
}

fn solve_1d_left_right(a: &[i32], start: usize, pop_unused: bool) -> Solution {
    let mut sol = Solution::default();
    // delta[i] = how much we need move from a[i] to a[i + 1]
    let mut deltas = vec![0; a.len() - 1];
    let mut sum = 0;
    for i in 0..a.len() - 1 {
        sum += a[i];
        deltas[i] = sum;
    }
    sum += a[a.len() - 1];
    assert_eq!(sum, 0);
    let mut add_op = |op: Operation, cur_balance: i32| {
        if op == Operation::Upload(0) {
            return;
        }
        sol.ops.push(op);
        match op {
            Operation::Upload(x) => {
                sol.cost += x.abs();
            }
            Operation::MoveLeft | Operation::MoveRight => {
                sol.cost += cur_balance + BASE_COST;
            }
            Operation::MoveUp | Operation::MoveDown => {
                unreachable!();
            }
        }
    };
    let mut cur_balance = 0;
    {
        // go left
        for i in (1..=start).rev() {
            let need = (-deltas[i - 1]).max(0);
            add_op(Operation::Upload(need - cur_balance), cur_balance);
            cur_balance = need;
            add_op(Operation::MoveLeft, cur_balance);
        }
    }
    {
        // go right
        for i in 0..a.len() - 1 {
            let need = deltas[i].max(0);
            add_op(Operation::Upload(need - cur_balance), cur_balance);
            cur_balance = need;
            add_op(Operation::MoveRight, cur_balance);
        }
    }
    {
        // go left to start
        for i in (start + 1..a.len()).rev() {
            let need = (-deltas[i - 1]).max(0);
            add_op(Operation::Upload(need - cur_balance), cur_balance);
            cur_balance = need;
            add_op(Operation::MoveLeft, cur_balance);
        }
    }
    {
        add_op(Operation::Upload(-cur_balance), cur_balance);
    }
    if pop_unused {
        sol.pop_unused_ops();
    }
    sol.check(a, start);
    sol
}

fn solve_1d(a: &[i32], start: usize, pop_unused: bool) -> Solution {
    let left_right = solve_1d_left_right(a, start, pop_unused);
    let mut a = a.to_vec();
    a.reverse();
    let start = a.len() - 1 - start;
    let mut right_left = solve_1d_left_right(&a, start, pop_unused);
    if right_left.cost < left_right.cost {
        for op in &mut right_left.ops {
            match op {
                Operation::MoveLeft => {
                    *op = Operation::MoveRight;
                }
                Operation::MoveRight => {
                    *op = Operation::MoveLeft;
                }
                _ => {}
            }
        }
        right_left
    } else {
        left_right
    }
}

fn solve_from_col(mut a: Array2D<i32>, mut start: usize) -> Solution {
    let n = a.rows();
    let mut sol = Solution::default();
    for _i in 0..start {
        sol.ops.push(Operation::MoveRight);
        sol.cost += BASE_COST;
    }
    let mut rows = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            rows[i] += a[i][j];
        }
    }
    let sol_rows = solve_1d(&rows, 0, false);
    let positions: Vec<Point> = (0..n).map(|i| Point::new(i, start)).collect();
    let mut sol = sol_rows.convert(0, &positions);
    {
        let mut base = vec![0; n];
        sol_rows.apply(&mut base, 0);
        for i in 0..n {
            a[i][0] += base[i];
        }
    }
    for i in 0..n {
        let cur_row_sol = solve_1d(&a[i], start, true);
        start = cur_row_sol.get_final_pos(start);
        sol.cost += cur_row_sol.cost;
        sol.ops.extend(cur_row_sol.ops);
        if i + 1 < n {
            sol.cost += BASE_COST;
            sol.ops.push(Operation::MoveDown);
        }
    }
    sol
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) -> i64 {
    let n = input.usize();
    let mut a = Array2D::new_f(n, n, |_, _| input.i32());
    let mut base_score = 0;
    for i in 0..n {
        for j in 0..n {
            base_score += a[i][j].abs();
        }
    }
    let mut best_sol = Solution::default();
    best_sol.cost = i32::MAX;
    for start in 0..n {
        let sol = solve_from_col(a.clone(), start);
        if sol.cost < best_sol.cost {
            best_sol = sol;
        }
    }
    let cnt_moves = best_sol
        .ops
        .iter()
        .filter(|&&op| !matches!(op, Operation::Upload(_)))
        .count();
    dbg!(best_sol.cost, cnt_moves as i32 * BASE_COST);
    // dbg!();
    best_sol.print(out);
    base_score as i64 * 1_000_000_000 / best_sol.cost as i64
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn stress() {
    const PROBLEM_NAME: &str = "a_leveling_with_adump_truck";

    let mut sum_cost = 0;
    for seed in 0..1 {
        let mut input = Input::new_file(format!("{}/tests/{:04}.txt", PROBLEM_NAME, seed));
        let mut out = Output::new_file("a_leveling_with_adump_truck/output.txt");
        sum_cost += solve(&mut input, &mut out, 0) as i64;
        out.flush();
    }
    dbg!(sum_cost);
}

fn main() {
    const PROBLEM_NAME: &str = "a_leveling_with_adump_truck";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
