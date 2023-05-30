//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use std::collections::HashMap;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut changes = vec![];
    let mut hm = HashMap::new();

    let conv = |hm: &mut HashMap<(i32, i32), usize>, p: (i32, i32)| -> usize {
        if let Some(&v) = hm.get(&p) {
            return v;
        }
        let v = hm.len();
        hm.insert(p, v);
        v
    };

    for _ in 0..n {
        let p1 = (input.i32(), input.i32());
        let p2 = (input.i32(), input.i32());
        let id1 = conv(&mut hm, p1);
        let id2 = conv(&mut hm, p2);
        changes.push((id1, id2));
    }

    let n = hm.len();
    let mut dsu = Dsu::new(n);
    let mut sz = vec![0; n];
    let mut cnt_odds = vec![0; n];

    let mut cnt_friendly = n;
    let mut cnt_size_1 = n;

    for &(p1, p2) in changes.iter() {
        for &v in [p1, p2].iter() {
            sz[v] += 1;
            let root = dsu.get(v);
            if cnt_odds[root] <= 2 {
                cnt_friendly -= 1;
            }
            if sz[v] % 2 == 1 {
                cnt_odds[root] += 1;
            } else {
                cnt_odds[root] -= 1;
            }
            if cnt_odds[root] <= 2 {
                cnt_friendly += 1;
            }
        }
        if dsu.get(p1) != dsu.get(p2) {
            let root1 = dsu.get(p1);
            let root2 = dsu.get(p2);
            if dsu.calc_size(p1) == 1 {
                cnt_size_1 -= 1;
            }
            if dsu.calc_size(p2) == 1 {
                cnt_size_1 -= 1;
            }
            let new_cnt_odds = cnt_odds[root1] + cnt_odds[root2];
            if cnt_odds[root1] <= 2 {
                cnt_friendly -= 1;
            }
            if cnt_odds[root2] <= 2 {
                cnt_friendly -= 1;
            }
            dsu.unite(p1, p2);
            let root = dsu.get(p1);
            cnt_odds[root] = new_cnt_odds;
            if new_cnt_odds <= 2 {
                cnt_friendly += 1;
            }
        }
        out_line!(cnt_friendly - cnt_size_1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
