//{"name":"midnight-strawberry","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"midnight-strawberry"}}}

use rayon::prelude::*;
use std::f64::consts::PI;
use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;
use algo_lib::misc::simulated_annealing::SimulatedAnnealing;

type Point = PointT<f64>;

fn solve2(l1: f64, l2: f64, p: Point) -> Option<[[f64; 2]; 2]> {
    // Distance from origin to target point
    let d = (p.x.powi(2) + p.y.powi(2)).sqrt();

    // Check reachability: p must be within the annulus [|l1-l2|, l1+l2]
    if d > l1 + l2 + 1e-9 || d < (l1 - l2).abs() - 1e-9 {
        return None;
    }

    // Base angle: angle from OX to the line connecting (0,0) and p.
    let base = p.y.atan2(p.x);

    // Compute delta using the law of cosines for the triangle (0, l1, p)
    let cos_delta = (l1 * l1 + d * d - l2 * l2) / (2.0 * l1 * d);
    // Clamp due to numerical errors
    let cos_delta = cos_delta.clamp(-1.0, 1.0);
    let delta = cos_delta.acos();

    // Compute phi: the angle between l1 and l2 when extended at the joint.
    let cos_phi = (l1 * l1 + l2 * l2 - d * d) / (2.0 * l1 * l2);
    let cos_phi = cos_phi.clamp(-1.0, 1.0);
    let phi = cos_phi.acos();

    // Two configurations:
    // Configuration 1: first segment rotated by (base - delta)
    // and second segment rotated by (phi - PI) relative to the first.
    let sol1_angle1 = (base - delta).rem_euclid(2.0 * PI);
    let sol1_angle2 = ((-phi) + PI).rem_euclid(2.0 * PI);

    // Configuration 2: first segment rotated by (base + delta)
    // and second segment rotated by (-phi + PI) relative to the first.
    let sol2_angle1 = (base + delta).rem_euclid(2.0 * PI);
    let sol2_angle2 = ((phi) - PI).rem_euclid(2.0 * PI);

    Some([[sol1_angle1, sol1_angle2], [sol2_angle1, sol2_angle2]])
}

fn solve3(len: &[f64], p: Point, rnd: &mut Random) -> [f64; 3] {
    loop {
        let angle = rnd.gen_double() * 2.0 * PI;
        let p1 = Point::new(len[0] * angle.cos(), len[0] * angle.sin());
        let delta_p = p - p1;
        if let Some(sol2) = solve2(len[1], len[2], delta_p) {
            let sol2 = sol2[rnd.gen(0..2)].to_vec();
            let mut sol = [0.0; 3];
            sol[0] = angle;
            sol[1] = (sol2[0] - angle).rem_euclid(2.0 * PI);
            sol[2] = sol2[1];
            {
                let mut cur_angle = 0.0;
                let mut now_p = Point::new(0.0, 0.0);
                for i in 0..3 {
                    cur_angle += sol[i];
                    let cur_p = Point::new(len[i] * cur_angle.cos(), len[i] * cur_angle.sin());
                    now_p += cur_p;
                }
                let dist = (now_p.x - p.x).powi(2) + (now_p.y - p.y).powi(2);
                dbg!(now_p);
                dbg!(p);
                assert!(dist < 1e-9);
            }
            return sol;
        }
    }
}

fn solve_case3(len: &[f64], a: &[Point]) -> (Vec<usize>, Vec<Vec<f64>>) {
    let mut sol3 = vec![];
    let mut rnd = Random::new(234234);
    for i in 0..a.len() {
        dbg!("solve3", i, a[i]);
        let sol = solve3(len, a[i], &mut rnd);
        sol3.push(sol);
    }
    let mut perm: Vec<usize> = (0..a.len()).collect();
    let mut sa = SimulatedAnnealing::new(
        60.0,
        algo_lib::misc::simulated_annealing::SearchFor::MinimumScore,
        100.0,
        1e-3,
        1e9,
    );
    let extract_moves = |perm: &[usize]| -> Vec<Vec<f64>> {
        let mut angles = vec![];
        for &id in perm.iter() {
            angles.push(sol3[id].to_vec());
        }
        angles
    };
    while sa.should_continue() {
        let i = rnd.gen(0..perm.len());
        let j = rnd.gen(i + 1..perm.len() + 1);
        let q_type = rnd.gen(0..2);
        let mut prev_sol = vec![];
        if q_type == 0 {
            perm[i..j].reverse();
        } else {
            prev_sol = sol3[perm[i]].to_vec();
        }
        perm[i..j].reverse();
        let new_score = calc_score(&extract_moves(&perm));
        if !sa.should_go(new_score) {
            perm[i..j].reverse();
        }
    }
    let moves = extract_moves(&perm);
    (perm, moves)
}

fn solve_case2(len: &[f64], a: &[Point]) -> (Vec<usize>, Vec<Vec<f64>>) {
    let mut sol2 = vec![];
    for i in 0..a.len() {
        let sol = solve2(len[0], len[1], a[i]).unwrap();
        sol2.push(sol);
    }

    let mut perm: Vec<usize> = (0..a.len()).collect();
    let mut side = vec![0; a.len()];
    let mut rnd = Random::new(345345);
    for i in 0..side.len() {
        side[i] = rnd.gen(0..2);
    }
    let mut sa = SimulatedAnnealing::new(
        60.0,
        algo_lib::misc::simulated_annealing::SearchFor::MinimumScore,
        100.0,
        1e-3,
        1e9,
    );
    let extract_moves = |perm: &[usize], side: &[usize]| -> Vec<Vec<f64>> {
        let mut angles = vec![];
        for &id in perm.iter() {
            angles.push(sol2[id][side[id]].to_vec());
        }
        angles
    };
    while sa.should_continue() {
        let i = rnd.gen(0..perm.len());
        let j = rnd.gen(i + 1..perm.len() + 1);
        let q_type = rnd.gen(0..2);
        if q_type == 0 {
            perm[i..j].reverse();
        } else {
            side[i] = 1 - side[i];
        }
        let new_score = calc_score(&extract_moves(&perm, &side));
        if !sa.should_go(new_score) {
            if q_type == 0 {
                perm[i..j].reverse();
            } else {
                side[i] = 1 - side[i];
            }
        }
    }
    let moves = extract_moves(&perm, &side);
    (perm, moves)
}

fn calc_score(moves: &[Vec<f64>]) -> f64 {
    let mut cur = vec![0.0; moves.len()];
    let mut res = 0.0;
    for i in 0..moves.len() {
        let mut max_delta = 0.0;
        for j in 0..moves[i].len() {
            let mut delta = (moves[i][j] - cur[j]).rem_euclid(2.0 * PI);
            if delta > PI {
                delta = 2.0 * PI - delta;
            }
            if delta > max_delta {
                max_delta = delta;
            }
        }
        res += max_delta;
        cur = moves[i].clone();
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let k = input.usize();
    let len = input.vec::<f64>(k);
    let n = input.usize();
    let mut a = vec![];
    for _ in 0..n {
        let x = input.f64();
        let y = input.f64();
        a.push(Point::new(x, y));
    }
    dbg!(k, len, n);
    let (perm, sol) = if k == 2 {
        solve_case2(&len, &a)
    } else {
        solve_case3(&len, &a)
    };
    let score = calc_score(&sol);
    // let sol = conv_sol(&sol);
    dbg!(score);
    for (i, sol) in sol.iter().enumerate() {
        out.print(perm[i] + 1);
        out.print(" ");
        // let sol_deg: Vec<_> = sol.iter().map(|x| conv_to_deg(*x)).collect();
        out.println(sol.to_vec());
        // unreachable!();
    }
}

// fn conv_sol(sol: &[Vec<f64>]) -> Vec<Vec<f64>> {
//     let mut res: Vec<Vec<f64>> = vec![];
//     let mut prev = vec![0.0; sol[0].len()];
//     for s in sol.iter() {
//         let mut deltas = vec![0.0; s.len()];
//         for i in 0..s.len() {
//             deltas[i] = (s[i] - prev[i]).rem_euclid(2.0 * PI);
//             if deltas[i] > std::f64::consts::PI {
//                 deltas[i] -= 2.0 * std::f64::consts::PI;
//             }
//         }
//         let mut next = if res.is_empty() {
//             vec![0.0; s.len()]
//         } else {
//             res.last().unwrap().clone()
//         };
//         for i in 0..s.len() {
//             next[i] += deltas[i];
//         }
//         res.push(next);
//         prev = s.to_vec();
//     }
//     res
// }

// fn conv_to_deg(x: f64) -> f64 {
//     x * 180.0 / PI
// }

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

fn stress2() {
    for t in 1..9 {
        let filename = format!("./tasks/midnight-strawberry/tests/{t:02}");
        dbg!(filename);
        let mut input = Input::new_file(format!("./tasks/midnight-strawberry/tests/{t:02}"));
        let mut output = Input::new_file(format!("./tasks/midnight-strawberry/out/{t:02}.out"));
        let k = input.usize();
        input.vec::<f64>(k);
        let n = input.usize();
        let mut pos = vec![0.0; k];
        let mut res = 0.0;
        for _ in 0..n {
            let id = output.usize() - 1;
            let mut mx = 0.0;
            for j in 0..k {
                let x = output.f64();
                let delta = (x - pos[j]).abs();
                let delta = delta.min(2.0 * PI - delta);
                if delta > mx {
                    mx = delta;
                }
                pos[j] = x;
            }
            res += mx;
        }
        dbg!(t, res);
    }
}

fn stress() {
    (9..10).into_par_iter().for_each(|tc| {
        let filename = format!("./tasks/midnight-strawberry/tests/{tc:02}");
        dbg!(filename);
        let mut input = Input::new_file(format!("./tasks/midnight-strawberry/tests/{tc:02}"));
        let mut output = Output::new_file(format!("./tasks/midnight-strawberry/out/{tc:02}.out"));
        solve(&mut input, &mut output, tc);
        output.flush();
    });
    // for tc in 1..8 {
    //     let filename = format!("./tasks/midnight-strawberry/tests/{tc:02}");
    //     dbg!(filename);
    //     let mut input = Input::new_file(format!("./tasks/midnight-strawberry/tests/{tc:02}"));
    //     let mut output = Output::new_file(format!("./tasks/midnight-strawberry/out/{tc:02}.out"));
    //     solve(&mut input, &mut output, 1);
    //     output.flush();
    // }
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "midnight-strawberry";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
