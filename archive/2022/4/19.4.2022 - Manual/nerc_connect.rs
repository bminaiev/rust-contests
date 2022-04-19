//{"name":"nerc_connect","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"nerc_connect"}}}

use std::collections::BTreeSet;

use algo_lib::geometry::bounding_box::BoundingBox;
use algo_lib::geometry::point::PointT;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::pairs_iter::PairsIterTrait;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::ordered_pair::OrderedPair;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i64>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = 3;
    let a = gen_vec(n, |_| Point::new(input.read(), input.read()));
    let mut edges = BTreeSet::new();
    for p1 in a.iter() {
        for p2 in a.iter() {
            for p3 in a.iter() {
                for p4 in a.iter() {
                    let x1 = p1.x;
                    let x2 = p2.x;
                    let y1 = p3.y;
                    let y2 = p4.y;
                    if (x1 == x2 || y1 == y2) && (x1, y1) != (x2, y2) {
                        let from = Point::new(x1, y1);
                        let to = Point::new(x2, y2);
                        edges.insert(OrderedPair::new(from, to));
                    }
                }
            }
        }
    }
    let edges: Vec<_> = edges.into_iter().collect();
    let on_seg = gen_vec(edges.len(), |idx| {
        let bbox = BoundingBox::new(&edges[idx].min, &edges[idx].max);
        (0..n)
            .filter(|p_id| bbox.contains(&a[*p_id]))
            .chain(
                (0..edges.len())
                    .filter(|&e_id2| -> bool {
                        bbox.contains(&edges[e_id2].min) || bbox.contains(&edges[e_id2].max)
                    })
                    .map(|x| x + n),
            )
            .collect::<Vec<_>>()
    });
    let mut best = (std::i64::MAX, 0);
    for mask in 0..1 << edges.len() {
        let full_mask = 7 | (mask << 3);
        let mut dsu = Dsu::new(n + edges.len());
        let mut cur_cost = 0;
        for i in 0..edges.len() {
            if ((1 << i) & mask) != 0 {
                cur_cost += edges[i].min.dist_manh(&edges[i].max);
                for (&p1, &p2) in on_seg[i].iter().pairs() {
                    let need_mask = (1 << p1) | (1 << p2);
                    if (need_mask & full_mask) == need_mask {
                        dsu.unite(p1, p2);
                    }
                }
            }
        }
        if dsu.get(1) == dsu.get(0) && dsu.get(2) == dsu.get(0) {
            best.update_min((cur_cost, mask));
        }
    }
    assert_ne!(best.0, std::i64::MAX);
    let mask: i32 = best.1;
    out_line!(mask.count_ones());
    for i in 0..edges.len() {
        if ((1 << i) & mask) != 0 {
            out_line!(edges[i].min, edges[i].max);
        }
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
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
