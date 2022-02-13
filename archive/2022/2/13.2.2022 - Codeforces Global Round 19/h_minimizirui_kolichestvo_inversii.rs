//{"name":"H. Минимизируй количество инверсий","group":"Codeforces - Codeforces Global Round 19","url":"https://codeforces.com/contest/1637/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1\n1\n4\n4 2 1 3\n5\n5 1 3 2 4\n","output":"0 0\n4 2 2 1 4\n5 4 2 2 1 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HMinimiziruiKolichestvoInversii"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.read_vec::<usize>(n).sub_from_all(1);
    let mut answer = 0;
    let mut profit = vec![0; n];
    {
        let mut fenw = Fenwick::new(n);
        for (pos, &val) in a.iter().enumerate() {
            let smaller = fenw.get_sum(val) as i32;
            let bigger = fenw.get_suffix_sum(val) as i32;
            answer += bigger as i64;
            profit[pos] = bigger - smaller;
            fenw.add(val, 1);
        }
    }
    {
        let mut fenw_back = Fenwick::new(n);
        for (pos, &val) in a.iter().enumerate().rev() {
            let smaller = fenw_back.get_sum(val) as i32;
            profit[pos] -= 2 * smaller;
            fenw_back.add(val, 1);
        }
    }
    out!(answer, "");
    let mut order = gen_vec(n, id);
    order.sort_by_key(|&id| -profit[id]);
    for (iter, &best_pos) in order.iter().enumerate() {
        answer -= (profit[best_pos] as i64) + (iter as i64);
        out!(answer, "");
    }
    out_line!();
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
