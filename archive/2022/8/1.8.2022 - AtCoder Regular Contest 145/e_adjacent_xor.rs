//{"name":"E - Adjacent XOR","group":"AtCoder - AtCoder Regular Contest 145","url":"https://atcoder.jp/contests/arc145/tasks/arc145_e","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 2 0\n1 2 3\n","output":"Yes\n2\n2 3\n"},{"input":"2\n10 100\n1 0\n","output":"No\n"},{"input":"2\n1152921504606846975 0\n1152921504606846975 0\n","output":"Yes\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAdjacentXOR"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable4, RecursiveFunction4};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input) {
    let n = input.usize();
    let mut a = input.vec::<i64>(n);
    let b = input.vec::<i64>(n);
    let mut ops = vec![];
    const MAX_BITS: usize = 60;

    let mut apply_op = |a: &mut [i64], pos: usize| {
        ops.push(pos);
        for p in (1..=pos).rev() {
            a[p] ^= a[p - 1];
        }
    };

    let mut rnd = Random::new(33);
    for _ in 0..500 {
        let pos = rnd.gen(0..n);
        apply_op(&mut a, pos);
    }

    let mut rec = RecursiveFunction4::new(
        |f, pos: usize, mut need_xor: i64, this_pos: bool, forb_mask: i64| -> bool {
            if this_pos {
                need_xor = b[pos] ^ a[pos];
            }
            if need_xor == 0 {
                return true;
            }
            if pos == 0 {
                return false;
            }
            let prev = a[pos - 1];
            let mut will_use = false;
            let mut next_forb = forb_mask;
            for bit in 0..MAX_BITS {
                if ((1 << bit) & prev) != 0 {
                    if ((1 << bit) & need_xor) != 0 && ((1 << bit) & forb_mask) == 0 {
                        will_use = true;
                        next_forb |= 1 << bit;
                        break;
                    }
                }
            }
            if !will_use {
                apply_op(&mut a, pos);
                if !f.call(pos - 1, need_xor, false, next_forb) {
                    return false;
                }
            } else {
                if !f.call(pos - 1, need_xor ^ prev, false, next_forb) {
                    return false;
                }
            }
            apply_op(&mut a, pos);
            return true;
        },
    );

    for fix_pos in (0..n).rev() {
        if !rec.call(fix_pos, 0, true, 0) {
            out_line!("No");
            return;
        }
    }
    assert_eq!(a, b);
    assert!(ops.len() <= 70_000);
    out_line!("Yes");
    out_line!(ops.len());
    for op in ops.into_iter() {
        out!(op + 1, "");
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
