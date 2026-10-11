#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment_intersection_coef::segment_interection_coef;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::frac::FracT;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;

type Point = PointT<i64>;
type PointF = PointT<OrdF64>;
type Frac = FracT<i128>;

fn seg_sum(p1: PointF, p2: PointF) -> f64 {
    p1.x.0 * p2.y.0 - p1.y.0 * p2.x.0
}

fn solve_case(a: &[Point]) -> f64 {
    let n = a.len();
    if n < 6 {
        return 0.0;
    }
    let get_p = |base: usize, offset: i32| -> Point {
        let i = ((base + 2 * n) as i32 + offset) % (n as i32);
        a[i as usize]
    };
    let to_f = |f: Frac| -> f64 {
        let num = f.num as f64;
        let denom = f.denom as f64;
        num / denom
    };
    let segm_coef = |p0: Point, p1: Point, coef: Frac| -> PointF {
        let not_coef = Frac::ONE - coef;
        let x = not_coef * Frac::new(p0.x as i128, 1) + coef * Frac::new(p1.x as i128, 1);
        let y = not_coef * Frac::new(p0.y as i128, 1) + coef * Frac::new(p1.y as i128, 1);
        PointF::new(OrdF64(to_f(x)), OrdF64(to_f(y)))
    };
    let get = |base: usize, offsets: [i32; 4]| -> PointF {
        let p0 = get_p(base, offsets[0]);
        let p1 = get_p(base, offsets[1]);
        let p2 = get_p(base, offsets[2]);
        let p3 = get_p(base, offsets[3]);
        let inter = segment_interection_coef([p0, p1], [p2, p3]);
        let inter = inter.unwrap();
        assert!(inter >= Frac::ZERO);
        assert!(inter <= Frac::ONE);
        segm_coef(p0, p1, inter)
    };
    let get_def = |i: usize| -> f64 {
        let p0 = get(i, [-2, 0, -1, 1]);
        let p1 = get(i, [-1, 1, 0, 2]);
        seg_sum(p0, p1)
    };
    let mut sum = 0.0;
    for i in 0..n {
        let tmp = get_def(i);
        dbg!(i, a[i], tmp);
        assert!(tmp.is_finite());
        sum += tmp;
    }
    assert!(sum >= 0.0);
    if n % 2 == 0 {
        return sum;
    }
    let mut res = f64::NEG_INFINITY;
    for base in 0..n {
        // base+1 and base+2 <- same color
        let mut cur_sum = sum;
        for i in 0..4 {
            cur_sum -= get_def(base + i);
        }
        let p0 = get(base, [-2, 0, -1, 1]);
        let p1 = get(base, [-1, 1, 0, 3]);
        let p2 = get(base, [0, 3, 2, 4]);
        let p3 = get(base, [2, 4, 3, 5]);
        cur_sum += seg_sum(p0, p1);
        cur_sum += seg_sum(p1, p2);
        cur_sum += seg_sum(p2, p3);
        if cur_sum > res {
            res = cur_sum;
        }
        dbg!(base, cur_sum);
    }
    res
}

fn centrize(a: &[Point]) -> Vec<Point> {
    let mut sumx = 0;
    let mut sumy = 0;
    for p in a {
        sumx += p.x;
        sumy += p.y;
    }
    let cx = sumx / (a.len() as i64);
    let cy = sumy / (a.len() as i64);
    let mut res = vec![];
    for p in a {
        res.push(Point::new(p.x - cx, p.y - cy));
    }
    res
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        dbg!("!!");
        let n = input.usize();
        let mut a = vec![];
        for i in 0..n {
            a.push(Point::new(input.i64(), input.i64()));
        }
        let a = centrize(&a);
        for p in a.iter() {
            dbg!(p);
        }
        let res = solve_case(&a);
        out.println(res / 2.0);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "l_white_night_3";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    run_single_test(PROBLEM_NAME, run, "3");
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
