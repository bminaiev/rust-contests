//{"name":"J. Cell Tower","group":"Yandex - Stage 18: Grand Prix of Bytedance","url":"https://official.contest.yandex.com/opencupXXII/contest/39023/problems/J/","interactive":false,"timeLimit":2000,"tests":[{"input":"1 1 1 1 2 3 3 3\n0 4 4 4 2 2 2 3\n0 0 5 5 6 6 7 7\n0 9 5 5 6 8 7 7\n9 9 9 1 6 8 8 8\n3 1 1 1 2 2 2 2\n4 5 6 0 0 4 4 3\n7 8 9 0 0 4 3 3\n16\n1111\n2222\n3333\n444\n0000\n5555\n6666\n7777\n8888\n9999\n111\n333\n3456\n789\n3478\n569\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JCellTower"}}}

use std::collections::{BTreeMap, HashSet, VecDeque};

use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::matrix_id_converter::MatrixIdConverter;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Figure(Vec<Point>);

impl Figure {
    pub fn new(mut pts: Vec<Point>) -> Self {
        pts.sort();
        Self(pts)
    }
}

pub fn gen_figures(max_cnt: usize) -> Vec<Figure> {
    let zero = Point::ZERO;
    let start = Figure::new(vec![zero]);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    seen.insert(start.clone());
    queue.push_back(start);
    let shifts = SHIFTS_4;
    while let Some(fig) = queue.pop_back() {
        if fig.0.len() == max_cnt {
            continue;
        }
        for p in fig.0.iter() {
            for shift in shifts.iter() {
                let new_point = p.apply_shift(shift);
                if new_point > zero && !fig.0.contains(&new_point) {
                    let mut new_points = fig.0.clone();
                    new_points.push(new_point);
                    let next_figure = Figure::new(new_points);
                    if !seen.contains(&next_figure) {
                        seen.insert(next_figure.clone());
                        queue.push_back(next_figure);
                    }
                }
            }
        }
    }
    seen.into_iter().collect()
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = 8;
    let a = input.matrix::<i32>(n, n);
    let n_words = input.usize();
    let words: HashSet<_> = gen_vec(n_words, |_| input.string()).into_iter().collect();
    let figures = gen_figures(4);
    let conv = MatrixIdConverter::new(n, n);
    let mut ok_masks = vec![vec![]; n * n];
    for id in 0..n * n {
        let p = conv.conv_back(id);
        for fig in figures.iter() {
            let mut word = vec![];
            let mut ok = true;
            let mut cur_mask = 0;
            for &fig_p in fig.0.iter() {
                let check = p + fig_p;
                if let Some(&v) = check.index_arr2d(&a) {
                    word.push(v as u8 + b'0');
                    cur_mask |= 1u64 << (conv.get_id(&check));
                } else {
                    ok = false;
                }
            }
            if ok && words.contains(&word) {
                ok_masks[id].push(cur_mask);
            }
        }
    }
    let mut seen = BTreeMap::new();
    seen.insert(0u64, 1u64);
    let mut res = 0;
    while !seen.is_empty() {
        let (&mask, &ways) = seen.iter().next().unwrap();
        seen.remove(&mask);

        let pos = mask.trailing_ones() as usize;
        if pos == (n * n) {
            res = ways;
            break;
        }
        for apply_mask in ok_masks[pos].iter() {
            if (apply_mask & mask) == 0 {
                let nmask = apply_mask | mask;
                *seen.entry(nmask).or_default() += ways;
            }
        }
    }
    out_line!(res);
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
