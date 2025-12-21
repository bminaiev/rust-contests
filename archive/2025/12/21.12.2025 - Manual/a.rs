//{"name":"a","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    let prec = precalc();
    for _ in 0..tc {
        let x = input.usize();
        assert!(x != 2);
        assert!(x != 5);
        if let Some(res) = &prec[x] {
            let g1 = &res.0;
            let graph = if let Some(g2) = &res.1 {
                combine(g1, g2, res.2)
            } else {
                g1.g.clone()
            };
            out.println(graph.len());
            for i in 0..graph.len() - 1 {
                let mut row = String::new();
                for j in i + 1..graph.len() {
                    if graph[i][j] {
                        row.push('1');
                    } else {
                        row.push('0');
                    }
                }
                out.println(row);
            }
        } else {
            unreachable!();
        }
    }
}

#[derive(Clone, Debug)]
struct Graph {
    n: usize,
    my_sum_dist: usize,
    sum_to_0: usize,
    g: Array2D<bool>,
}

fn calc_dists(g: &Array2D<bool>) -> Array2D<usize> {
    let n = g.len();
    let mut d = Array2D::new(usize::MAX / 3, n, n);
    for i in 0..n {
        d[i][i] = 0;
        for j in 0..n {
            if g[i][j] {
                d[i][j] = 1;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }
    d
}

fn calc_sum_dists(d: &Array2D<usize>) -> usize {
    let n = d.len();
    let mut sum_dist = 0;
    for i in 0..n {
        for j in i + 1..n {
            if d[i][j] >= usize::MAX / 3 {
                return usize::MAX;
            }
            sum_dist += d[i][j];
        }
    }
    sum_dist
}

const MAX_TOTAL_N: usize = 85;
const MAX_N: usize = 25;

fn stress123() {
    let extra = calc_extra();
    let mut rnd = Random::new(213);
    for _ in 0..1000 {
        let g1 = gen_graph(&mut rnd);
        let g2 = gen_graph(&mut rnd);
        dbg!(g1);
        let more = rnd.gen_range(1..10);

        let combined_g = combine(&g1, &g2, more);
        let expected = expected_combine(&g1, &g2, more, &extra);
        let real = {
            let d = calc_dists(&combined_g);
            calc_sum_dists(&d)
        };
        dbg!(expected, real);
        assert!(expected == real);
    }
}

fn gen_graph(rnd: &mut Random) -> Graph {
    let n = rnd.gen_range(1..MAX_N);
    let mut g = Array2D::new(false, n, n);
    let p = rnd.gen_double();
    for i in 0..n {
        for j in i + 1..n {
            if rnd.gen_double() < p {
                g[i][j] = true;
                g[j][i] = true;
            }
        }
    }
    let d = calc_dists(&g);

    let mut fail = false;
    for i in 1..n {
        if d[0][i] == usize::MAX / 3 {
            fail = true;
        }
    }
    if fail {
        return gen_graph(rnd);
    }
    let sum_dist = calc_sum_dists(&d);

    let mut sum_to_0 = 0;
    for i in 0..n {
        sum_to_0 += d[0][i];
    }
    Graph {
        n,
        my_sum_dist: sum_dist,
        sum_to_0,
        g,
    }
}

fn calc_extra() -> Vec<usize> {
    let mut extra = vec![0; MAX_TOTAL_N + 1];
    for more in 0..=MAX_TOTAL_N {
        for n1 in 0..more {
            for n2 in n1 + 1..more {
                extra[more] += n2 - n1;
            }
        }
    }
    extra
}

fn precalc() -> Vec<Option<(Graph, Option<Graph>, usize)>> {
    const MAX_ITERS: usize = 3000;

    let mut rnd = Random::new(12365);
    // const MAX_SEEN: usize = 1_000;
    let mut seen = vec![None; 100_001];
    // const MAX_OTHER_SIZE: usize = 100;
    // let mut seen_other = Array2D::new(None, MAX_OTHER_SIZE, MAX_SEEN);

    let mut graphs = vec![];
    let extra = calc_extra();
    let start = Instant::now();
    for _ in 0..MAX_ITERS {
        let g = gen_graph(&mut rnd);
        if seen[g.my_sum_dist].is_none() {
            seen[g.my_sum_dist] = Some((g.clone(), None, 0));
        }
        graphs.push(g);
    }
    // dbg!(start.elapsed());

    for i in 0..graphs.len() {
        for j in i + 1..graphs.len() {
            let n1 = graphs[i].n;
            let n2 = graphs[j].n;
            if n1 + n2 > MAX_TOTAL_N {
                continue;
            }
            for more in 1..=(MAX_TOTAL_N - n1 - n2) {
                let total_n = n1 + n2 + more;
                assert!(total_n <= MAX_TOTAL_N);
                let combined_sum_dist = expected_combine(&graphs[i], &graphs[j], more, &extra);
                // dbg!(combined_sum_dist);

                // let real_g2 = combine(&graphs[i], &graphs[j], more);
                // let real_d = calc_dists(&real_g2);
                // let real_sum_dist = calc_sum_dists(&real_d);
                // dbg!(combined_sum_dist, real_sum_dist);
                // assert!(combined_sum_dist == real_sum_dist);

                if combined_sum_dist < seen.len() {
                    if seen[combined_sum_dist].is_none() {
                        seen[combined_sum_dist] =
                            Some((graphs[i].clone(), Some(graphs[j].clone()), more));
                    }
                } else {
                    break;
                }
            }
        }
    }
    seen
}

fn stress() {
    let seen = precalc();
    let mut sum_fail = 0;
    for i in 0..seen.len() {
        if seen[i].is_none() {
            sum_fail += 1;
            dbg!(i);
        }
    }
    dbg!(sum_fail);
}

fn expected_combine(g1: &Graph, g2: &Graph, more: usize, extra: &[usize]) -> usize {
    g1.my_sum_dist
        + g2.my_sum_dist
        + g1.sum_to_0 * (g2.n + more)
        + g2.sum_to_0 * (g1.n + more)
        + g1.n * g2.n * (more + 1)
        + extra[more]
        + (g1.n + g2.n) * (more * (more + 1) / 2)
}

fn combine(g1: &Graph, g2: &Graph, more: usize) -> Array2D<bool> {
    assert!(more > 0);
    let n = g1.n + g2.n + more;
    let mut g = Array2D::new(false, n, n);
    for i in 0..g1.n {
        for j in 0..g1.n {
            g[i][j] = g1.g[i][j];
        }
    }
    for i in 0..g2.n {
        for j in 0..g2.n {
            g[g1.n + more + i][g1.n + more + j] = g2.g[i][j];
        }
    }
    for i in 0..more {
        g[g1.n + i][g1.n + i + 1] = true;
        g[g1.n + i + 1][g1.n + i] = true;
    }
    g[0][g1.n] = true;
    g[g1.n][0] = true;
    g
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "a";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
