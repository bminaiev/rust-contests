//{"name":"I. Space Station","group":"Codeforces - The 2019 ICPC Asia Nanjing Regional Contest","url":"https://codeforces.com/gym/103466/problem/I","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2 1 2 3\n","output":"4\n"},{"input":"5\n1 1 1 1 2 3\n","output":"54\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ISpaceStation"}}}

use std::cmp::max;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::combinations::{Combinations, CombinationsFact};
use algo_lib::math::factorials::gen_facts;
use algo_lib::math::modulo::Mod7;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn gen_all(max_sum: usize) -> Vec<Vec<Vec<usize>>> {
    let mut res = vec![vec![]; max_sum as usize];
    let mut cur = vec![];
    RecursiveFunction2::new(|f, cur_sum: usize, start: usize| {
        res[cur_sum as usize].push(cur.clone());
        if cur_sum + start >= max_sum {
            return;
        }
        f.call(cur_sum, start + 1);
        cur.push(start);
        f.call(cur_sum + start, start);
        cur.pop();
    })
    .call(0, 1);
    for it in res.iter_mut() {
        it.sort();
    }
    res
}

fn stress() {
    let all = gen_all(50);
    dbg!(all.len());
}

type Mod = Mod7;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let start = input.usize();
    let a = input.vec::<usize>(n);
    let max_ai = *a.iter().max().unwrap_or(&0);
    let by_sum = gen_all(max(1, max_ai));
    let mut cnt = vec![0; max_ai + 1];
    for &x in a.iter() {
        cnt[x] += 1;
    }
    let mut dp = gen_vec(by_sum.len(), |sum| vec![Mod::ZERO; by_sum[sum].len()]);
    dp[0][0] = Mod::ONE;
    let n_without_zeros: usize = cnt[1..].iter().sum();

    let facts: Vec<Mod> = gen_facts(n);
    let comb = CombinationsFact::<Mod>::new(n + 1);

    let mut res = Mod::ZERO;
    for sum in 0..dp.len() {
        for i in 0..dp[sum].len() {
            let cur = dp[sum][i];
            if cur == Mod::ZERO {
                continue;
            }
            let real_sum = sum + start;
            let vec = &by_sum[sum][i];
            if real_sum >= max_ai {
                let more_elements = n_without_zeros - vec.len();
                res += cur * facts[more_elements];
            } else {
                for elem in 1..=real_sum {
                    let mut ways = cnt[elem];
                    for &x in vec.iter() {
                        if x == elem {
                            ways -= 1;
                        }
                    }
                    if ways != 0 {
                        let ncur = cur * Mod::new(ways);
                        let new_real_sum = real_sum + elem;
                        if new_real_sum >= max_ai {
                            let more_elements = n_without_zeros - vec.len() - 1;
                            res += ncur * facts[more_elements];
                        } else {
                            // save...
                            let mut to_find = vec.clone();
                            to_find.push(elem);
                            to_find.sort();
                            let nsum = sum + elem;
                            let idx = by_sum[nsum].binary_search(&to_find).unwrap();
                            dp[nsum][idx] += ncur;
                        }
                    }
                }
            }
        }
    }
    res *= comb.c(n, cnt[0]);
    res *= facts[cnt[0]];
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
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
