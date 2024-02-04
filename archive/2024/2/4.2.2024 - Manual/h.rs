//{"name":"h","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"h"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::bottom_up_seg_tree::BottomUpSegTree;
use algo_lib::seg_trees::seg_tree_trait::SegTreeNode;

type Point = PointT<i64>;

type Float = f64;

#[derive(Clone, Copy, Debug)]
struct Node {
    pref: Float,
    suf: Float,
    all: Float,
    max: Float,
}

const NEG_INF: Float = -1e18;
impl Default for Node {
    fn default() -> Self {
        Self {
            pref: NEG_INF,
            suf: NEG_INF,
            all: 0.0,
            max: 0.0,
        }
    }
}

impl SegTreeNode for Node {
    fn join_nodes(l: &Self, r: &Self, _context: &Self::Context) -> Self {
        Self {
            pref: (l.pref + r.all).max(r.pref),
            suf: (r.suf + l.all).max(l.suf),
            all: l.all + r.all,
            max: l.max.max(r.max).max(l.pref + r.suf),
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        todo!()
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        todo!()
    }

    type Update = ();

    type Context = ();
}

fn calc_abc(p: Point, q: Point) -> (i64, i64, i64) {
    let a = q.x - p.x;
    let b = q.y - p.y;
    let c = -a * p.x - b * p.y;
    (a, b, c)
}

fn solve_case(pts: &[Point], w: &[Float]) -> Float {
    let n = pts.len();
    let mut res = 0.0f64;
    for i in 0..n {
        res = res.max(w[i]);
    }
    let mut x_pos = vec![usize::MAX; n];

    let mut inside = vec![];
    let mut calc = |p: Point, q: Point| -> Float {
        inside.clear();
        let q2 = p + (q - p).rotate_ccw();
        let (a, b, c) = calc_abc(p, q2);
        for i in 0..n {
            let dy = pts[i].x * a + pts[i].y * b + c;
            if dy >= 0 {
                inside.push(i);
            }
        }
        let (ax, bx, cx) = calc_abc(p, q);
        inside.sort_by_key(|&i| pts[i].x * ax + pts[i].y * bx);
        for idx in 0..inside.len() {
            x_pos[inside[idx]] = idx;
        }

        let (ay, by, cy) = calc_abc(p, q2);
        inside.sort_by_key(|&i| pts[i].x * ay + pts[i].y * by);

        let mut res = 0.0f64;
        let x_coef = 1.0 / (Point::dist2(&p, &q) as Float).sqrt();
        let y_coef = 1.0 / (Point::dist2(&p, &q2) as Float).sqrt();
        let mut st = BottomUpSegTree::new(inside.len(), |_| Node::default());
        for &i in inside.iter() {
            let mut real_x = (pts[i].x * ax + pts[i].y * bx) as Float;
            real_x *= x_coef;
            let mut real_y = (pts[i].x * ay + pts[i].y * by + cy) as Float;
            real_y *= y_coef;
            let new_node = Node {
                pref: w[i] + 2.0 * real_x,
                suf: w[i] - 2.0 * real_x,
                all: w[i],
                max: w[i],
            };
            st.update_point(x_pos[i], new_node);

            let cur_res = st.get_root().max - 2.0 * real_y;
            res = res.max(cur_res);
        }
        res
    };
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut ignore = false;
            for k in 0..n {
                if k != i && k != j {
                    if Point::vect_mul(&pts[i], &pts[j], &pts[k]) == 0
                        && Point::scal_mul(&pts[i], &pts[j], &pts[k]) >= 0
                    {
                        ignore = true;
                    }
                }
            }
            if ignore {
                continue;
            }
            {
                let cur = calc(pts[i], pts[j]);
                res = res.max(cur);
            }
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let mut pts = vec![];
    let mut w = vec![];
    for _ in 0..n {
        pts.push(Point::new(input.i64(), input.i64()));
        w.push(input.f64() as Float);
    }
    let res = solve_case(&pts, &w);
    out.println(res);
}

fn stress() {
    let mut rnd = Random::new(78788);
    let n = 400;
    const MAX_C: i64 = 1_000_000;
    let pts = (0..n)
        .map(|_| Point::new(rnd.gen(0..MAX_C), rnd.gen(0..MAX_C)))
        .collect::<Vec<_>>();
    let w = (0..n)
        .map(|_| rnd.gen(0..MAX_C) as Float)
        .collect::<Vec<_>>();
    let res = solve_case(&pts, &w);
    dbg!(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "h";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
