//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable4, RecursiveFunction4};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Copy, Clone)]
struct Monster {
    sub: i128,
    add: i128,
    id: usize,
}

struct MonsterDesc {
    sub: i128,
    delta_sub: i128,
    add: i128,
    delta_add: i128,
}

#[derive(Clone)]
struct OneDaySolve {
    order: Vec<usize>,
    min_pos: usize,
    min_sum: i128,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.i128();
    let a = gen_vec(n, |_| MonsterDesc {
        sub: input.i128(),
        delta_sub: input.i128(),
        add: input.i128(),
        delta_add: input.i128(),
    });
    let solve_day = |day: i128| {
        let mut a = gen_vec(n, |id| Monster {
            sub: a[id].sub + a[id].delta_sub * day,
            add: a[id].add + a[id].delta_add * day,
            id,
        });
        a.sort_by_key(|m| {
            if m.add > m.sub {
                (0, m.sub)
            } else {
                (1, -m.add)
            }
        });
        let mut min_pos = 0;
        let mut min_sum = std::i128::MAX;
        let mut cur_sum = 0;
        for i in 0..n {
            cur_sum -= a[i].sub;
            if cur_sum < min_sum {
                min_sum = cur_sum;
                min_pos = i;
            }
            cur_sum += a[i].add;
        }
        OneDaySolve {
            order: a.iter().map(|m| m.id).collect(),
            min_pos,
            min_sum: -min_sum,
        }
    };
    let mut res = 0u64;
    RecursiveFunction4::new(
        |f, fr: i128, to: i128, sol_fr: OneDaySolve, sol_to: OneDaySolve| {
            if fr + 1 == to || (sol_fr.order == sol_to.order && sol_fr.min_pos == sol_to.min_pos) {
                let mut cnt = to - fr;
                let delta = sol_to.min_sum - sol_fr.min_sum;
                assert!(delta % cnt == 0);
                let mut sum = sol_fr.min_sum + sol_to.min_sum - delta / cnt;
                if cnt % 2 == 0 {
                    cnt /= 2;
                } else {
                    sum /= 2;
                }
                let cur_res = (cnt * sum) as u64;
                res = res.overflowing_add(cur_res).0;
            } else {
                let mid = (fr + to) / 2;
                let sol_mid = solve_day(mid);
                f.call(fr, mid, sol_fr, sol_mid.clone());
                f.call(mid, to, sol_mid, sol_to);
            }
        },
    )
    .call(0, m + 1, solve_day(0), solve_day(m + 1));
    out_line!(res);
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
