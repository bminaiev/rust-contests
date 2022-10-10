//{"name":"E1: Zero Crossings - Chapter 1","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Round 3","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/round-3/problems/E1","interactive":false,"timeLimit":360000,"tests":[{"input":"4\n4\n3\n17 13 18 11 16 11\n4\n1 1 1 2 2 2 2 1\n8\n15 8 15 4 12 1 5 1 2 4 2 8 5 11 12 11\n4\n5 4 5 8 12 8 12 4\n1\n7 6 10 6\n7\n6\n10 90 20 90 30 80 20 70 10 70 0 80\n5\n10 0 0 50 10 60 100 100 100 0\n4\n30 40 30 60 90 90 90 40\n4\n90 20 90 10 40 10 30 30\n3\n20 10 10 50 30 20\n3\n40 50 40 60 50 50\n3\n60 50 80 70 80 50\n7\n10 80 20 80\n30 90 40 90\n40 90 20 80\n60 70 70 55\n20 20 40 20\n30 10 90 30\n90 30 30 90\n1\n3\n0 4 7 7 3 0\n1\n3 4 2 6\n1\n4\n0 4 0 1000000000 7 7 3 0\n1\n3 4 2 6\n","output":"Case #1: 1\nCase #2: 4\nCase #3: 0\nCase #4: 1\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"zero_crossings_chapter__.*input[.]txt"},"output":{"type":"file","fileName":"zero_crossings_chapter__output.txt","pattern":null},"languages":{"java":{"taskClass":"E1ZeroCrossingsChapter1"}}}

use algo_lib::collections::index_of::IndexOf;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    run_parallel::<Job>(input, Some(8), &());
}

type Point = PointT<i64>;

#[derive(Clone, Default)]
struct Job {
    poly: Vec<Vec<Point>>,
    queries: Vec<Vec<Point>>,
    res: i64,
}

impl ParallelJob for Job {
    type Context = ();

    fn read_input(&mut self, input: &mut Input) {
        let n = input.usize();
        for _ in 0..n {
            let cnt = input.usize();
            let p = gen_vec(cnt, |_| Point::new(input.read(), input.read()));
            self.poly.push(p);
        }
        let q = input.usize();
        for j in 0..q {
            self.queries.push(vec![]);
            for i in 0..2 {
                self.queries[j].push(Point::new(input.read(), input.read()));
            }
        }
    }

    fn solve(&mut self, _context: &Self::Context) {
        let mut events = vec![];
        for i in 0..self.queries.len() {
            for j in 0..2 {
                events.push(Event {
                    y: self.queries[i][j].y,
                    type_: 2,
                    id: (i, j),
                })
            }
        }
        let mut is_left = vec![vec![]; self.poly.len()];
        for i in 0..self.poly.len() {
            let pts = &self.poly[i];
            for j in 0..pts.len() {
                let p1 = pts[j];
                let p2 = pts[(j + 1) % pts.len()];
                if p1.y == p2.y {
                    continue;
                }
                if p1.y < p2.y {
                    // left
                    events.push(Event {
                        y: p1.y,
                        type_: 0,
                        id: (i, j),
                    });
                    events.push(Event {
                        y: p2.y,
                        type_: 1,
                        id: (i, j),
                    });
                    is_left[i].push(true);
                } else {
                    // left
                    events.push(Event {
                        y: p2.y,
                        type_: 0,
                        id: (i, j),
                    });
                    events.push(Event {
                        y: p1.y,
                        type_: 1,
                        id: (i, j),
                    });
                    is_left[i].push(false);
                }
            }
        }
        events.sort_by(|e1, e2| e1.y.cmp(&e2.y).then(e1.type_.cmp(&e2.type_)));
        let mut elems = vec![];

        let get_x = |y: i64, e: &Elem| -> f64 {
            let poly = &self.poly[e.poly_id];
            let p1 = poly[e.e_id];
            let p2 = if e.e_id + 1 == poly.len() {
                poly[0]
            } else {
                poly[e.e_id + 1]
            };
            let coef = (y - p1.y) as f64 / (p2.y - p1.y) as f64;
            p1.x as f64 + (p2.x - p1.x) as f64 * coef
        };

        let mut magic = 0;
        let mut seen_magic = vec![0; self.poly.len()];
        let mut query_parents = vec![[self.poly.len(); 2]; self.queries.len()];
        let mut poly_parents = vec![self.poly.len(); self.poly.len()];
        for &e in events.iter() {
            magic += 1;
            if e.type_ == 0 {
                let new_e = Elem {
                    poly_id: e.id.0,
                    e_id: e.id.1,
                    left: is_left[e.id.0][e.id.1],
                };
                let xx = get_x(e.y, &new_e);
                let ins_pos = binary_search_first_true(0..elems.len(), |check| {
                    get_x(e.y, &elems[check]) >= xx
                });
                if e.id.1 == 0 {
                    let mut parent = self.poly.len();
                    for pos in (0..ins_pos).rev() {
                        let elem = elems[pos];
                        if elem.left {
                            if seen_magic[elem.poly_id] == magic {
                                continue;
                            } else {
                                parent = elem.poly_id;
                                break;
                            }
                        } else {
                            seen_magic[elem.poly_id] = magic;
                        }
                    }
                    poly_parents[e.id.0] = parent;
                }

                elems.insert(ins_pos, new_e);
            } else if e.type_ == 1 {
                let old_e = Elem {
                    poly_id: e.id.0,
                    e_id: e.id.1,
                    left: is_left[e.id.0][e.id.1],
                };
                let pos = elems.index_of(&old_e).unwrap();
                elems.remove(pos);
            } else if e.type_ == 2 {
                let id = e.id.0;
                let xx = self.queries[id][e.id.1].x as f64;
                let ins_pos = binary_search_first_true(0..elems.len(), |check| {
                    get_x(e.y, &elems[check]) >= xx
                });
                let mut parent = self.poly.len();
                for pos in (0..ins_pos).rev() {
                    let elem = elems[pos];
                    if elem.left {
                        if seen_magic[elem.poly_id] == magic {
                            continue;
                        } else {
                            parent = elem.poly_id;
                            break;
                        }
                    } else {
                        seen_magic[elem.poly_id] = magic;
                    }
                }
                query_parents[id][e.id.1] = parent;
            }
        }

        type Mod = Mod_998_244_353;
        let mut g = vec![vec![]; self.poly.len() + 1];
        for v in 0..self.poly.len() {
            g[poly_parents[v]].push(v);
        }
        let mut hashes = vec![Mod::ZERO; self.poly.len() + 1];
        RecursiveFunction::new(|f, v: usize| {
            let mut child = vec![Mod::new(239)];
            for to in g[v].iter() {
                let zz = f.call(*to);
                let zz = zz * zz * zz;
                child.push(zz);
            }
            child.sort();
            let mut r = Mod::ZERO;
            for x in child.iter() {
                r = r * Mod::new(3321) + *x;
            }
            hashes[v] = r;
            r
        })
        .call(self.poly.len());
        RecursiveFunction::new(|f, v: usize| {
            for &to in g[v].iter() {
                hashes[to] = hashes[v] * Mod::new(444991) + hashes[to];
                f.call(to);
            }
        })
        .call(self.poly.len());
        for i in 0..self.queries.len() {
            let p1 = query_parents[i][0];
            let p2 = query_parents[i][1];
            if hashes[p1] == hashes[p2] {
                self.res += 1;
            }
        }
    }

    fn write_output(&mut self, test_case: usize) {
        out_line!(format!("Case #{}: {}", test_case, self.res));
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Elem {
    poly_id: usize,
    e_id: usize,
    left: bool,
}

// 0 - create
// 1 - del
// 2 - query
#[derive(Clone, Copy)]
struct Event {
    y: i64,
    type_: usize,
    id: (usize, usize),
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
