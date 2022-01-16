//{"name":"E. Не спасаться","group":"Codeforces - Codeforces Round #766 (Div. 2)","url":"https://codeforces.com/contest/1627/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 3 3\n5 17 8 1 4\n1 3 3 3 4\n3 1 5 2 5\n3 2 5 1 6\n6 3 3\n5 17 8 1 4 2\n1 3 3 3 4\n3 1 5 2 5\n3 2 5 1 6\n5 3 1\n5 17 8 1 4\n1 3 5 3 100\n5 5 5\n3 2 3 7 5\n3 5 4 2 1\n2 2 5 4 5\n4 4 5 2 3\n1 2 4 2 2\n3 3 5 2 4\n","output":"16\nNO ESCAPE\n-90\n27\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENeSpasatsya"}}}

use algo_lib::collections::id_map::IdMap;
use algo_lib::geometry::point::PointT;
use algo_lib::graph::dijkstra::dijkstra;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::weighted_graph::WeightedGraph;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::func::snd;
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let rows_num = input.usize();
    let cols_num = input.usize();
    let edges_num = input.usize();
    let cost_inside_row: Vec<i64> = input.read_vec(rows_num);

    let mut point_ids = IdMap::<Point>::new();

    let start = Point::new(1, 1);
    let finish = Point::new(rows_num as i64, cols_num as i64);
    point_ids.get_or_add(&start);
    point_ids.get_or_add(&finish);

    let mut graph = WeightedGraph::<i64>::new(2);

    let potential = |row: i64| row * (2e6 as i64);

    for _ in 0..edges_num {
        let (from, to) = input.read();
        let bonus = input.i64();
        let from_id = point_ids.get_or_add(&from);
        let to_id = point_ids.get_or_add(&to);
        graph.add_edge_maybe_new_vertices(
            from_id,
            WeightedEdge::new(to_id, potential(to.x) - potential(from.x) - bonus),
        );
    }

    let mut all_points: Vec<Point> = point_ids.iter().map(snd).cloned().collect();
    all_points.sort_by_key(|p| (p.x, p.y));
    for w in all_points.windows(2) {
        if w[0].x == w[1].x {
            let id1 = point_ids.get_exn(&w[0]);
            let id2 = point_ids.get_exn(&w[1]);
            let dist = cost_inside_row[w[0].x as usize - 1] * (w[0].y - w[1].y).abs();
            graph.add_edge(id1, WeightedEdge::new(id2, dist));
            graph.add_edge(id2, WeightedEdge::new(id1, dist));
        }
    }
    let res = dijkstra(&graph, point_ids.get_exn(&start));
    match res[point_ids.get_exn(&finish)].dist {
        i64::MAX => {
            out_line!("NO ESCAPE");
        }
        some_dist => {
            let res = some_dist - potential(finish.x) + potential(start.x);
            out_line!(res);
        }
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
