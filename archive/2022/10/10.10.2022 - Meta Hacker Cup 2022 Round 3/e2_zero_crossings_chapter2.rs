//{"name":"E2: Zero Crossings - Chapter 2","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/E2","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n4\n3\n17 13 18 11 16 11\n4\n1 1 1 2 2 2 2 1\n8\n15 8 15 4 12 1 5 1 2 4 2 8 5 11 12 11\n4\n5 4 5 8 12 8 12 4\n1\n7 6 10 6 999\n7\n6\n10 90 20 90 30 80 20 70 10 70 0 80\n5\n10 0 0 50 10 60 100 100 100 0\n4\n30 40 30 60 90 90 90 40\n4\n90 20 90 10 40 10 30 30\n3\n20 10 10 50 30 20\n3\n40 50 40 60 50 50\n3\n60 50 80 70 80 50\n7\n10 80 20 80 1024\n1054 1114 1064 1114 2048\n3112 3162 3092 3152 1024\n3132 3142 3142 3127 9999999\n3092 3092 3112 3092 9999999\n10000993 10001013 10000933 10000993 9999999\n3162 3102 3102 3162 0\n1\n3\n0 4 7 7 3 0\n1\n3 4 2 6 0\n1\n4\n0 4 0 1000000000 7 7 3 0\n1\n3 4 2 6 0\n","output":"Case #1: 1\nCase #2: 4\nCase #3: 0\nCase #4: 1\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"zero_crossings_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"zero_crossings_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"E2ZeroCrossingsChapter2"}}}

use std::cmp::Ordering;
use std::time::Instant;

use algo_lib::collections::reversed::ReversedTrait;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable, Callable2, RecursiveFunction, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

type Point = PointT<i64>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Segment {
    fr: Point,
    to: Point,
    polygon_id: usize,
}

impl Segment {
    pub fn get_lower_higher(&self) -> (Point, Point) {
        if self.fr.y < self.to.y {
            (self.fr, self.to)
        } else {
            (self.to, self.fr)
        }
    }

    pub fn cmp_p(&self, p: Point) -> Ordering {
        let (lower, higher) = self.get_lower_higher();
        if p.y < lower.y || p.y > higher.y {
            return Ordering::Equal;
        }
        Point::vect_mul(&lower, &higher, &p).cmp(&0)
    }
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_p(other.fr)
            .then_with(|| self.cmp_p(other.to))
            .then_with(|| other.cmp_p(self.fr).reverse())
            .then_with(|| other.cmp_p(self.to).reverse())
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PointLocation {
    all_y: Vec<i64>,
    pub parents: Vec<Option<usize>>,
    tree_nodes: Vec<Vec<Segment>>,
}

impl PointLocation {
    // vertices should be specified in ccw order
    pub fn new(polygons: &[Vec<Point>]) -> Self {
        let mut all_y: Vec<i64> = polygons
            .iter()
            .flat_map(|poly| poly.iter().map(|p| p.y))
            .collect();
        all_y.sort();
        all_y.dedup();
        let tree_nodes_cnt = all_y.len().next_power_of_two() * 2;
        let mut res = Self {
            all_y,
            parents: vec![None; polygons.len()],
            tree_nodes: vec![vec![]; tree_nodes_cnt],
        };
        for (polygon_id, polygon) in polygons.iter().enumerate() {
            for i in 0..polygon.len() {
                let segment = Segment {
                    fr: polygon[i],
                    to: polygon[if i + 1 == polygon.len() { 0 } else { i + 1 }],
                    polygon_id,
                };
                res.add_segment(0, 0, res.all_y.len() - 1, &segment);
            }
        }
        for node in res.tree_nodes.iter_mut() {
            node.sort();
        }

        let mut polygons_left_points: Vec<_> = polygons
            .iter()
            .enumerate()
            .map(|(id, poly)| (poly.iter().min().unwrap(), id))
            .collect();
        polygons_left_points.sort();
        for (&p, polygon_id) in polygons_left_points.into_iter() {
            res.parents[polygon_id] = res.locate_point(p);
        }
        res
    }

    fn add_segment(&mut self, tree_v: usize, l: usize, r: usize, segment: &Segment) {
        let min_y = self.all_y[l];
        let max_y = self.all_y[r];
        let (lower, higher) = segment.get_lower_higher();
        if lower.y <= min_y && higher.y >= max_y {
            self.tree_nodes[tree_v].push(segment.clone());
        } else if lower.y >= max_y || higher.y <= min_y {
            return;
        } else {
            let m = (l + r) >> 1;
            self.add_segment(tree_v * 2 + 1, l, m, segment);
            self.add_segment(tree_v * 2 + 2, m, r, segment);
        }
    }

    pub fn locate_point(&self, p: Point) -> Option<usize> {
        let mut segment: Option<Segment> = None;
        let mut tree_v = 0;
        let (mut l, mut r) = (0, self.all_y.len() - 1);
        loop {
            let min_y = self.all_y[l];
            let max_y = self.all_y[r];
            if p.y < min_y || p.y > max_y {
                break;
            }
            if let Some(idx) = binary_search_last_true(0..self.tree_nodes[tree_v].len(), |i| {
                self.tree_nodes[tree_v][i].cmp_p(p) == Ordering::Less
            }) {
                let new_segment = self.tree_nodes[tree_v][idx];
                if segment.is_none() || segment.unwrap().cmp(&new_segment) == Ordering::Less {
                    segment = Some(new_segment);
                }
            }
            if l + 1 < r {
                let m = (l + r) >> 1;
                let mid_y = self.all_y[m];
                if p.y < mid_y {
                    tree_v = tree_v * 2 + 1;
                    r = m;
                } else {
                    tree_v = tree_v * 2 + 2;
                    l = m;
                }
            } else {
                break;
            }
        }
        segment.and_then(|segment| {
            if segment.fr.y < segment.to.y {
                self.parents[segment.polygon_id]
            } else {
                Some(segment.polygon_id)
            }
        })
    }
}

fn solve(input: &mut Input) {
    let tc = input.usize();
    for test_case in 1..=tc {
        dbg!(test_case);
        let n = input.usize();
        let polygons = gen_vec(n, |_| {
            gen_vec(input.usize(), |_| Point::new(input.read(), input.read())).reversed()
        });

        let queries = gen_vec(input.usize(), |_| {
            [
                input.i64(),
                input.i64(),
                input.i64(),
                input.i64(),
                input.i64(),
            ]
        });

        let point_location = PointLocation::new(&polygons);
        let mut g = vec![vec![]; n + 1];
        for i in 0..n {
            g[point_location.parents[i].unwrap_or(n)].push(i);
        }
        type Mod = Mod_998_244_353;
        let mut hashes = vec![Mod::ZERO; n + 1];
        RecursiveFunction::new(|f, v: usize| {
            let mut h = Mod::new(239);
            let mut child = vec![];
            for &to in g[v].iter() {
                let hc = f.call(to);
                child.push(hc * hc * hc);
            }
            child.sort();
            for &c in child.iter() {
                h = h * Mod::new(239017) + c;
            }
            hashes[v] = h;
            h
        })
        .call(n);
        RecursiveFunction2::new(|f, v: usize, base: Mod| {
            hashes[v] += base;
            hashes[v] *= Mod::new(1_000_000_007);
            for &to in g[v].iter() {
                f.call(to, hashes[v]);
            }
        })
        .call(n, Mod::ZERO);

        let mut xor = 0;
        let mut res = 0;
        for [a, b, c, d, e] in queries.into_iter() {
            let p1 = Point::new(a ^ xor, b ^ xor);
            let p2 = Point::new(c ^ xor, d ^ xor);
            let poly1 = point_location.locate_point(p1).unwrap_or(n);
            let poly2 = point_location.locate_point(p2).unwrap_or(n);
            if hashes[poly1] == hashes[poly2] {
                res += 1;
                xor ^= e;
            }
        }

        out_line!(format!("Case #{}: {}", test_case, res));
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let start = Instant::now();
    solve(&mut input);
    eprintln!("Elapsed: {:?}", start.elapsed());
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::File("zero_crossings_chapter__output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    tester::run_single_test("2");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
