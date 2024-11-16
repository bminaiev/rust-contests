//{"name":"e-qf","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e-qf"}}}

use algo_lib::collections::fx_hash_map::FxHashMap;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;

type Point = PointT<i64>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Circle {
    center: Point,
    r: i64,
    id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
struct Region {
    lvl: usize,
    x: i64,
    y: i64,
}

fn good_circles(c1: Circle, c2: Circle) -> bool {
    let dx = c1.center.x - c2.center.x;
    let dy = c1.center.y - c2.center.y;
    (c1.r + c2.r) * (c1.r + c2.r) == dx * dx + dy * dy
}

fn count(c1: Circle, circles: &[Circle]) -> usize {
    const MX: usize = 100;
    if circles.len() > MX {
        let mut rnd = Random::new(787788);
        const SZ: usize = 20;
        let mut est = 0;
        for _ in 0..SZ {
            let c2 = circles[rnd.gen(0..circles.len())];
            if good_circles(c1, c2) {
                est += 1;
            }
        }
        if est == 0 {
            return 0;
        }
        if est == SZ {
            return circles.len();
        }
        let mid = circles.len() / 2;
        return count(c1, &circles[..mid]) + count(c1, &circles[mid..]);
    }
    let mut res = 0;
    for &c2 in circles.iter() {
        if good_circles(c1, c2) {
            res += 1;
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    // 13:18
    // 13:27
    let n = input.usize();
    const INF: i64 = 1e9 as i64 + 10;
    let mut circles = gen_vec(n, |id| Circle {
        id,
        center: Point::new(input.i64() + INF, input.i64()),
        r: input.i64(),
    });
    circles.sort();
    let mut res = 0;
    let mut hm = FxHashMap::<Region, Vec<Circle>>::default();
    const MAX_LVL: usize = 31;
    for lvl in (0..MAX_LVL).rev() {
        let r_min = 1i64 << lvl;
        let r_max = r_min * 2;

        for &c in circles.iter() {
            if c.r >= r_min && c.r < r_max {
                for prev_lvl in lvl..MAX_LVL {
                    let x = c.center.x / (1 << (prev_lvl + 2));
                    let y = c.center.y / (1 << (prev_lvl + 2));
                    const DELTA: i64 = 1;
                    for dx in -DELTA..=DELTA {
                        for dy in -DELTA..=DELTA {
                            if let Some(entry) = hm.get(&Region {
                                lvl: prev_lvl,
                                x: x + dx,
                                y: y + dy,
                            }) {
                                res += count(c, entry)
                            }
                        }
                    }
                }
                hm.entry(Region {
                    lvl,
                    x: c.center.x / (r_max * 2),
                    y: c.center.y / (r_max * 2),
                })
                .or_default()
                .push(c);
            }
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e-qf";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
