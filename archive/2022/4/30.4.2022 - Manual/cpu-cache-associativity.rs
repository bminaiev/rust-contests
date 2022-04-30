//{"name":"cpu-cache-associativity","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"cpu-cache-associativity"}}}

use std::time::Instant;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    #[cfg(target_feature = "avx")]
    panic!("have_avx");

    let n = 1_000_000;
    let mut rnd = Random::new(787788);
    let a = gen_vec(n / 2, |_| {
        [rnd.gen_in_range(0..100i64), rnd.gen_in_range(0..100i64)]
    });
    let expected_sum: i64 = a.iter().map(|x| x[0] + x[1]).sum();
    dbg!(expected_sum);
    let start = Instant::now();
    for _ in 0..10000 {
        let my_sum = a
            .iter()
            .fold([0, 0], |sum, new| [sum[0] + new[0], sum[1] + new[1]]);
        // let mut my_sum = i64x4::default();
        // for x in a.vectorize() {
        //     my_sum += x;
        // }
        // let my_sum = my_sum.horizontal_sum();
        let my_sum = my_sum[0] + my_sum[1];
        assert_eq!(my_sum, expected_sum);
    }
    dbg!(start.elapsed());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    // input.skip_whitespace();
    // input.peek().is_none()
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_locally();
    // tester::run_str
}
//END MAIN
