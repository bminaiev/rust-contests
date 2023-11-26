//{"name":"C. Сопоставление массивов","group":"Codeforces - CodeTON Round 7 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1896/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1 0\n1\n2\n1 1\n1\n2\n3 0\n2 4 3\n4 1 2\n3 1\n2 4 3\n4 1 2\n3 2\n2 4 3\n4 1 2\n3 3\n2 4 3\n4 1 2\n5 2\n6 4 5 6 2\n9 7 9 1 1\n","output":"YES\n2\nNO\nNO\nYES\n2 4 1\nYES\n4 1 2\nNO\nYES\n1 9 9 7 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSopostavlenieMassivov"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let x = input.usize();
    let a = input.vec::<i32>(n);
    let mut b = input.vec::<i32>(n);
    b.sort();
    let mut sorted_ids = gen_vec(n, |i| i);
    sorted_ids.sort_by_key(|&i| a[i]);
    let mut b_try = vec![0; n];
    for i in 0..x {
        let j = (n - x) + i;
        let j = sorted_ids[j];
        b_try[j] = b[i];
    }
    for i in x..n {
        let j = i - x;
        let j = sorted_ids[j];
        b_try[j] = b[i];
    }
    let mut real_x = 0;
    for i in 0..n {
        if a[i] > b_try[i] {
            real_x += 1;
        }
    }
    if x == real_x {
        out_line!("YES");
        out_line!(b_try);
    } else {
        out_line!("NO");
    }
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
