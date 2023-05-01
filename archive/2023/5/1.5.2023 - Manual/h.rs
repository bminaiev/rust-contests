//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

use std::cmp::min;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_case(
    mut all_edges: Vec<(usize, usize)>,
    costs: &[i32],
    base_set: Vec<usize>,
    g: &[Vec<usize>],
) -> ResState {
    let n = costs.len();
    let mut color = vec![0; n];
    let mut leafs_order = vec![];
    RecursiveFunction::new(|f, v: usize| {
        color[v] = 1;
        let mut cnt_child = 0;
        for &to in g[v].iter() {
            if color[to] == 0 {
                f.call(to);
                cnt_child += 1;
            }
        }
        color[v] = 2;
        if cnt_child == 0 || (cnt_child == 1 && v == 0) {
            leafs_order.push(v);
        }
    })
    .call(0);

    for i in 0..leafs_order.len() {
        let v = leafs_order[i];
        let u = leafs_order[(i + 1) % leafs_order.len()];
        all_edges.push((v, u));
    }

    // for &(x, y) in all_edges.iter() {
    //     eprintln!("{x} {y}");
    // }

    // for i in 0..n {
    //     for &j in g[i].iter() {
    //         if j > i {
    //             eprintln!("{i} {j}");
    //         }
    //     }
    // }

    let mut edges_table = Array2D::new(false, n, n);
    for &(x, y) in all_edges.iter() {
        edges_table[x][y] = true;
        edges_table[y][x] = true;
    }

    let init_dp = |v: usize| {
        let mut frontend = base_set.clone();
        frontend.push(v);
        frontend.sort();
        frontend.dedup();

        let states = (0..1 << frontend.len())
            .map(|mask| {
                let mut used = BitSet::new(n);
                let mut cost = 0;
                for i in 0..frontend.len() {
                    if (1 << i) & mask != 0 {
                        if frontend[i] == v {
                            cost += costs[v];
                        }
                        used.set(frontend[i], true);
                    }
                }
                for i in 0..frontend.len() {
                    for j in i + 1..frontend.len() {
                        if ((1 << i) & mask) != 0 {
                            if ((1 << j) & mask) != 0 {
                                if edges_table[frontend[i]][frontend[j]] {
                                    return ResState {
                                        used,
                                        cost: NEG_INF,
                                    };
                                }
                            }
                        }
                    }
                }
                ResState { used, cost }
            })
            .collect::<Vec<_>>();

        Dp {
            all_vertices: frontend.clone(),
            frontend,
            states,
        }
    };

    const NEG_INF: i32 = std::i32::MIN / 100;

    let join_dp = |left: Dp, right: Dp| -> Dp {
        let mut all_vertices = left.all_vertices.clone();
        all_vertices.extend(right.all_vertices);
        all_vertices.sort();
        all_vertices.dedup();

        let mut frontend = vec![];
        for &(fr, to) in all_edges.iter() {
            if all_vertices.binary_search(&fr).is_ok() && !all_vertices.binary_search(&to).is_ok() {
                frontend.push(fr);
            }
            if all_vertices.binary_search(&to).is_ok() && !all_vertices.binary_search(&fr).is_ok() {
                frontend.push(to);
            }
        }
        frontend.extend(&base_set);
        frontend.sort();
        frontend.dedup();

        // dbg!(all_vertices);
        // if frontend.len() > 16 {
        //     for &(x, y) in all_edges.iter() {
        //         if x == 13 || y == 13 {
        //             dbg!(x, y);
        //         }
        //     }
        // }
        assert!(frontend.len() <= 16, "fron = {:?}", frontend);

        let mut states = vec![
            ResState {
                used: BitSet::new(n),
                cost: NEG_INF,
            };
            1 << frontend.len()
        ];

        for mask in 0..1 << left.frontend.len() {
            if left.states[mask].cost < 0 {
                continue;
            }

            let mut known_right_mask = 0;
            let mut right_mask = 0;
            for i in 0..right.frontend.len() {
                if let Ok(pos) = left.frontend.binary_search(&right.frontend[i]) {
                    known_right_mask |= 1 << i;
                    if (1 << pos) & mask != 0 {
                        right_mask |= 1 << i;
                    }
                } else {
                    for j in 0..left.frontend.len() {
                        if ((1 << j) & mask) != 0 {
                            if edges_table[left.frontend[j]][right.frontend[i]] {
                                known_right_mask |= 1 << i;
                                break;
                            }
                        }
                    }
                }
            }
            let not_known_mask: usize = (1 << right.frontend.len()) - 1 - known_right_mask;

            assert!(not_known_mask.count_ones() + left.frontend.len() as u32 <= 20);

            {
                let mut add_right_mask = not_known_mask;
                loop {
                    let right_mask = right_mask | add_right_mask;

                    let r1 = &left.states[mask];
                    let r2 = &right.states[right_mask];
                    if r2.cost >= 0 {
                        let mut used = r1.used.clone();
                        used |= &r2.used;
                        let new_state = ResState {
                            used,
                            cost: r1.cost + r2.cost,
                        };
                        {
                            let mut final_mask = 0;
                            for i in 0..frontend.len() {
                                if let Ok(pos) = left.frontend.binary_search(&frontend[i]) {
                                    if (1 << pos) & mask != 0 {
                                        final_mask |= 1 << i;
                                    }
                                } else if let Ok(pos) = right.frontend.binary_search(&frontend[i]) {
                                    if (1 << pos) & right_mask != 0 {
                                        final_mask |= 1 << i;
                                    }
                                } else {
                                    assert!(false);
                                }
                            }
                            if states[final_mask].cost < new_state.cost {
                                states[final_mask] = new_state;
                            }
                        }
                    }

                    if add_right_mask == 0 {
                        break;
                    }
                    add_right_mask = (add_right_mask - 1) & not_known_mask;
                }
            }
        }

        Dp {
            all_vertices,
            frontend,
            states,
        }
    };

    let mut color = vec![0; n];
    let root_dp = RecursiveFunction::new(|f, v: usize| -> Dp {
        color[v] = 1;

        let mut cur_dp = init_dp(v);

        for &to in g[v].iter() {
            if color[to] == 0 {
                let child_dp = f.call(to);
                // dbg!("joiing", v, to);
                cur_dp = join_dp(cur_dp, child_dp);
            }
        }
        color[v] = 2;

        cur_dp
    })
    .call(0);

    root_dp
        .states
        .iter()
        .max_by_key(|state| state.cost)
        .unwrap()
        .clone()
}

fn stress() {
    for it in 81.. {
        dbg!(it);
        let mut rng = Random::new(787788 + it);
        let n = rng.next_in_range(1, 500);
        let mut g = vec![vec![]; n];
        let mut all_edges = vec![];
        for i in 1..n {
            let prev = rng.next_in_range(0, i);
            g[prev].push(i);
            g[i].push(prev);
            all_edges.push((i, prev));
        }
        let costs = (0..n).map(|_| rng.gen(1..100)).collect::<Vec<_>>();
        let base_mx = min(10, n);
        let base_set: Vec<_> = (0..base_mx).collect();
        for _ in 0..100 {
            let x = rng.gen(0..base_mx);
            let y = rng.gen(0..n);
            all_edges.push((x, y));
        }
        solve_case(all_edges, &costs, base_set, &g);
        // assert!(false);
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let costs = input.vec::<i32>(n);
    let mut g = vec![vec![]; n];
    let mut all_edges = vec![];
    for _ in 0..m {
        let fr = input.usize();
        let to = input.usize();
        g[fr].push(to);
        g[to].push(fr);
        all_edges.push((fr, to));
    }

    let k = input.usize();
    let mut seen = vec![0; n];
    let mut base_set = vec![];
    for _ in 0..k {
        let fr = input.usize();
        let to = input.usize();
        all_edges.push((fr, to));
        seen[fr] += 1;
        if seen[fr] == 2 {
            base_set.push(fr);
        }
        seen[to] += 1;
        if seen[to] == 2 {
            base_set.push(to);
        }
    }
    assert!(base_set.len() <= 12);

    let best = solve_case(all_edges, &costs, base_set, &g);

    out_line!(best.cost, best.used.count_ones());
    for i in 0..n {
        if best.used.get(i) {
            out!(i, "");
        }
    }
}

struct Dp {
    all_vertices: Vec<usize>,
    frontend: Vec<usize>,
    states: Vec<ResState>,
}

#[derive(Clone)]
struct ResState {
    used: BitSet,
    cost: i32,
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
