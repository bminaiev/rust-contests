//{"name":"heroes_slow","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"heroes_slow"}}}

use std::collections::HashMap;

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let damage = input.i64();
    let health = input.i64();
    let a = input.vec::<i64>(n);

    let mut cache = HashMap::new();
    let mut dfs = RecursiveFunction::new(|f, mut h: Vec<i64>| -> i64 {
        h.sort();
        if h.is_empty() || *h.last_exn() == 0 {
            return 0;
        }
        if let Some(&res) = cache.get(&h) {
            return res;
        }
        let mut res = std::i64::MAX;
        for pos in 0..h.len() {
            if h[pos] > 0 {
                let mut nh = h.clone();
                nh[pos] -= damage;
                nh[pos].update_max(0);
                let alive = nh.iter().map(|&x| (x + health - 1) / health).sum::<i64>();
                res.update_min(alive + f.call(nh));
            }
        }
        cache.insert(h, res);
        return res;
    });
    let res = dfs.call(a.into_iter().map(|cnt| cnt * health).collect()) + 1;
    out_line!(res);
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
