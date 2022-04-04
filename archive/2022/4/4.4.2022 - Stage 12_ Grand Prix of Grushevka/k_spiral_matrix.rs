//{"name":"K. Spiral Matrix","group":"Yandex - Stage 12: Grand Prix of Grushevka","url":"https://official.contest.yandex.com/opencupXXII/contest/35268/problems/K/","interactive":false,"timeLimit":4000,"tests":[{"input":"5 7 10\n10 11 12 13 14 15 16\n9 2 3 32 31 30 17\n8 1 4 25 26 29 18\n7 6 5 24 27 28 19\n52 51 50 23 22 21 20\n1 1 5 7\n1 1 4 1\n2 2 5 3\n1 4 5 7\n1 1 4 3\n1 1 5 3\n2 2 2 2\n2 2 2 3\n3 4 5 7\n3 3 4 4\n","output":"NO\nYES\nNO\nYES\nYES\nNO\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"KSpiralMatrix"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::SHIFTS_4;
use algo_lib::iters::shifts_iter::ShiftsIterator;
use algo_lib::misc::pref_sum::PrefSum;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let q = input.usize();
    let a = input.matrix::<i64>(n, m);
    let a2 = Array2D::new_f(n, m, |x, y| {
        let val = a[x][y] as i128;
        val * val
    });
    let pref_sum = a.pref_sum();
    let pref_sum2 = a2.pref_sum();
    let all = a.iter().cloned().collect::<Vec<_>>().sorted();
    let all2 = a2.iter().cloned().collect::<Vec<_>>().sorted().pref_sum();
    let shifts_iter = ShiftsIterator::new(&SHIFTS_4, n, m);
    let mut good = vec![0; all.len()];
    for x in 0..n {
        for y in 0..m {
            let mut ok = false;
            for (nx, ny) in shifts_iter.iter(x, y) {
                if a[nx][ny] == a[x][y] + 1 {
                    ok = true;
                }
            }
            if ok {
                let pos = all.binary_search(&a[x][y]).unwrap();
                good[pos] = 1;
            }
        }
    }
    let good = good.pref_sum();
    for _ in 0..q {
        let x1 = input.usize();
        let y1 = input.usize();
        let x2 = input.usize();
        let y2 = input.usize();
        let sum = pref_sum[x2][y2] - pref_sum[x2][y1 - 1] - pref_sum[x1 - 1][y2]
            + pref_sum[x1 - 1][y1 - 1];
        let cnt = (x2 - x1 + 1) * (y2 - y1 + 1);
        let ok = || -> bool {
            if let Some(start) = calc_start(sum, cnt as i64) {
                if let Ok(pos) = all.binary_search(&start) {
                    if pos + cnt <= all.len() && all[pos + cnt - 1] == start + (cnt as i64) - 1 {
                        let cnt_good = good[pos + cnt - 1] - good[pos];
                        if cnt_good == cnt - 1 {
                            let expected_sum2 = all2[pos + cnt] - all2[pos];
                            let my_sum2 =
                                pref_sum2[x2][y2] - pref_sum2[x2][y1 - 1] - pref_sum2[x1 - 1][y2]
                                    + pref_sum2[x1 - 1][y1 - 1];
                            return expected_sum2 == my_sum2;
                        }
                    }
                }
            }
            false
        };
        if ok() {
            out_line!("YES");
        } else {
            out_line!("NO");
        }
    }
}

fn calc_start(sum: i64, cnt: i64) -> Option<i64> {
    if sum * 2 % cnt != 0 {
        return None;
    }
    let two_start = (sum * 2) / cnt + 1 - cnt;
    if two_start % 2 != 0 {
        return None;
    }
    let start = two_start / 2;
    assert_eq!(cnt * (start + start + cnt - 1) / 2, sum);
    Some(start)
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
