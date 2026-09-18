use algo_lib::collections::compressed_coords::CompressedCoords;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::geometry::point::PointT;
use algo_lib::geometry::segment_intersection_coef::segment_interection_coef;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::frac::FracT;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree_max_add::{Node, SegTreeMaxAdd};

type Point = PointT<i64>;
type Frac = FracT<i128>;
type SegTree = SegTreeMaxAdd<i32>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Position {
    edge_id: usize,
    coef: Frac,
}

#[allow(non_snake_case)]
fn solve(input: &mut Input, out: &mut Output) {
    let zero = Frac::new(0, 1);
    let one = Frac::new(1, 1);

    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let m = input.usize();
        let a = gen_vec(n, |_| Point::new(input.i64(), input.i64()));
        let b = gen_vec(m, |_| Point::new(input.i64(), input.i64()));
        let p = input.vec::<usize>(n).sub_from_all(1);

        let gen_positions = |a: &[Point], rev: bool| -> Vec<Position> {
            let mut res = Vec::with_capacity(a.len());
            let mut edge_id = 0;
            for i in 0..a.len() {
                let A = a[i];
                let B = a[(i + 1) % a.len()];
                loop {
                    let C = b[edge_id];
                    let D = b[(edge_id + 1) % b.len()];
                    if let Some(coef) = segment_interection_coef([C, D], [A, B]) {
                        if coef >= zero && coef < one {
                            if let Some(coef2) = segment_interection_coef([A, B], [C, D]) {
                                if coef2 > one {
                                    res.push(Position { edge_id, coef });
                                    break;
                                }
                            }
                        }
                    }
                    if rev {
                        edge_id = (edge_id + b.len() - 1) % b.len();
                    } else {
                        edge_id = (edge_id + 1) % b.len();
                    }
                }
            }
            res
        };

        let from_pos = gen_positions(&a, false);
        let to_pos = {
            let a_rev = {
                let mut a_rev = a.clone();
                a_rev.reverse();
                a_rev
            };
            let mut positions = gen_positions(&a_rev, true);
            positions.reverse();
            positions
        };
        let mut all_coords = from_pos.clone();
        all_coords.extend_from_slice(&to_pos);
        let compressed = CompressedCoords::new(&all_coords);
        let mut res = vec![];
        let sz = compressed.size();
        let mut st = SegTree::new(sz, |_| Node { max_val: 0 });
        for &idx in p.iter().rev() {
            let from = compressed.get(from_pos[idx]);
            let to = compressed.get(to_pos[idx]) + 1;
            if from < to {
                st.update(from..to, 1);
            } else {
                st.update(from..sz, 1);
                st.update(0..to, 1);
            }
            res.push(st.get(0..sz).max_val);
        }
        res.reverse();
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "h";
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
