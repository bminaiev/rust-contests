//{"name":"E. Не спасаться","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 3 3\n5 17 8 1 4\n1 3 3 3 4\n3 1 5 2 5\n3 2 5 1 6\n6 3 3\n5 17 8 1 4 2\n1 3 3 3 4\n3 1 5 2 5\n3 2 5 1 6\n5 3 1\n5 17 8 1 4\n1 3 5 3 100\n5 5 5\n3 2 3 7 5\n3 5 4 2 1\n2 2 5 4 5\n4 4 5 2 3\n1 2 4 2 2\n3 3 5 2 4\n","output":"16\nNO ESCAPE\n-90\n27\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENeSpasatsya"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::graph::dijkstra::dijkstra;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};
use std::collections::HashMap;

type Point = PointT<usize>;

#[derive(Clone)]
struct Edge {
    from: Point,
    to: Point,
    bonus: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let rows = input.usize();
    let cols = input.usize();
    let edges_num = input.usize();
    let mut edges_by_row = vec![vec![]; rows];

    let mut pts_by_row = vec![vec![]; rows];
    let start = Point::new(0, 0);
    let finish = Point::new(rows - 1, cols - 1);
    pts_by_row[start.x].push(start);
    pts_by_row[finish.x].push(finish);


    let rem_by_row = input.read_vec::<i64>(rows);

    for _ in 0..edges_num {
        let from = Point::new(input.usize() - 1, input.usize() - 1);
        let to = Point::new(input.usize() - 1, input.usize() - 1);
        let bonus = input.i64();
        edges_by_row[from.x].push(Edge { from, to, bonus });
        pts_by_row[from.x].push(from);
        pts_by_row[to.x].push(to);
    }
    for row in pts_by_row.iter_mut() {
        row.sort_by_key(|p| p.y);
        row.dedup_by_key(|p| p.y);
    }

    let mut dist: HashMap<Point, i64> = HashMap::new();
    dist.insert(start, 0);

    for row in 0..rows {
        let n = pts_by_row[row].len() + 1;
        let mut graph = SimpleGraphT::<WeightedEdge<i64>>::new(n);
        for (id, w) in pts_by_row[row].windows(2).enumerate() {
            let dist = (w[1].y - w[0].y) as i64 * rem_by_row[row];
            graph.add_edge(id, WeightedEdge::new(id + 1, dist));
            graph.add_edge(id + 1, WeightedEdge::new(id, dist));
        }
        for (id, p) in pts_by_row[row].iter().enumerate() {
            if let Some(&d) = dist.get(p) {
                graph.add_edge(n - 1, WeightedEdge::new(id, d));
            }
        }
        let res = dijkstra(&graph, n - 1);
        for i in 0..n - 1 {
            let vertex = res[i];
            if vertex.dist != i64::MAX {
                dist.insert(pts_by_row[row][i], vertex.dist);
            }
        }
        for edge in edges_by_row[row].iter() {
            if let Some(&cur_dist) = dist.get(&edge.from) {
                let next_dist = cur_dist - edge.bonus;
                dist.entry(edge.to)
                    .or_insert(i64::MAX)
                    .update_min(next_dist);
            }
        }
    }
    if let Some(&res) = dist.get(&finish) {
        out_line!(res);
    } else {
        out_line!("NO ESCAPE");
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
