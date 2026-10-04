#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::frac::FracT;
use algo_lib::misc::ord_f64::OrdF64;

#[derive(Clone, Copy)]
struct Item {
    q: i64,
    pos: i64,
    group: usize,
}

// ax + b
#[derive(Clone, Copy)]
struct Line {
    a: f64,
    b: f64,
    pt_id: usize,
    line_id: usize,
}

#[derive(Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    value: f64,
}

const INF_EDGE: Edge = Edge {
    from: usize::MAX,
    to: usize::MAX,
    value: f64::NEG_INFINITY,
};

const EPS: f64 = 1e-9;

fn when_better(line1: &Line, line2: &Line) -> f64 {
    assert!(line2.a >= line1.a);
    if (line1.a - line2.a).abs() < EPS {
        if line1.b > line2.b {
            f64::INFINITY
        } else {
            f64::NEG_INFINITY
        }
    } else {
        (line2.b - line1.b) / (line1.a - line2.a)
    }
}

struct ConvexHull {
    lines: Vec<Line>,
    lines_it: usize,
}

impl ConvexHull {
    fn new(all_lines: &[Line]) -> Self {
        let mut hull = vec![];
        for line in all_lines.iter() {
            while hull.len() >= 2 {
                let last = hull[hull.len() - 1];
                let last2 = hull[hull.len() - 2];
                if when_better(&last2, &last) < when_better(&last2, line) {
                    break;
                }
                hull.pop();
            }
            if hull.len() == 1 && when_better(&hull[0], line) == f64::NEG_INFINITY {
                hull.pop();
            }
            hull.push(*line);
        }
        Self {
            lines: hull,
            lines_it: 0,
        }
    }

    fn go_to(&mut self, x: f64) {
        while self.lines_it + 1 < self.lines.len()
            && when_better(&self.lines[self.lines_it], &self.lines[self.lines_it + 1]) < x
        {
            self.lines_it += 1;
        }
    }
}

fn solve_case(mut a: Vec<Item>) -> FracT<i64> {
    let n = a.len();

    let mut at_least_on_group = false;
    for v in 0..n {
        if a[v].group != 2 {
            at_least_on_group = true;
        }
    }
    if !at_least_on_group {
        a[0].group = 0;
    }

    let mut lines = vec![];
    for id in 0..n {
        let q = a[id].q as f64;
        lines.push(Line {
            a: 1.0 / q.sqrt(),
            b: -a[id].pos as f64 / q.sqrt(),
            pt_id: id,
            line_id: 0,
        });
        lines.push(Line {
            a: -1.0 / q.sqrt(),
            b: a[id].pos as f64 / q.sqrt(),
            pt_id: id,
            line_id: 0,
        });
    }
    lines.sort_by_key(|l| OrdF64(l.a));
    for i in 0..lines.len() {
        lines[i].line_id = i;
    }
    let mut hull = ConvexHull::new(&lines);
    let mut hull_from = vec![0; hull.lines.len()];
    let mut hull_to = vec![hull.lines.len() - 1; hull.lines.len()];

    let mut sorted_by_x: Vec<_> = (0..n).collect();
    sorted_by_x.sort_by_key(|id| a[*id].pos);

    let mut dsu = Dsu::new(n + 2);
    let mut g = vec![vec![]; n + 2];
    for i in 0..n {
        if a[i].group == 0 {
            dsu.unite(i, n);
            g[i].push(n);
            g[n].push(i);
        } else if a[i].group == 1 {
            dsu.unite(i, n + 1);
            g[i].push(n + 1);
            g[n + 1].push(i);
        }
    }
    dsu.unite(n, n + 1);
    g[n].push(n + 1);
    g[n + 1].push(n);
    while dsu.num_components() > 1 {
        let mut best_edges = vec![INF_EDGE; n + 2];
        hull.lines_it = 0;
        for i in 1..hull.lines.len() {
            let color = dsu.get(hull.lines[i].pt_id);
            let prev_color = dsu.get(hull.lines[i - 1].pt_id);
            if color == prev_color {
                hull_from[i] = hull_from[i - 1];
            } else {
                hull_from[i] = i;
            }
        }
        for i in (0..hull.lines.len() - 1).rev() {
            let color = dsu.get(hull.lines[i].pt_id);
            let next_color = dsu.get(hull.lines[i + 1].pt_id);
            if color == next_color {
                hull_to[i] = hull_to[i + 1];
            } else {
                hull_to[i] = i;
            }
        }
        let mut second_hull = ConvexHull::new(&[]);
        let mut second_hull_from = usize::MAX;
        for &id in sorted_by_x.iter() {
            let x = a[id].pos as f64;
            let color = dsu.get(id);
            hull.go_to(x);
            let line = hull.lines[hull.lines_it];
            let line_color = dsu.get(line.pt_id);
            let q_mult = 1.0 / (a[id].q as f64).sqrt();
            let e = if color != line_color {
                Edge {
                    from: id,
                    to: line.pt_id,
                    value: (line.a * x + line.b) * q_mult,
                }
            } else {
                let need_from = hull_from[hull.lines_it];
                if need_from != second_hull_from {
                    second_hull_from = need_from;
                    let from = if need_from == 0 {
                        0
                    } else {
                        hull.lines[need_from - 1].line_id
                    };
                    let second_hull_to = hull_to[need_from];
                    let to = if second_hull_to == hull.lines.len() - 1 {
                        lines.len()
                    } else {
                        hull.lines[second_hull_to + 1].line_id + 1
                    };
                    let mut second_hull_lines = lines[from..to].to_vec();
                    second_hull_lines.retain(|l| dsu.get(l.pt_id) != color);
                    second_hull = ConvexHull::new(&second_hull_lines);
                }
                second_hull.go_to(x);
                let line = second_hull.lines[second_hull.lines_it];
                assert_ne!(dsu.get(line.pt_id), color);
                Edge {
                    from: id,
                    to: line.pt_id,
                    value: (line.a * x + line.b) * q_mult,
                }
            };
            if e.value > best_edges[color].value {
                best_edges[color] = e;
            }
        }
        for e in best_edges.iter() {
            if e.value > f64::NEG_INFINITY {
                if dsu.get(e.from) != dsu.get(e.to) {
                    dsu.unite(e.from, e.to);
                    g[e.from].push(e.to);
                    g[e.to].push(e.from);
                }
            }
        }
    }
    let mut final_colors = vec![usize::MAX; n + 2];
    final_colors[n] = 0;
    let mut st = vec![n];
    while let Some(v) = st.pop() {
        for &to in g[v].iter() {
            if final_colors[to] == usize::MAX {
                final_colors[to] = 1 - final_colors[v];
                st.push(to);
            }
        }
    }
    for v in 0..n + 2 {
        assert_ne!(final_colors[v], usize::MAX);
    }
    let mut hulls: Vec<ConvexHull> = (0..2)
        .map(|color| {
            let mut lines = lines.clone();
            lines.retain(|l| color == final_colors[l.pt_id]);
            ConvexHull::new(&lines)
        })
        .collect();

    let mut res = FracT::new(1e9 as i64, 1);
    for &id in sorted_by_x.iter() {
        let x = a[id].pos as f64;
        let color = final_colors[id];
        let hull = &mut hulls[color];
        hull.go_to(x);
        let line = hull.lines[hull.lines_it];
        let id2 = line.pt_id;
        let num = a[id].q * a[id2].q;
        let denom = (a[id].pos - a[id2].pos).pow(2);
        res = res.min(FracT::new(num, denom));
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _ in 0..n {
            let q = input.i64();
            let pos = input.i64();
            let ss = input.string();
            let group = b"AB?".iter().position(|&c| c == ss[0]).unwrap();
            a.push(Item { q, pos, group });
        }
        let res = solve_case(a).norm();
        out.println(format!("{}/{}", res.num, res.denom));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "h_harmonizing_charges";
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
