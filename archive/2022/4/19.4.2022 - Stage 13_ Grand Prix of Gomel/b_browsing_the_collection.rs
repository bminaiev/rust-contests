//{"name":"B. Browsing The Collection","group":"Yandex - Stage 13: Grand Prix of Gomel","url":"https://official.contest.yandex.com/opencupXXII/contest/35270/problems/B/","interactive":false,"timeLimit":4000,"tests":[{"input":"9 3\n5 3 7\n5 3 4\n5 3 7\n5 3 2\n5 3 4\n5 3 7\n2 3 7\n5 3 7\n2 3 7\n","output":"0 1 2 1 2 3 1 2 1\n1 0 1 1 2 2 1 3 2\n2 1 0 1 1 2 1 3 2\n3 2 1 0 1 1 1 3 2\n3 2 2 1 0 1 1 3 2\n3 1 2 1 1 0 1 2 2\n2 1 3 1 2 1 0 1 2\n2 1 3 1 2 2 1 0 1\n1 1 3 1 2 3 2 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBrowsingTheCollection"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::bit_set::BitSet;
use algo_lib::graph::bfs_bitsets::bfs_bitsets;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve_a(a: &Array2D<i32>) {
    let n = a.len();
    let m = a[0].len();
    let state_id = |i: usize, mask: usize| -> usize { i * (1 << m) + mask };
    let graph_size = n * (1 << m);
    // let mut graph = SimpleGraphT::new(graph_size);
    let mut graph = vec![BitSet::new(graph_size); graph_size];
    // let start = Instant::now();
    for i in 0..n {
        for mask in 0..1 << m {
            for bit in 0..m {
                let nmask = mask ^ (1 << bit);
                graph[state_id(i, mask)].set(state_id(i, nmask), true);
            }
            for delta in 1..n {
                let ni = (i + delta) % n;
                let mut ok = true;
                for j in 0..m {
                    if ((1 << j) & mask) != 0 {
                        if a[i][j] != a[ni][j] {
                            ok = false;
                        }
                    }
                }
                if ok {
                    graph[state_id(i, mask)].set(state_id(ni % n, mask), true);
                    graph[state_id(ni % n, mask)].set(state_id(i, mask), true);
                    break;
                }
            }
            let mut seen = BitSet::new(n * m);
            for delta in 1..n {
                let ni = (i + delta) % n;
                let mut ok = true;
                for j in 0..m {
                    if ((1 << j) & mask) != 0 {
                        if a[i][j] != a[ni][j] {
                            ok = false;
                        }
                    }
                }
                if !ok {
                    continue;
                }
                for bit in 0..m {
                    if ((1 << bit) & mask) == 0 && a[i][bit] != a[ni][bit] {
                        let restriction = (a[ni][bit] as usize - 1) * m + bit;
                        if !seen.get(restriction) {
                            seen.set(restriction, true);
                            graph[state_id(i, mask)].set(state_id(ni, mask | (1 << bit)), true);
                        }
                    }
                }
            }
        }
    }
    for i in 0..n {
        let dist = bfs_bitsets(state_id(i, 0), &graph);
        assert!(dist[state_id(i, 0)] == 0);
        for j in 0..n {
            let mut cur_dist = std::u32::MAX;
            for mask in 0..1 << m {
                cur_dist.update_min(dist[state_id(j, mask)]);
            }
            out!(cur_dist, "");
        }
        out_line!();
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let a = input.matrix::<i32>(n, m);
    solve_a(&a);
}

fn stress() {
    let n = 500;
    let m = 5;
    let mut rnd = Random::new(444);
    let mut a = Array2D::new_f(n, m, |_, _| rnd.gen_in_range(0..1000));
    solve_a(&a);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    tester::run_tests();
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
