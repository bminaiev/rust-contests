//{"name":"J. Exam","group":"Yandex - Stage 17: Grand Prix of Seoul","url":"https://official.contest.yandex.com/opencupXXII/contest/39021/problems/J/","interactive":false,"timeLimit":1000,"tests":[{"input":"3 3\n1 2 -5\n-2 3 0\n-1 -1 1\n","output":"2\n"},{"input":"4 3\n1 -1 1 1\n1 1 1 1\n1 1 1 1\n1 1 -1 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JExam"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy)]
struct Res {
    seen_k: bool,
    max_suf: i64,
}

fn gen_res(way: &[i64], k: i64) -> Option<Res> {
    let n = way.len();
    let mut max_suf = std::i64::MIN / 3;
    let mut cur_sum = 0;
    for i in (0..n).rev() {
        cur_sum += way[i];
        max_suf.update_max(cur_sum);
    }
    let mut total_max = std::i64::MIN / 2;
    for i in 0..n {
        let mut cur_sum = 0;
        for &val in &way[i..] {
            cur_sum += val;
            total_max.update_max(cur_sum);
        }
    }
    if total_max > k {
        return None;
    }
    Some(Res {
        seen_k: total_max == k,
        max_suf,
    })
}

fn gen(a: &Array2D<i64>, k: i64) -> Vec<Vec<Res>> {
    let steps = a.len() - 1;
    let mut res = vec![vec![]; steps + 1];
    let mut way = vec![];
    for mask in 0..(1 << steps) {
        let mut x = 0;
        let mut y = 0;
        way.clear();
        for i in 0..steps {
            way.push(a[x][y]);
            if (1 << i) & mask != 0 {
                x += 1;
            } else {
                y += 1;
            }
        }
        way.push(a[x][y]);
        if let Some(r) = gen_res(&way, k) {
            res[x].push(r);
        }
    }

    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.i64();
    let a = input.matrix::<i64>(n, n);
    if n == 1 {
        let res = if a[0][0] == k { 1 } else { 0 };
        out_line!(res);
        return;
    }
    let mut from_top = gen(&a, k);
    let mut rev_a = Array2D::new(0, n, n);
    for x in 0..n {
        for y in 0..n {
            rev_a[x][y] = a[n - 1 - x][n - 1 - y];
        }
    }
    let mut from_bottom = gen(&rev_a, k);

    let mut res = 0;

    for x in 0..n {
        let left = &mut from_top[x];
        let right = &mut from_bottom[n - 1 - x];

        left.sort_by_key(|e| e.max_suf);
        right.sort_by_key(|e| e.max_suf);

        let cur_elem = a[x][n - 1 - x];

        let mut it_need_k = 0;
        let mut it_less_k = 0;
        let mut cnt_less_seen_k = 0;
        for r in left.iter().rev() {
            while it_less_k != right.len() && right[it_less_k].max_suf + r.max_suf - cur_elem < k {
                if right[it_less_k].seen_k {
                    cnt_less_seen_k += 1;
                }
                it_less_k += 1;
            }
            while it_need_k != right.len() && right[it_need_k].max_suf + r.max_suf - cur_elem <= k {
                it_need_k += 1;
            }

            res += (it_need_k - it_less_k) as i64;
            if r.seen_k {
                res += it_less_k as i64;
            } else {
                res += cnt_less_seen_k;
            }
        }
    }

    out_line!(res);
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
    // tester::run_stress(stress);
}
//END MAIN
