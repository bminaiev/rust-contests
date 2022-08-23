//{"name":"Maximal Prefix After Flip","group":"CodeChef - LTIME111A","url":"https://www.codechef.com/LTIME111A/problems-old/MAXPREFFLIP","interactive":false,"timeLimit":2500,"tests":[{"input":"4\n5\n-3 4 -2 -3 1\n3\n-8 -5 -2\n1\n2\n10\n-10 -8 -4 -12 -19 -13 -17 -1 -10 -1\n","output":"1 7 9 13 13 13\n0 8 13 15\n2 2\n0 10 18 29 45 59 75 84 92 94 95\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaximalPrefixAfterFlip"}}}

use algo_lib::collections::prependable_vector::PrependableVector;
use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::fenwick::Fenwick;
use algo_lib::simd::apply_fast::fast_apply;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[target_feature(enable = "avx2")]
unsafe fn update_ans(res: &mut [i64], prefix_sums: &[i64], cur_sum: i64) {
    for i in 0..prefix_sums.len() {
        if prefix_sums[i] + cur_sum > res[i] {
            res[i] = prefix_sums[i] + cur_sum;
        }
    }
}

fn solve_case(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![0; n + 1];
    let sorted = a
        .iter()
        .enumerate()
        .map(|(pos, &x)| (x, pos))
        .collect::<Vec<_>>()
        .sorted();
    let mut cur_sum = 0;
    let mut prefix_sums = PrependableVector::new(n + 1);
    prefix_sums.push(0);
    let mut present = Fenwick::new(n);
    for (pos, &x) in a.iter().enumerate() {
        cur_sum += x;
        if x < 0 {
            let sorted_pos = sorted.binary_search(&(x, pos)).unwrap();
            let insert_pos = present.get_sum(sorted_pos);
            present.add(sorted_pos, 1);
            prefix_sums.insert(insert_pos + 1, prefix_sums[insert_pos]);
            fast_apply(&mut prefix_sums[insert_pos + 1..], |e| e - x * 2);
        }
        unsafe {
            update_ans(&mut res, &prefix_sums, cur_sum);
        }
    }
    for i in 0..n {
        if res[i] > res[i + 1] {
            res[i + 1] = res[i];
        }
    }
    res
}

fn stress() {
    let n = 100_000;
    let mut a = gen_vec(n, |pos| (pos as i64) * -1);
    solve_case(&a);
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let a = input.vec::<i64>(n);
    let res = solve_case(&a);
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
    // tester::run_stress(stress);
}
//END MAIN
