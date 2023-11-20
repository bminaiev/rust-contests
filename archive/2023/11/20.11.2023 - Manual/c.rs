//{"name":"c","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"c"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::seg_trees::fenwick::Fenwick;
use algo_lib::strings::z_function::z_function;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Query {
    r: usize,
    time: usize,
}

#[target_feature(enable = "avx2")]
unsafe fn apply(same_prefix: usize, l: usize, cur_res: &mut [i64], pref_w: &[i64]) {
    for i in 0..same_prefix {
        cur_res[l + i] += pref_w[i + 1];
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let s = input.string();
    let t = input.string();
    let w = input.vec::<i64>(n);
    let pref_w = w.pref_sum();
    let mut queries = vec![vec![]; t.len()];
    for time in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        queries[fr].push(Query { r: to, time })
    }
    let z_function = {
        let mut res = s.clone();
        res.push(b'#');
        res.extend_from_slice(&t);
        z_function(&res)[s.len() + 1..].to_vec()
    };
    let mut res = vec![0; m];
    let mut cur_res = vec![0; n];
    let mut fenw = Fenwick::<i64>::new(t.len());
    for l in (0..t.len()).rev() {
        let same_prefix = z_function[l];
        unsafe {
            apply(same_prefix, l, &mut cur_res, &pref_w);
        }
        fenw.add(l + same_prefix, pref_w[same_prefix]);
        for query in queries[l].iter() {
            res[query.time] = cur_res[query.r] + fenw.get_sum(query.r);
        }
    }
    for &x in res.iter() {
        out_line!(x);
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
