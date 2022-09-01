//{"name":"B. Puzzle: Patrick’s Parabox","group":"Yandex - Day 6","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39551/problems/B/","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n9 9\n#########\n#####..-#\n#..=##.##\n#.p.##.##\n....##.##\n#...b..##\n#...##.##\n#....####\n####.####\n9 9\n#########\n#.......#\n#.#####.#\n#.#=....#\n..#....-#\n###.p.#.#\n#.....#b#\n#.....#.#\n####.####\n9 9\n####.####\n#....####\n#.####.##\n#.......#\n#.......#\n###.#####\n#=.b#..##\n#-..p..##\n#########\n","output":"7\n4\n19\n"},{"input":"1\n2 2\npb\n-=\n","output":"-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPuzzlePatricksParabox"}}}

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use algo_lib::collections::array_2d::Array2D;
use algo_lib::geometry::point::PointT;
use algo_lib::graph::bfs::bfs01;
use algo_lib::graph::dsu_with_rollbacks::{DsuNodeTrait, DsuWithRollbacks};
use algo_lib::graph::dynamic_connectivity_offline::DynamicConnectivityOffline;
use algo_lib::graph::edges::weighted_edge::WeightedEdge;
use algo_lib::graph::graph_builder::GraphBuilder;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Edge {
    fr: usize,
    to: usize,
}

impl Edge {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            fr: min(x, y),
            to: max(x, y),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct DsuNode {
    mask: u32,
}

impl DsuNodeTrait for DsuNode {
    fn join(lhs: &Self, rhs: &Self) -> Self {
        DsuNode {
            mask: lhs.mask | rhs.mask,
        }
    }
}

fn solve_fast(a: &[Vec<u8>]) -> u32 {
    let n = a.len();
    let m = a[0].len();
    let ok = Array2D::new_f(n, m, |r, c| a[r][c] != b'#');

    let shifts = SHIFTS_4;

    let mid_points = [
        Point::new(n as i32 - 1, (m - 1) as i32 / 2),
        Point::new((n - 1) as i32 / 2, 0),
        Point::new(0, (m - 1) as i32 / 2),
        Point::new((n - 1) as i32 / 2, m as i32 - 1),
    ];

    let find_pos = |c: u8| -> Point {
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == c {
                    return Point::new(i as i32, j as i32);
                }
            }
        }
        unreachable!();
    };

    let pts = Array2D::new_f(n, m, |r, c| Point::new(r as i32, c as i32));
    let mut all_ok_cells = vec![];
    let mut id_in_ok_cells = Array2D::new(std::usize::MAX, n, m);
    for i in 0..n {
        for j in 0..m {
            if ok[i][j] {
                id_in_ok_cells[i][j] = all_ok_cells.len();
                all_ok_cells.push(pts[i][j]);
            }
        }
    }

    let get_pt_id = |p: Point| match p.index_arr2d(&id_in_ok_cells) {
        Some(&x) => {
            if x == std::usize::MAX {
                None
            } else {
                Some(x)
            }
        }
        None => None,
    };
    let final_graph_conv_id = |pt_id: usize, shift_id: usize| pt_id * 6 + shift_id;
    let final_graph_conv_pt_id =
        |pt: Point, shift_id: usize| final_graph_conv_id(get_pt_id(pt).unwrap(), shift_id);

    let get_edge_id = |p1: Point, p2: Point| -> Edge {
        let x = get_pt_id(p1).unwrap();
        let y = get_pt_id(p2).unwrap();
        Edge::new(x, y)
    };

    let pl_start_pos_idx = get_pt_id(find_pos(b'p')).unwrap();
    let pl_finish_pos_idx = get_pt_id(find_pos(b'=')).unwrap();
    let box_start_pos = find_pos(b'b');
    let box_finish_pos = find_pos(b'-');

    let mut dynamic_connectivity =
        DynamicConnectivityOffline::<DsuNode, usize>::new(all_ok_cells.len());

    for i in 0..n {
        for j in 0..m {
            if i + 1 < n && ok[i][j] && ok[i + 1][j] {
                let e = get_edge_id(pts[i][j], pts[i + 1][j]);
                dynamic_connectivity.add_edge(e.fr, e.to);
            }
            if j + 1 < m && ok[i][j] && ok[i][j + 1] {
                let e = get_edge_id(pts[i][j], pts[i][j + 1]);
                dynamic_connectivity.add_edge(e.fr, e.to);
            }
        }
    }

    let mut final_graph = GraphBuilder::new(all_ok_cells.len() * 6);

    let mut add_final_edge = |prev_box: Point,
                              next_box: Point,
                              prev_shift: usize,
                              next_shift: usize,
                              bidirectional: bool| {
        let fr = final_graph_conv_pt_id(prev_box, prev_shift);
        let to = final_graph_conv_pt_id(next_box, next_shift);
        let cost = if prev_box == next_box { 0 } else { 1 };
        final_graph.add_edge(fr, WeightedEdge::new(to, cost));
        if bidirectional {
            final_graph.add_edge(to, WeightedEdge::new(fr, cost));
        }
    };

    for &cur_p in all_ok_cells.iter() {
        let mut rem_edges_here = vec![];
        let mut add_edges_here = vec![];

        for (shift_idx, shift) in shifts.iter().enumerate() {
            let np = cur_p.apply_shift(shift);
            if np.index_arr2d(&ok) == Some(&true) {
                rem_edges_here.push(get_edge_id(cur_p, np));
                let rev_p = cur_p.apply_shift(&shift.rev());
                match rev_p.index_arr2d(&ok) {
                    Some(true) => add_final_edge(cur_p, rev_p, shift_idx, shift_idx, false),
                    Some(false) => {
                        let mid_point = mid_points[shift_idx];
                        if mid_point.index_arr2d(&ok) == Some(&true) && mid_point != cur_p {
                            let e2 = get_edge_id(np, mid_point);
                            add_edges_here.push(e2);
                        }
                    }
                    None => {}
                };
            }
        }

        for e in add_edges_here.iter() {
            dynamic_connectivity.add_edge(e.fr, e.to);
        }
        for e in rem_edges_here.iter() {
            dynamic_connectivity.remove_edge(e.fr, e.to);
        }
        dynamic_connectivity.add_query(get_pt_id(cur_p).unwrap());
        for e in add_edges_here.iter() {
            dynamic_connectivity.remove_edge(e.fr, e.to);
        }
        for e in rem_edges_here.iter() {
            dynamic_connectivity.add_edge(e.fr, e.to);
        }
    }

    let dsu = dynamic_connectivity.get_dsu_mut();

    for &p in all_ok_cells.iter() {
        let mut mask = 0;
        if p.x == n as i32 - 1 {
            mask |= 1;
        }
        if p.y == 0 {
            mask |= 2;
        }
        if p.x == 0 {
            mask |= 4;
        }
        if p.y == m as i32 - 1 {
            mask |= 8;
        }
        dsu.set_node(get_pt_id(p).unwrap(), DsuNode { mask });
    }

    dynamic_connectivity.run(&mut |&pt_id, dsu: &DsuWithRollbacks<DsuNode>| {
        let p = all_ok_cells[pt_id];

        let mut dsu_ids = vec![std::usize::MAX; 6];

        for (idx, shift) in shifts.iter().enumerate() {
            let np = p.apply_shift(shift);
            if let Some(np) = get_pt_id(np) {
                dsu_ids[idx] = dsu.get(np);
            }
        }
        dsu_ids[4] = dsu.get(pl_start_pos_idx);
        dsu_ids[5] = dsu.get(pl_finish_pos_idx);

        for i in 0..dsu_ids.len() {
            for j in i + 1..dsu_ids.len() {
                if dsu_ids[i] == dsu_ids[j] {
                    add_final_edge(p, p, i, j, true);
                    break;
                }
            }
        }

        for s_id in 0..dsu_ids.len() {
            if dsu_ids[s_id] != std::usize::MAX {
                let mask = dsu.get_node(dsu_ids[s_id]).mask;
                for shift_it in 0..SHIFTS_4.len() {
                    if ((1 << shift_it) & mask) != 0 {
                        let np = p.apply_shift(&SHIFTS_4[shift_it]);
                        if np.index_arr2d(&ok) == Some(&true) {
                            add_final_edge(p, p, s_id, shift_it, false);
                        }
                    }
                }
            }
        }
    });

    let final_graph = final_graph.build();

    let start_idx = final_graph_conv_pt_id(box_start_pos, 4);
    let end_idx = final_graph_conv_pt_id(box_finish_pos, 5);

    bfs01(start_idx, &final_graph).dist[end_idx]
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let _m = input.usize();
    let a = gen_vec(n, |_| input.string());
    let res = solve_fast(&a);
    if res == std::u32::MAX {
        out_line!(-1);
    } else {
        out_line!(res);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct State {
    player_pos: Point,
    box_pos: Point,
}

fn solve_slow(a: &[Vec<u8>]) -> u32 {
    let n = a.len();
    let m = a[0].len();
    let ok = Array2D::new_f(n, m, |r, c| a[r][c] != b'#');

    let mid_points = [
        Point::new(n as i32 - 1, (m - 1) as i32 / 2),
        Point::new((n - 1) as i32 / 2, 0),
        Point::new(0, (m - 1) as i32 / 2),
        Point::new((n - 1) as i32 / 2, m as i32 - 1),
    ];

    let find_pos = |c: u8| -> Point {
        for i in 0..n {
            for j in 0..m {
                if a[i][j] == c {
                    return Point::new(i as i32, j as i32);
                }
            }
        }
        unreachable!();
    };

    let start_state = State {
        player_pos: find_pos(b'p'),
        box_pos: find_pos(b'b'),
    };

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();
    queue.push_back(start_state);
    dist.insert(start_state, 0);

    while let Some(state) = queue.pop_front() {
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);
        let pl = state.player_pos;
        let bo = state.box_pos;

        let cur_dist = *dist.get(&state).unwrap();

        let mut add_to_queue = |next_state: State, ndist: u32| {
            if *dist.get(&next_state).unwrap_or(&std::u32::MAX) > ndist {
                dist.insert(next_state, ndist);
                if cur_dist == ndist {
                    queue.push_front(next_state);
                } else {
                    queue.push_back(next_state);
                }
            }
        };

        for shift in SHIFTS_4.iter() {
            let np = pl.apply_shift(shift);
            if np == bo && bo.apply_shift(shift).index_arr2d(&ok) == Some(&true) {
                let next_state = State {
                    player_pos: bo,
                    box_pos: bo.apply_shift(shift),
                };
                add_to_queue(next_state, cur_dist + 1);
            }
            if np != bo && np.index_arr2d(&ok) == Some(&true) {
                let next_state = State {
                    player_pos: np,
                    box_pos: bo,
                };
                add_to_queue(next_state, cur_dist);
            }
            let s_idx_rev = SHIFTS_4.iter().position(|e| *e == shift.rev()).unwrap();
            if np == bo
                && bo.apply_shift(shift).index_arr2d(&ok) == Some(&false)
                && mid_points[s_idx_rev].index_arr2d(&ok) == Some(&true)
            {
                let next_state = State {
                    player_pos: mid_points[s_idx_rev],
                    box_pos: bo,
                };
                add_to_queue(next_state, cur_dist);
            }
            if np.index_arr2d(&ok).is_none()
                && bo.apply_shift(shift).index_arr2d(&ok) == Some(&true)
            {
                let next_state = State {
                    player_pos: bo.apply_shift(shift),
                    box_pos: bo,
                };
                add_to_queue(next_state, cur_dist);
            }
        }
    }

    let finish_state = State {
        player_pos: find_pos(b'='),
        box_pos: find_pos(b'-'),
    };

    match dist.get(&finish_state) {
        Some(x) => *x,
        None => std::u32::MAX,
    }
}

fn stress() {
    for it in 5481.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_N: usize = 30;
        let n = rnd.gen(1..MAX_N);
        let m = rnd.gen(1..MAX_N);
        if n * m < 4 {
            continue;
        }
        let mut a = gen_vec(n, |_| {
            gen_vec(m, |_| if rnd.gen_bool() { b'#' } else { b'.' })
        });
        for &to_gen in [b'p', b'b', b'-', b'='].iter() {
            loop {
                let x = rnd.gen(0..n);
                let y = rnd.gen(0..m);
                if a[x][y] == b'.' || a[x][y] == b'#' {
                    a[x][y] = to_gen;
                    break;
                }
            }
        }
        let fast = solve_fast(&a);
        let slow = solve_slow(&a);
        if fast != slow {
            dbg!(slow);
            dbg!(fast);
            for i in 0..n {
                dbg!(vec2str(&a[i]));
            }
            assert!(false);
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
    // tester::run_single_test("4");
    // tester::run_stress(stress);
}
//END MAIN
