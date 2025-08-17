//{"name":"midnight-knapsack","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"midnight-knapsack"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::SimulatedAnnealing;

#[derive(Debug, Clone)]
struct Item {
    weight: i64,
    value: i64,
}

fn solve_case_1(test: &TestCase) -> i64 {
    let items = test.items.clone();
    let W = test.w;
    let N = items.len();
    let mut best_v = 0;
    let mut cost = 0;

    // Initially, no item is taken.
    let mut current_solution = vec![false; N];
    let mut best_solution = vec![false; N];

    // Precompute cumulative values: cumulative_values[d] holds the sum of items[d..].value.
    let mut cumulative_values = vec![0; N + 1];
    cumulative_values[N] = 0;
    for i in (0..N).rev() {
        cumulative_values[i] = cumulative_values[i + 1] + items[i].value;
    }

    // Recursive backtracking function.
    // sw: current weight, sv: current value, d: current index (depth)
    fn go(
        sw: i64,
        sv: i64,
        d: usize,
        W: i64,
        N: usize,
        cumulative_values: &Vec<i64>,
        items: &Vec<Item>,
        current_solution: &mut Vec<bool>,
        best_solution: &mut Vec<bool>,
        best_v: &mut i64,
        cost: &mut i64,
    ) {
        *cost += 1;
        // Only proceed if the best possible value (sv plus remaining available value)
        // can beat the current best value.
        if sv + cumulative_values[d] > *best_v {
            if d == N {
                // All items have been considered.
                if sv > *best_v {
                    *best_v = sv;
                    *best_solution = current_solution.clone(); // Save current solution.
                }
            } else {
                // Try taking the current item if it fits.
                if sw + items[d].weight <= W {
                    current_solution[d] = true;
                    go(
                        sw + items[d].weight,
                        sv + items[d].value,
                        d + 1,
                        W,
                        N,
                        cumulative_values,
                        items,
                        current_solution,
                        best_solution,
                        best_v,
                        cost,
                    );
                    // Backtrack: drop the item.
                    current_solution[d] = false;
                }
                // Try not taking the current item.
                go(
                    sw,
                    sv,
                    d + 1,
                    W,
                    N,
                    cumulative_values,
                    items,
                    current_solution,
                    best_solution,
                    best_v,
                    cost,
                );
            }
        }
    }

    // Start recursion with weight and value zero, at depth 0.
    go(
        0,
        0,
        0,
        W,
        N,
        &cumulative_values,
        &items,
        &mut current_solution,
        &mut best_solution,
        &mut best_v,
        &mut cost,
    );

    // Return the number of recursive calls.
    cost
}

fn solve_case_2(test: &TestCase) -> i64 {
    let mut items = test.items.clone();
    let W = test.w;
    let N = items.len();
    let mut best_v = 0;
    let mut cost = 0;
    // current_solution keeps track of the items chosen so far (true = chosen)
    let mut current_solution = vec![false; N];
    // best_solution will save the best combination found (not used later, but stored as in pseudocode)
    let mut best_solution = vec![false; N];

    // Sort items by non-increasing ratio (value/weight).
    // In case of equal ratios, sort by non-increasing weight.
    items.sort_by(|a, b| {
        let r1 = a.value as f64 / a.weight as f64;
        let r2 = b.value as f64 / b.weight as f64;
        // We want descending order: higher ratio comes first.
        r2.partial_cmp(&r1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.weight.cmp(&a.weight))
    });

    // Precompute cumulative weights from index d to the end for use in the heuristic.
    let mut cum_weights = vec![0; N + 1]; // cum_weights[d] = sum(items[d..])
    cum_weights[N] = 0;
    for i in (0..N).rev() {
        cum_weights[i] = cum_weights[i + 1] + items[i].weight;
    }

    // The heuristic function:
    // Given current weight (sw) and value (sv) and starting index d,
    // it estimates whether it is possible to reach a better solution.
    fn heuristic(
        d: usize,
        sw: i64,
        sv: i64,
        W: i64,
        best_v: i64,
        items: &[Item],
        cum_weights: &[i64],
    ) -> bool {
        // Calculate remaining weight: the lesser of the available capacity and
        // the total weight of the remaining items.
        let rw = std::cmp::min(W - sw, cum_weights[d]);
        // This is the extra value needed to beat the current best.
        let rv = best_v + 1 - sv;
        if rw <= 0 {
            return false;
        }
        // Compare the ratio of the current item to the bound implied by remaining weight/value.
        let current_ratio = items[d].value as f64 / items[d].weight as f64;
        let bound_ratio = rv as f64 / rw as f64;
        current_ratio >= bound_ratio
    }

    // The recursive backtracking function.
    // d: current index, sw: current total weight, sv: current total value.
    fn go(
        d: usize,
        sw: i64,
        sv: i64,
        W: i64,
        N: usize,
        items: &[Item],
        cum_weights: &[i64],
        best_v: &mut i64,
        cost: &mut i64,
        current_solution: &mut [bool],
        best_solution: &mut [bool],
    ) {
        *cost += 1;
        if d == N {
            // Reached a complete selection.
            if sv > *best_v {
                *best_v = sv;
                // Save the current solution (this example simply copies the current selection).
                best_solution.copy_from_slice(current_solution);
            }
        } else if sv > *best_v || heuristic(d, sw, sv, W, *best_v, items, cum_weights) {
            // Try picking the current item if it fits.
            if sw + items[d].weight <= W {
                current_solution[d] = true; // pick item d
                go(
                    d + 1,
                    sw + items[d].weight,
                    sv + items[d].value,
                    W,
                    N,
                    items,
                    cum_weights,
                    best_v,
                    cost,
                    current_solution,
                    best_solution,
                );
                current_solution[d] = false; // backtrack: drop item d
            }
            // Also try not picking the current item.
            go(
                d + 1,
                sw,
                sv,
                W,
                N,
                items,
                cum_weights,
                best_v,
                cost,
                current_solution,
                best_solution,
            );
        }
    }

    // Start the recursive search.
    go(
        0,
        0,
        0,
        W,
        N,
        &items,
        &cum_weights,
        &mut best_v,
        &mut cost,
        &mut current_solution,
        &mut best_solution,
    );

    cost
}

#[derive(Clone, Debug)]
struct Node {
    index: usize,
    prev: Option<usize>,
    next: Option<usize>,
}

struct Solver {
    W: i64,                 // Knapsack capacity.
    items: Vec<Item>,       // Sorted list of items.
    knapsack: Vec<bool>,    // Greedy solution: true if picked.
    best_v: i64,            // Best total value found so far.
    cost: i64,              // Counter for the number of recursive calls.
    gw: i64,                // Greedy total weight.
    gv: i64,                // Greedy total value.
    b: usize,               // Index of first item that did not fit in the greedy solution.
    bw: i64,                // Weight of item b.
    bv: i64,                // Value of item b.
    exceptions: Vec<usize>, // List of item indices to toggle.
    nodes: Vec<Node>,       // Doubly linked list of items (by index in `items`).
}

impl Solver {
    // Remove a node from the linked list by updating its neighbors.
    fn remove_node(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;
        if let Some(p) = prev {
            self.nodes[p].next = next;
        }
        if let Some(n) = next {
            self.nodes[n].prev = prev;
        }
        // Mark this node as removed.
        self.nodes[index].prev = None;
        self.nodes[index].next = None;
    }

    // Recursive function `go`, with parameters:
    // - sw: current total weight,
    // - sv: current total value,
    // - left: pointer (as Option<usize>) to the left node,
    // - right: pointer to the right node.
    fn go(&mut self, sw: i64, sv: i64, left: Option<usize>, mut right: Option<usize>) -> bool {
        self.cost += 1;
        let mut improved = false;
        if sw <= self.W {
            if sv > self.best_v {
                self.best_v = sv;
                self.exceptions.clear();
            }
            while let Some(r) = right {
                let item_r = &self.items[r];
                // Check the pruning condition:
                // (best_v + 1 - gv - right.value) * bw > (W - gw - right.weight) * bv
                if (self.best_v + 1 - self.gv - item_r.value) * self.bw
                    > (self.W - self.gw - item_r.weight) * self.bv
                {
                    // Remove node r from the linked list and continue.
                    let next_r = self.nodes[r].next;
                    self.remove_node(r);
                    right = next_r;
                    continue;
                }
                // Check if further recursion is futile.
                if (sv - self.best_v - 1) * item_r.weight < (sw - self.W) * item_r.value {
                    return improved;
                }
                let next_right = self.nodes[r].next;
                if self.go(sw + item_r.weight, sv + item_r.value, left, next_right) {
                    improved = true;
                    self.exceptions.push(r);
                }
                right = self.nodes[r].next;
            }
        } else {
            let mut left_opt = left;
            while let Some(l) = left_opt {
                let item_l = &self.items[l];
                // Pruning condition for the left side:
                if (self.best_v + 1 - self.gv + item_l.value) * self.bw
                    > (self.W - self.gw + item_l.weight) * self.bv
                {
                    let prev_l = self.nodes[l].prev;
                    self.remove_node(l);
                    left_opt = prev_l;
                    continue;
                }
                if (sv - self.best_v - 1) * item_l.weight < (sw - self.W) * item_l.value {
                    return improved;
                }
                let prev_left = self.nodes[l].prev;
                if self.go(sw - item_l.weight, sv - item_l.value, prev_left, right) {
                    improved = true;
                    self.exceptions.push(l);
                }
                left_opt = self.nodes[l].prev;
            }
        }
        improved
    }
}

fn solve_case_3(test: &TestCase) -> i64 {
    let mut items = test.items.clone();
    let W = test.w;

    // 1. Drop (remove) items with zero value.
    items.retain(|item| item.value != 0);

    // 2. Take all items with zero weight (they are automatically picked) and remove them.
    let mut extra_value = 0;
    let mut filtered = Vec::new();
    for item in items {
        if item.weight == 0 {
            extra_value += item.value;
        } else {
            filtered.push(item);
        }
    }
    items = filtered;

    // If no items remain, return 0.
    if items.is_empty() {
        return 0;
    }

    // 3. Sort items by non-increasing (value/weight) ratio,
    //    and in case of a tie, by non-increasing weight.
    items.sort_by(|a, b| {
        let r1 = a.value as f64 / a.weight as f64;
        let r2 = b.value as f64 / b.weight as f64;
        r2.partial_cmp(&r1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.weight.cmp(&a.weight))
    });

    let N = items.len();

    // 4. Greedily pick items (from left to right) as long as they fit.
    let mut knapsack = vec![false; N];
    let mut gw = 0;
    let mut gv = 0;
    let mut b = N; // b will be the index of the first item that did not fit.
    for i in 0..N {
        if gw + items[i].weight <= W {
            gw += items[i].weight;
            gv += items[i].value;
            knapsack[i] = true;
        } else {
            b = i;
            break;
        }
    }
    // If all items fit, the algorithm returns 0.
    if b == N {
        return 0;
    }

    let bw = items[b].weight;
    let bv = items[b].value;

    // 5. Set the initial best value to the greedy value.
    let mut best_v = gv;

    // 6. Examine items after b for possible improvements.
    let mut exceptions: Vec<usize> = Vec::new();
    for i in (b + 1)..N {
        let new_v = gv + items[i].value;
        if new_v > best_v && gw + items[i].weight <= W {
            best_v = new_v;
            exceptions.push(i);
        }
    }
    // 7. Examine swaps: for each item in the greedy set [0, b),
    //    consider replacing it with item b.
    for i in 0..b {
        let new_v = gv + items[b].value - items[i].value;
        if new_v > best_v && gw + items[b].weight - items[i].weight <= W {
            best_v = new_v;
            // Add both items to the exceptions.
            exceptions.push(b);
            exceptions.push(i);
        }
    }

    // 8. Build the linked list for the items.
    let mut nodes = Vec::with_capacity(N);
    for i in 0..N {
        nodes.push(Node {
            index: i,
            prev: if i == 0 { None } else { Some(i - 1) },
            next: if i == N - 1 { None } else { Some(i + 1) },
        });
    }

    // 9. Create a Solver instance holding all state.
    let mut solver = Solver {
        W,
        items,
        knapsack,
        best_v,
        cost: 0,
        gw,
        gv,
        b,
        bw,
        bv,
        exceptions,
        nodes,
    };

    // 10. Call the recursive function `go`.
    // For the left pointer, use the node just before index b (if any).
    let left = if b > 0 { Some(b - 1) } else { None };
    let right = Some(b);
    solver.go(gw, gv, left, right);

    // 11. Toggle the knapsack: for each index in exceptions,
    //     if the item was already picked then drop it, otherwise pick it.
    for i in solver.exceptions {
        solver.knapsack[i] = !solver.knapsack[i];
    }

    // The pseudocode returns the cost (the number of recursive calls).
    solver.cost
}

#[derive(Clone)]
struct TestCase {
    w: i64,
    items: Vec<Item>,
}

impl TestCase {
    fn len(&self) -> usize {
        let mut res = format!("{} ", self.w).len();
        for item in &self.items {
            res += &format!("{} {} ", item.weight, item.value).len();
        }
        res
    }
}

const MAX_LEN: usize = 100;

fn solve(input: &mut Input, out: &mut Output, test_case: usize) {
    let mut best_cost = 0;
    let mut best_tc = TestCase {
        w: 0,
        items: vec![],
    };
    for it in 17090555.. {
        let mut rnd = Random::new(it);
        let w_log = rnd.gen(1..30);
        let w = rnd.gen(1..1 << w_log);
        let v_log = rnd.gen(1..30);
        let v = rnd.gen(1..1 << v_log);
        let mut test = TestCase { w, items: vec![] };
        while test.len() <= MAX_LEN {
            let weight = rnd.gen(1..w + 1);
            let value = rnd.gen(1..v + 1);
            test.items.push(Item { weight, value });
        }
        test.items.pop();
        assert!(test.len() <= MAX_LEN);
        // dbg!(test.len());
        let cost = if test_case == 2 {
            solve_case_2(&test)
        } else {
            solve_case_3(&test)
        };
        let cost_log = (cost as f64).ln();
        if cost > best_cost {
            best_cost = cost;
            best_tc = test.clone();
            dbg!(it, cost, cost_log);
            break;
        }
    }
    let mut sa = SimulatedAnnealing::new(
        1800.0,
        algo_lib::misc::simulated_annealing::SearchFor::MaximumScore,
        10.0,
        1e-3,
        (best_cost as f64).ln(),
    );
    let mut rnd = Random::new(123123);
    let max_w = [0, 100, 100, 1000][test_case];
    let max_v = [0, 100, 100, 20][test_case];
    let max_test_w = [0, 100, 100, 100][test_case];
    while sa.should_continue() {
        let mut test = best_tc.clone();
        if rnd.gen_double() < 0.2 {
            test.w = rnd.gen(1..max_test_w);
        } else {
            let weight = rnd.gen(1..max_w);
            let value = rnd.gen(1..max_v);
            if rnd.gen_double() < 0.1 {
                test.items.push(Item { weight, value });
            } else {
                let pos = rnd.gen(0..test.items.len());
                test.items[pos].weight = weight;
                test.items[pos].value = value;
            }
        }
        if test.len() > MAX_LEN {
            continue;
        }
        let new_score = if test_case == 2 {
            solve_case_2(&test)
        } else {
            solve_case_3(&test)
        };
        let new_score_log = (new_score as f64).ln();
        if sa.should_go(new_score_log) {
            best_tc = test.clone();
        }
    }
    out.println(best_tc.w);
    for item in &best_tc.items {
        out.print(item.weight);
        out.print(" ");
        out.println(item.value);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 2);
    output.flush();
    true
}

fn stress() {
    let tc = 2;
    let mut input = Input::new_file("./tasks/midnight-knapsack/tests/01");
    let mut output = Output::new_file(format!("./tasks/midnight-knapsack/out/{tc:02}.out"));
    solve(&mut input, &mut output, tc);
    output.flush();
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "midnight-knapsack";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
