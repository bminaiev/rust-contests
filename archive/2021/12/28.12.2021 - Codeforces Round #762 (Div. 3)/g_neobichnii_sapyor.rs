//{"name":"G. Необычный сапёр","group":"Codeforces - Codeforces Round #762 (Div. 3)","url":"http://codeforces.com/contest/1619/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n\n5 0\n0 0 1\n0 1 4\n1 0 2\n1 1 3\n2 2 9\n\n5 2\n0 0 1\n0 1 4\n1 0 2\n1 1 3\n2 2 9\n\n6 1\n1 -1 3\n0 -1 9\n0 1 7\n-1 0 1\n-1 1 9\n-1 -1 7\n","output":"2\n1\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GNeobichniiSapyor"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

#[derive(Copy, Clone)]
struct Bomb {
    p: Point,
    timer: i32,
    id: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.i32();

    let mut a: Vec<_> = (0..n)
        .map(|id| {
            let x = input.i32();
            let y = input.i32();
            let timer = input.i32();
            Bomb {
                p: Point::new(x, y),
                timer,
                id,
            }
        })
        .collect();

    let mut dsu = Dsu::new(n);

    let near = |b1: &Bomb, b2: &Bomb| -> bool {
        if b1.p.x == b2.p.x && (b1.p.y - b2.p.y).abs() <= k {
            return true;
        }
        b1.p.y == b2.p.y && (b1.p.x - b2.p.x).abs() <= k
    };

    for key in [|b : &Bomb| (b.p.x, b.p.y), |b : &Bomb| (b.p.y, b.p.x)] {
        a.sort_by_key(key);
        for w in a.windows(2) {
            if near(&w[0], &w[1]) {
                dsu.unite(w[0].id, w[1].id);
            }
        }
    }

    a.sort_by_key(|b| b.id);

    let mut smallest_time = vec![i32::MAX; n];
    for id in 0..n {
        let group = dsu.get(id);
        smallest_time[group].update_min(a[id].timer);
    }
    let timers: Vec<_> = (0..n)
        .filter_map(|id| {
            if dsu.is_root(id) {
                Some(smallest_time[id])
            } else {
                None
            }
        })
        .collect();
    let enough_time = binary_search_first_true(0..n + 1, |time| {
        timers
            .iter()
            .filter(|&will_end| *will_end > (time as i32))
            .count()
            <= time + 1
    });
    out_line!(enough_time);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
