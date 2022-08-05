//{"name":"Triangles","group":"Google Coding Competitions - World Finals 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/000000000087762e/0000000000b9c555","interactive":false,"timeLimit":15000,"tests":[{"input":"3\n9\n8 2\n10 2\n2 0\n0 5\n2 3\n10 4\n10 0\n8 3\n2 4\n7\n0 0\n0 3\n3 0\n0 1\n1 0\n1 1\n2 2\n3\n0 0\n0 1\n0 2\n","output":"Case #1: 3\n3 4 5\n1 7 9\n6 2 8\nCase #2: 2\n2 3 1\n6 5 4\nCase #3: 0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Triangles"}}}

use std::cmp::{max, min};
use std::collections::HashSet;

use algo_lib::geometry::bounding_box::BoundingBox;
use algo_lib::geometry::point::{PointT, PointWithIdT};
use algo_lib::geometry::segment_intersection::inside_bounding_box;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;
type PointWithId = PointWithIdT<i64>;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
struct Triangle([PointWithId; 3]);

impl Triangle {
    fn is_good(&self) -> bool {
        Point::vect_mul(&self.0[0].p, &self.0[1].p, &self.0[2].p) != 0
    }

    fn get_bb(&self) -> BoundingBox<i64> {
        let mut res = BoundingBox::new(&self.0[0].p, &self.0[1].p);
        res.add(&self.0[2].p);
        return res;
    }
}

#[derive(Debug)]
struct MostCommon {
    p1: Point,
    p2: Point,
    cnt: usize,
}

fn segs_intersect(s1: [Point; 2], s2: [Point; 2]) -> bool {
    if i64::signum(Point::vect_mul(&s1[0], &s1[1], &s2[0]))
        * i64::signum(Point::vect_mul(&s1[0], &s1[1], &s2[1]))
        >= 0
    {
        return false;
    }
    if i64::signum(Point::vect_mul(&s2[0], &s2[1], &s1[0]))
        * i64::signum(Point::vect_mul(&s2[0], &s2[1], &s1[1]))
        >= 0
    {
        return false;
    }
    true
}

fn inside_one_dim(range: (i64, i64), val: i64) -> bool {
    min(range.0, range.1) <= val && val <= max(range.0, range.1)
}

pub fn inside_bb(seg: [Point; 2], p: &Point) -> bool {
    inside_one_dim((seg[0].x, seg[1].x), p.x) && inside_one_dim((seg[0].y, seg[1].y), p.y)
}

fn on_segment(p: Point, seg: [Point; 2]) -> bool {
    return Point::vect_mul(&p, &seg[0], &seg[1]) == 0 && inside_bb(seg, &p);
}

fn is_good_pair_of_triangles(tr1: Triangle, tr2: Triangle) -> bool {
    if !tr1.is_good() || !tr2.is_good() {
        return false;
    }
    let bb1 = tr1.get_bb();
    let bb2 = tr2.get_bb();

    if !bb1.intersect(&bb2) {
        return true;
    }

    for (p1, p2) in [
        (tr1.0[0], tr1.0[1]),
        (tr1.0[1], tr1.0[2]),
        (tr1.0[2], tr1.0[0]),
    ]
    .into_iter()
    {
        for (p3, p4) in [
            (tr2.0[0], tr2.0[1]),
            (tr2.0[1], tr2.0[2]),
            (tr2.0[2], tr2.0[0]),
        ]
        .into_iter()
        {
            if segs_intersect([p1.p, p2.p], [p3.p, p4.p]) {
                return false;
            }
            if on_segment(p3.p, [p1.p, p2.p]) && on_segment(p4.p, [p1.p, p2.p]) {
                return false;
            }
            if on_segment(p1.p, [p3.p, p4.p]) && on_segment(p2.p, [p3.p, p4.p]) {
                return false;
            }
        }
    }
    return true;
}
fn get_the_most_common_line(pts: &[PointWithId], iters: usize, rnd: &mut Random) -> MostCommon {
    let mut res = MostCommon {
        p1: pts[0].p,
        p2: pts[0].p,
        cnt: 1,
    };
    for _ in 0..iters {
        let p1 = pts[rnd.gen(0..pts.len())].p;
        let p2 = pts[rnd.gen(0..pts.len())].p;
        if p1 == p2 {
            continue;
        }
        let mut cnt = 0;
        for p in pts.iter() {
            if PointT::vect_mul(&p1, &p2, &p.p) == 0 {
                cnt += 1;
            }
        }
        if cnt >= res.cnt {
            res.cnt = cnt;
            res.p1 = p1;
            res.p2 = p2;
        }
    }
    res
}

fn fmax(f1: f64, f2: f64) -> f64 {
    if f1 < f2 {
        f2
    } else {
        f1
    }
}

fn first_is_better(f1: f64, f2: f64) -> bool {
    const EPS: f64 = 1e-10;
    f1 + EPS * fmax(1.0, f2) < f2
}

fn find_better_split(tr1: Triangle, tr2: Triangle) -> Option<(Triangle, Triangle)> {
    let all_pts = [tr1.0[0], tr1.0[1], tr1.0[2], tr2.0[0], tr2.0[1], tr2.0[2]];

    let tr_by_mask = |mask: usize| -> Triangle {
        let mut tr = vec![];
        for i in 0..all_pts.len() {
            if ((1 << i) & mask) != 0 {
                tr.push(all_pts[i]);
            }
        }
        assert!(tr.len() == 3);
        Triangle([tr[0], tr[1], tr[2]])
    };

    let get_sum_len = |mut mask: usize| -> f64 {
        let mut res = 0.0;
        let mut bad = false;
        for _ in 0..2 {
            {
                let tr = tr_by_mask(mask);
                let p1 = tr.0[0];
                let p2 = tr.0[1];
                let p3 = tr.0[2];
                if Point::vect_mul(&p1.p, &p2.p, &p3.p) == 0 {
                    bad = true;
                }
                for (p, q) in [(p1, p2), (p2, p3), (p3, p1)].iter() {
                    res += (p.p.dist2(&q.p) as f64).sqrt();
                }
            }
            mask ^= (1 << 6) - 1;
        }
        let tr1 = tr_by_mask(mask);
        let tr2 = tr_by_mask(mask ^ ((1 << 6) - 1));
        if !is_good_pair_of_triangles(tr1, tr2) {
            bad = true;
        }
        if bad {
            return std::f64::MAX;
        }
        return res;
    };

    let mut best_mask = 7;
    let mut best_sum = get_sum_len(best_mask);
    if best_sum < std::f64::MAX / 2.0 {
        return None;
    }

    for mask in 0usize..(1 << 6) {
        if mask.count_ones() != 3 {
            continue;
        }
        let cur_sum = get_sum_len(mask);
        if first_is_better(cur_sum, best_sum) {
            best_sum = cur_sum;
            best_mask = mask;
        }
    }
    assert!(best_sum < std::f64::MAX / 2.0);
    if best_mask == 7 {
        return None;
    }
    let tr1 = tr_by_mask(best_mask);
    let tr2 = tr_by_mask(best_mask ^ ((1 << 6) - 1));
    Some((tr1, tr2))
}

fn solve_case(mut pts: Vec<PointWithId>) -> Vec<Triangle> {
    const ITERS: usize = 100;
    let mut rnd = Random::new(87333);
    let mut most_common = get_the_most_common_line(&pts, ITERS, &mut rnd);
    let mut left = pts.len() - most_common.cnt;
    let at_most_triangles = min(left, pts.len() / 3);
    let mut triangles = vec![];
    let mut already = 0;

    let magic_point = PointWithId::new(Point::ZERO, pts.len() + 5);
    let magic_triangle = Triangle([magic_point, magic_point, magic_point]);
    let mut rnd2 = Random::new(777);

    let mut add_triangle = RecursiveFunction::new(|f, mut tr: Triangle| -> Option<Triangle> {
        if tr == magic_triangle {
            let idx = rnd2.gen(0..triangles.len());
            let res = triangles[idx];
            triangles.swap_remove(idx);
            return Some(res);
        }
        let mut it = 0;
        while it != triangles.len() {
            let another_tr = triangles[it];
            if is_good_pair_of_triangles(tr, another_tr) {
                it += 1;
                continue;
            }
            if let Some((tr1, tr2)) = find_better_split(tr, another_tr) {
                triangles.swap_remove(it);
                f.call(tr1);
                tr = tr2;
                it = 0;
            } else {
                it += 1;
            }
        }
        triangles.push(tr);
        None
    });

    let mut need_some_random = false;
    while pts.len() >= 3 && already < at_most_triangles {
        // dbg!(pts.len());
        if (left + 5) * 2 >= pts.len() {
            most_common = get_the_most_common_line(&pts, ITERS, &mut rnd);
            left = pts.len() - most_common.cnt;
        }
        if left == 0 {
            let last = add_triangle.call(magic_triangle).unwrap();
            pts.push(last.0[0]);
            pts.push(last.0[1]);
            pts.push(last.0[2]);
            already -= 1;
            need_some_random = true;
            // dbg!("rollback!");
            continue;
        }
        let mut most_common_line = vec![];
        for p in pts.iter() {
            if Point::vect_mul(&most_common.p1, &most_common.p2, &p.p) == 0 {
                most_common_line.push(p);
            }
        }
        most_common_line.sort_unstable();
        let p1 = most_common_line[0].clone();
        let p2 = most_common_line[1].clone();

        let mut best_dist = std::i64::MAX;
        let mut best_p = p2;
        for p3 in pts.iter() {
            if Point::vect_mul(&most_common.p1, &most_common.p2, &p3.p) == 0 {
                continue;
            }
            let sum_dist = p3.p.dist2(&p1.p) + p3.p.dist2(&p2.p);
            if sum_dist < best_dist {
                best_dist = sum_dist;
                best_p = p3.clone();
            }
        }
        if need_some_random {
            for _ in 0..10 {
                let idx = rnd.gen(0..pts.len());
                let p3 = pts[idx];
                if Point::vect_mul(&most_common.p1, &most_common.p2, &p3.p) == 0 {
                    continue;
                }
                best_p = p3.clone();
            }
        }
        assert_ne!(best_p, p2);
        for p in [p1, p2, best_p].iter() {
            let index = pts.iter().position(|x| *x == *p).unwrap();
            pts.remove(index);
        }
        let new_triangle = Triangle([p1, p2, best_p]);
        add_triangle.call(new_triangle);
        already += 1;
    }
    assert_eq!(triangles.len(), at_most_triangles);
    triangles
}

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let mut pts = gen_vec(n, |id| {
        PointWithId::new(PointT::new(input.read(), input.read()), id)
    });
    let triangles = solve_case(pts);
    out_line!(format!("Case #{}: {}", test_case, triangles.len()));
    for t in triangles.iter() {
        out_line!(t.0[0].id() + 1, t.0[1].id() + 1, t.0[2].id() + 1);
    }
}

fn stress() {
    for it in 216.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(3..13);
        let n = 3000;
        const MAX: i64 = 1e9 as i64;
        let pts = gen_vec(n, |id| {
            PointWithId::new(Point::new(rnd.gen(0..MAX), rnd.gen(0..MAX)), id)
        });
        let mut set = HashSet::new();
        for p in pts.iter() {
            set.insert(p.p);
        }
        if set.len() != n {
            dbg!("skip");
            continue;
        }
        dbg!(n);
        let triangles = solve_case(pts);
        for i in 0..triangles.len() {
            for j in i + 1..triangles.len() {
                if !(is_good_pair_of_triangles(triangles[i], triangles[j])) {
                    dbg!(triangles[i]);
                    dbg!(triangles[j]);
                    let r = find_better_split(triangles[i], triangles[j]);
                    dbg!(r);
                    assert!(false);
                }
            }
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
    // tester::run_tests();
    tester::run_stress(stress);
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
