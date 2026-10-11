#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::convex_hull::convex_hull;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment_intersection_coef::segment_interection_coef;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::double_double::DoubleDouble;
use algo_lib::misc::rand::Random;

type Point = PointT<i64>;

fn solve_case(a: &[Point]) -> f64 {
    let n = a.len();
    if n < 6 {
        return 0.0;
    }
    let mut a = a.to_vec();
    let sx = a[0].x;
    let sy = a[0].y;
    for i in 0..n {
        a[i].x -= sx;
        a[i].y -= sy;
    }
    let get_p = |base: usize, offset: i32| -> Point {
        let i = ((base + 2 * n) as i32 + offset) % (n as i32);
        a[i as usize]
    };
    let get = |base: usize, offsets: [i32; 6]| -> DoubleDouble {
        let p0 = get_p(base, offsets[0]);
        let p1 = get_p(base, offsets[1]);
        let p2 = get_p(base, offsets[2]);
        let p3 = get_p(base, offsets[3]);
        let p4 = get_p(base, offsets[4]);
        let p5 = get_p(base, offsets[5]);

        let inter1 = segment_interection_coef([p0, p1], [p2, p3]).unwrap();
        let inter2 = segment_interection_coef([p0, p1], [p4, p5]).unwrap();
        let delta = inter2 - inter1;
        assert!(delta.denom > 0);
        assert!(delta.num >= 0);
        assert!(delta.num <= delta.denom);
        let delta_f = DoubleDouble::from(delta.num) / DoubleDouble::from(delta.denom);

        let vmul = p0.x as i128 * p1.y as i128 - p0.y as i128 * p1.x as i128;
        DoubleDouble::from(vmul) * delta_f
    };

    // Balanced summation also limits accumulated rounding error for n = 1e5.
    let sum: DoubleDouble = (0..n).map(|i| get(i, [-1, 1, -2, 0, 0, 2])).sum();
    if n % 2 == 0 {
        return sum.to_f64().max(0.0);
    }
    let mut min_loss = None;
    for base in 0..n {
        // base+1 and base+2 <- same color
        let mut cur_sum = DoubleDouble::ZERO;
        let tmp = [
            [-1, 1, 0, 3, 0, 2],
            [0, 2, -1, 1, 1, 3],
            [1, 3, 0, 2, 2, 4],
            [2, 4, 1, 3, 3, 0],
            [3, 0, 2, 4, -1, 1],
        ];
        for offsets in tmp {
            cur_sum += get(base, offsets);
        }
        if min_loss.map_or(true, |prev| cur_sum < prev) {
            min_loss = Some(cur_sum);
        }
    }

    (sum - min_loss.unwrap()).to_f64().max(0.0)
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = vec![];
        for _ in 0..n {
            a.push(Point::new(input.i64(), input.i64()));
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

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..10);
        const MX_X: i64 = 1_000_000_000;
        let mut a = vec![];
        for _ in 0..n {
            let p = Point::new(rnd.gen_range(0..MX_X), rnd.gen_range(0..MX_X));
            a.push(p);
        }
        let a_hull = convex_hull(&a);
        let res = solve_case(&a_hull);
        dbg!(res);
    }
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "l_white_night_3";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "3");
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
