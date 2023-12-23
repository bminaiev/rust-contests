//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use std::collections::{BTreeMap, BTreeSet};

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    dbg!("RUN TEST");
    let n = input.usize();
    let k = input.i32();
    let a = input.vec::<i32>(n);
    let mut res = vec![];
    let mut map = BTreeMap::<i32, i32>::new();
    map.insert(0, 0);
    map.insert(i32::MAX, i32::MAX);
    let mut extra = 0;
    let mut used = BTreeSet::new();
    for &x in a.iter() {
        used.insert(x);
        let mut new_fr = x;
        let mut new_to = x;
        let (&fr, &to) = map.range(..=x).next_back().unwrap();
        if to >= new_to {
            // already covered
        } else if to + k >= x {
            map.remove(&fr);
            extra -= (to - fr) / k;
            new_fr = fr;
            new_to = to + k;
        }
        loop {
            let (&fr, &to) = map.range(x + 1..).next().unwrap();
            if new_to + k >= fr {
                // TODO: think about it?
                map.remove(&fr);
                extra -= (to - fr) / k;
                let last_to_cover = *used.range(..=to).next_back().unwrap();
                new_to = new_fr + (last_to_cover - new_fr) / k * k;
                if new_to < last_to_cover {
                    new_to += k;
                }
                assert!(new_to >= last_to_cover);
            } else {
                break;
            }
        }
        extra += (new_to - new_fr) / k;
        map.insert(new_fr, new_to);
        res.push(extra + (map.len() - 2) as i32);
        dbg!("AFTER", x);
        dbg!(map);
        dbg!(extra);
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
