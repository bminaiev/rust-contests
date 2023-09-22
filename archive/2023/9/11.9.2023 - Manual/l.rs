//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::index_of::IndexOf;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Check {
    base: usize,
    same: usize,
    bigger: usize,
}

#[derive(Clone)]
struct Pos {
    score_delta: i32,
}

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let word = vec![b'I', b'C', b'P'];
    let s: Vec<_> = input
        .string()
        .iter()
        .map(|c| word.index_of(c).unwrap())
        .collect();
    let mut cnt = [0i32; 3];
    let checks = [
        Check {
            base: 0,
            same: 1,
            bigger: 2,
        },
        Check {
            base: 0,
            same: 2,
            bigger: 1,
        },
        Check {
            base: 1,
            same: 2,
            bigger: 0,
        },
    ];
    let shift = s.len() as i32;
    let mut positions: Array2D<Vec<Pos>> = Array2D::new(vec![], checks.len(), s.len() * 2 + 1);
    for i in 0..=s.len() {
        for (c_it, check) in checks.iter().enumerate() {
            let same_delta = (cnt[check.same] - cnt[check.base] + shift) as usize;
            let score_delta = cnt[check.bigger] - cnt[check.base];
            positions[c_it][same_delta].push(Pos { score_delta })
        }
        if i != s.len() {
            cnt[s[i]] += 1;
        }
    }
    for i in 0..checks.len() {
        for j in 0..positions[i].len() {
            positions[i][j].sort_by_key(|p| p.score_delta);
        }
    }
    let mut fenw = Array2D::new_f(checks.len(), positions[0].len(), |i, j| {
        Fenwick::<Mod>::new(positions[i][j].len())
    });
    for x in cnt.iter_mut() {
        *x = 0;
    }
    for i in 0..=s.len() {
        let mut ways = Mod::ZERO;
        for (c_it, check) in checks.iter().enumerate() {
            let same_delta = (cnt[check.same] - cnt[check.base] + shift) as usize;
            let score_delta = cnt[check.bigger] - cnt[check.base];
            let pos = binary_search_first_true(0..positions[c_it][same_delta].len(), |idx| {
                positions[c_it][same_delta][idx].score_delta >= score_delta
            });
            if pos > 0 {
                ways += fenw[c_it][same_delta].get_sum(pos - 1);
            }
        }
        if i == 0 {
            ways += Mod::ONE;
        }
        for (c_it, check) in checks.iter().enumerate() {
            let same_delta = (cnt[check.same] - cnt[check.base] + shift) as usize;
            let score_delta = cnt[check.bigger] - cnt[check.base];
            let pos = binary_search_first_true(0..positions[c_it][same_delta].len(), |idx| {
                positions[c_it][same_delta][idx].score_delta >= score_delta
            });
            fenw[c_it][same_delta].add(pos, ways);
        }
        if i != s.len() {
            cnt[s[i]] += 1;
        } else {
            out_line!(ways);
        }
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
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
