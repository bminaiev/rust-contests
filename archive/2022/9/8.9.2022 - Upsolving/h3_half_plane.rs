//{"name":"H3. Half Plane","group":"Yandex - Upsolving","url":"https://official.contest.yandex.com/ptz-summer-2022/contest/39545/problems/H3/","interactive":false,"timeLimit":12000,"tests":[{"input":"5\n1 1 2 3 4\n12 12 4 6 1\n1 12 5 1 2\n12 1 1 5 5\n6 6 2 0 3\n3\n1 1 4 1 1 2 3 4 5 2 3 4\n1 1 400 1 3 4 2 1 2 3 4 5\n-1 -1 -10 3 2 1 4 6 5 4 3 2\n","output":"2 3 4\n25 50 40\n92 58 139\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"H3HalfPlane"}}}

use std::ops::{Add, AddAssign, Mul};
use std::time::Instant;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::f;
use algo_lib::geometry::line::Line;
use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;
type PointF = PointT<OrdF64>;

const SZ: usize = 3;
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
struct Vec3([u32; SZ]);

const MD: u32 = 1e9 as u32 + 7;
const MD_64: u64 = MD as u64;

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Vec3::default();
        for i in 0..SZ {
            res.0[i] = self.0[i] + rhs.0[i];
            if res.0[i] >= MD {
                res.0[i] -= MD;
            }
        }
        res
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..SZ {
            self.0[i] += rhs.0[i];
            if self.0[i] >= MD {
                self.0[i] -= MD;
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Mat3([[u32; SZ]; SZ]);

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut res = Vec3::default();
        for i in 0..SZ {
            let mut sum = 0u64;
            for j in 0..SZ {
                sum += (self.0[i][j] as u64) * (rhs.0[j] as u64);
            }
            res.0[i] = (sum % MD_64) as u32;
        }
        res
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let mut res = Mat3::default();
        for i in 0..SZ {
            for k in 0..SZ {
                let mut sum = 0u64;
                for j in 0..SZ {
                    sum += (self.0[i][j] as u64) * (rhs.0[j][k] as u64);
                }
                res.0[i][k] = (sum % MD_64) as u32;
            }
        }
        res
    }
}

#[derive(Clone, Copy)]
struct QueryLine {
    a: i32,
    b: i32,
    c: i32,
}

const EPS_F: f64 = 0.1;

impl QueryLine {
    pub fn should_apply(&self, p: &Point) -> bool {
        self.a * p.x + self.b * p.y < self.c
    }

    // -1 -> not
    // 0 -> maybe
    // 1 - yes
    pub fn should_apply_f(&self, p: &PointF) -> i32 {
        let r = (self.a as f64) * p.x.0 + (self.b as f64) * p.y.0 - (self.c as f64);
        if r < -EPS_F {
            return 1;
        }
        if r > EPS_F {
            return -1;
        }
        return 0;
    }
}

#[derive(Clone, Copy)]
struct Query {
    line: QueryLine,
    mat: Mat3,
}

enum NodeInner {
    Leaf(Vec<Point>, Vec<Vec3>),
    Child([Box<TriangleNode>; 2]),
}

struct TriangleNode {
    bound: [PointF; 3],
    sum: Vec3,
    // if [apply] is not None, it is already applied to [sum]
    apply: Option<Mat3>,
    inner: NodeInner,
}

fn are_points_inside_bound(bound: &[PointF; 3], pts: &[Point]) -> bool {
    for i in 0..SZ {
        let cur = bound[i];
        let next = bound[(i + 1) % bound.len()];
        for p in pts.iter() {
            let vmul = PointF::vect_mul(&cur, &next, &p.conv_float()).0;
            if vmul < -EPS_F / 2.0 {
                return false;
            }
        }
    }
    return true;
}

impl TriangleNode {
    const MAX_NODE_SIZE: usize = 64;

    pub fn push(&mut self, mat: Mat3) {
        self.sum = mat * self.sum;
        self.apply = if let Some(alr) = self.apply {
            Some(mat * alr)
        } else {
            Some(mat)
        };
    }

    pub fn query(&mut self, query: &Query) -> (Vec3, i32) {
        let should_apply = [
            query.line.should_apply_f(&self.bound[0]),
            query.line.should_apply_f(&self.bound[1]),
            query.line.should_apply_f(&self.bound[2]),
        ];
        if should_apply[0] == -1 && should_apply[1] == -1 && should_apply[2] == -1 {
            return (Vec3::default(), 1);
        }
        if should_apply[0] == 1 && should_apply[1] == 1 && should_apply[2] == 1 {
            let res = self.sum;
            self.push(query.mat);
            return (res, 1);
        }
        match &mut self.inner {
            NodeInner::Leaf(pts, vals) => {
                if let Some(to_apply) = self.apply {
                    for v in vals.iter_mut() {
                        *v = to_apply * *v;
                    }
                    self.apply = None;
                }
                let mut res = Vec3::default();
                let mut new_sum = Vec3::default();
                for i in 0..pts.len() {
                    if query.line.should_apply(&pts[i]) {
                        res += vals[i];
                        vals[i] = query.mat * vals[i];
                    }
                    new_sum += vals[i];
                }
                self.sum = new_sum;
                return (res, pts.len() as i32);
            }
            NodeInner::Child([c1, c2]) => {
                if let Some(to_push) = self.apply {
                    c1.push(to_push);
                    c2.push(to_push);
                    self.apply = None;
                }
                let r1 = c1.query(query);
                let r2 = c2.query(query);
                self.sum = c1.sum + c2.sum;
                return (r1.0 + r2.0, r1.1 + r2.1);
            }
        }
    }

    pub fn new(bound: [PointF; 3], pts: Vec<Point>, values: Vec<Vec3>) -> Self {
        for i in 0..SZ {
            let cur = bound[i];
            let next = bound[(i + 1) % bound.len()];
            for p in pts.iter() {
                let vmul = PointF::vect_mul(&cur, &next, &p.conv_float()).0;
                if vmul < -EPS_F {
                    dbg!(vmul);
                    dbg!(cur);
                    dbg!(next);
                    dbg!(p);
                    let vmul2 = PointF::vect_mul(&next, &cur, &p.conv_float()).0;
                    dbg!(vmul2);
                }
                assert!(vmul >= -EPS_F);
            }
        }

        let (sum, inner) = if pts.len() <= Self::MAX_NODE_SIZE {
            let mut sum = Vec3::default();
            for v in values.iter() {
                sum += *v;
            }
            (sum, NodeInner::Leaf(pts, values))
        } else {
            let mut ids = gen_vec(pts.len(), id);
            ids.sort_by(|&i1, &i2| {
                PointF::vect_mul(&bound[0], &pts[i1].conv_float(), &pts[i2].conv_float())
                    .cmp(&OrdF64::ZERO)
                    .reverse()
            });

            let gen_children = |ids: &[usize]| -> TriangleNode {
                let children_pts: Vec<Point> = ids.iter().map(|&id| pts[id]).collect();
                let children_values: Vec<Vec3> = ids.iter().map(|&id| values[id]).collect();

                let l = Line::new(&bound[1], &bound[2]);
                let l_first = Line::new(&bound[0], &children_pts[0].conv_float());
                let l_last = Line::new(&bound[0], &children_pts.last_exn().conv_float());

                let rotated_bound = [bound[1], bound[2], bound[0]];
                let new_bound = if let Some(b) = l.intersect(&l_first) {
                    if let Some(c) = l.intersect(&l_last) {
                        let new_bound = [b, c, bound[0]];
                        if are_points_inside_bound(&new_bound, &children_pts) {
                            new_bound
                        } else {
                            // dbg!("f1");
                            rotated_bound
                        }
                    } else {
                        // dbg!("f2");
                        rotated_bound
                    }
                } else {
                    // dbg!("f3");
                    rotated_bound
                };

                Self::new(new_bound, children_pts, children_values)
            };

            let bound = ids.len() / 2;
            let lhs = gen_children(&ids[..bound]);
            let rhs = gen_children(&ids[bound..]);
            let sum = lhs.sum + rhs.sum;
            (sum, NodeInner::Child([Box::new(lhs), Box::new(rhs)]))
        };
        Self {
            bound,
            sum,
            apply: None,
            inner,
        }
    }
}

fn solve_slow(pts: &[Point], mut values: Vec<Vec3>, queries: &[Query]) -> Vec<Vec3> {
    let mut res = Vec::with_capacity(queries.len());
    for q in queries.iter() {
        let mut sum = Vec3::default();
        for i in 0..pts.len() {
            if !q.line.should_apply(&pts[i]) {
                continue;
            }
            sum += values[i];
            values[i] = q.mat * values[i];
        }
        res.push(sum);
    }
    res
}

fn solve_fast(pts: &[Point], values: Vec<Vec3>, queries: &[Query]) -> Vec<Vec3> {
    let mut res = Vec::with_capacity(queries.len());
    const MAX_C: OrdF64 = f!(1.1e6);
    let bound = [
        PointF::new(-MAX_C, -MAX_C),
        PointF::new(MAX_C * f!(3.0), -MAX_C),
        PointF::new(-MAX_C, MAX_C * f!(3.0)),
    ];
    let mut tree = TriangleNode::new(bound, pts.to_vec(), values);
    for (q_it, q) in queries.iter().enumerate() {
        // dbg!(q_it);
        let rr = tree.query(q);
        dbg!(rr.1);
        res.push(rr.0);
    }
    res
}

fn stress() {
    for it in 1..2 {
        dbg!(it);
        let start = Instant::now();
        let mut rnd = Random::new(it);
        const MAX_C: i32 = 1_000_000;
        const MAX_N: usize = 300_000;
        const MAX_F: u32 = 1_000_000_007;
        const MAX_QUERIES: usize = 15_000;
        const MAX_QUERY_C: i32 = 1000;
        let n = MAX_N; //rnd.gen(1..MAX_N);
        let pts = gen_vec(n, |_| {
            Point::new(rnd.gen(-MAX_C..MAX_C), rnd.gen(-1000..1000))
        });
        let values = gen_vec(n, |_| {
            Vec3([rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)])
        });
        let m = MAX_QUERIES; //rnd.gen(1..MAX_QUERIES);
        let queries = gen_vec(m, |_| Query {
            line: QueryLine {
                a: rnd.gen(-1..2),
                b: rnd.gen(-MAX_QUERY_C..MAX_QUERY_C),
                c: rnd.gen(-MAX_QUERY_C * MAX_QUERY_C..MAX_QUERY_C * MAX_QUERY_C),
            },
            mat: Mat3([
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
            ]),
        });

        let fast = solve_fast(&pts, values, &queries);
        dbg!(start.elapsed());
    }
}

fn stress2() {
    for it in 1..2 {
        dbg!(it);
        let start = Instant::now();
        let mut rnd = Random::new(it);
        const MAX_C: i32 = 1_000_000;
        const MAX_N: usize = 300_000;
        const MAX_F: u32 = 1_000_000_007;
        const MAX_QUERIES: usize = 15_000;
        const MAX_QUERY_C: i32 = 1000;
        let n = MAX_N; //rnd.gen(1..MAX_N);
        let pts = gen_vec(n, |_| {
            Point::new(rnd.gen(-MAX_C..MAX_C), rnd.gen(-MAX_C..MAX_C))
        });
        let values = gen_vec(n, |_| {
            Vec3([rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)])
        });
        let m = MAX_QUERIES; //rnd.gen(1..MAX_QUERIES);
        let queries = gen_vec(m, |_| Query {
            line: QueryLine {
                a: rnd.gen(-MAX_QUERY_C..MAX_QUERY_C),
                b: rnd.gen(-MAX_QUERY_C..MAX_QUERY_C),
                c: rnd.gen(-MAX_QUERY_C..MAX_QUERY_C),
            },
            mat: Mat3([
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
            ]),
        });

        let fast = solve_fast(&pts, values, &queries);
        dbg!(start.elapsed());
    }
}

fn stress_correct() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX_C: i32 = 1_000_000;
        const MAX_N: usize = 10_000;
        const MAX_F: u32 = 1_000_000_007;
        const MAX_QUERIES: usize = 1000;
        const MAX_QUERY_C: i32 = 1000;
        let n = rnd.gen(1..MAX_N);
        let pts = gen_vec(n, |_| {
            Point::new(rnd.gen(-MAX_C..MAX_C), rnd.gen(-MAX_C..MAX_C))
        });
        let values = gen_vec(n, |_| {
            Vec3([rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)])
        });
        let m = rnd.gen(1..MAX_QUERIES);
        let queries = gen_vec(m, |_| Query {
            line: QueryLine {
                a: rnd.gen(-MAX_QUERY_C..MAX_QUERY_C),
                b: rnd.gen(-MAX_QUERY_C..MAX_QUERY_C),
                c: rnd.gen(-MAX_QUERY_C..MAX_QUERY_C),
            },
            mat: Mat3([
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
                [rnd.gen(0..MAX_F), rnd.gen(0..MAX_F), rnd.gen(0..MAX_F)],
            ]),
        });

        let slow = solve_slow(&pts, values.clone(), &queries);
        let fast = solve_fast(&pts, values, &queries);
        if slow != fast {
            for i in 0..slow.len() {
                if slow[i] != fast[i] {
                    dbg!("different", i, slow[i], fast[i]);
                    assert!(false);
                }
            }
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    // stress();
    // if true {
    //     return;
    // }

    let n = input.usize();
    let mut pts = vec![];
    let mut values = vec![];
    for _ in 0..n {
        pts.push(Point::new(input.read(), input.read()));
        values.push(Vec3([input.read(), input.read(), input.read()]));
    }
    let m = input.usize();
    let queries = gen_vec(m, |_| Query {
        line: QueryLine {
            a: input.read(),
            b: input.read(),
            c: input.read(),
        },
        mat: Mat3([
            [input.read(), input.read(), input.read()],
            [input.read(), input.read(), input.read()],
            [input.read(), input.read(), input.read()],
        ]),
    });
    let res = solve_fast(&pts, values, &queries);
    for v in res.iter() {
        out_line!(v.0[0], v.0[1], v.0[2]);
    }
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
}
//END MAIN
