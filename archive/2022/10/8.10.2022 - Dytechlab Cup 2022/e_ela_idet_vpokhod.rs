//{"name":"E. Эла идет в поход","group":"Codeforces - Dytechlab Cup 2022","url":"https://codeforces.com/contest/1737/problem/E","interactive":false,"timeLimit":2500,"tests":[{"input":"3\n4\n5\n2\n","output":"0\n250000002\n250000002\n500000004\n0\n250000002\n250000002\n250000002\n250000002\n0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EElaIdetVPokhod"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod7;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

fn solve2(n: usize) -> Vec<Mod> {
    let pw2 = Mod::gen_powers(Mod::TWO, n + 2);
    let mut pr_winning = vec![Mod::ZERO; n + 1];
    pr_winning[n] = Mod::ONE;
    let mut suf = vec![Mod::ZERO; n + 1];
    let inv2 = Mod::ONE / Mod::TWO;
    suf[n] = Mod::ONE;
    for sz in (1..n).rev() {
        pr_winning[sz] = suf[sz + 1] * inv2;
        if sz + 1 == n {
            pr_winning[sz] += suf[sz + 1] * inv2;
        }
        {
            let nsz = sz * 2;
            if nsz < n {
                pr_winning[sz] -= suf[nsz] / pw2[sz];
            } else if nsz == n {
                let h = pr_winning[n] / pw2[sz - 1];
                pr_winning[sz] -= h;
            }
        }
        suf[sz] = suf[sz + 1] * inv2 + pr_winning[sz];
        if sz + 1 == n {
            let h = suf[sz + 1] * inv2;
            suf[sz] += h;
        }
    }
    let mut res = vec![Mod::ZERO; n];
    let mut sz_last = vec![Mod::ZERO; n + 1];
    sz_last[0] = Mod::ONE;
    // for sz in 0..n {
    //     let mut pr_here = sz_last[sz];
    //     for next_sz in sz + 1..=n {
    //         if next_sz != n {
    //             pr_here /= Mod::TWO;
    //         }
    //         sz_last[next_sz] += pr_here;
    //         if next_sz - sz >= sz {
    //             res[next_sz - 1] += pr_here * pr_winning[next_sz];
    //         }
    //     }
    // }

    let mut pref = vec![Mod::ZERO; n + 1];
    pref[0] = sz_last[0];
    for next_sz in 1..=n {
        sz_last[next_sz] = pref[next_sz - 1] * inv2;
        {
            let psz = next_sz / 2;
            // res[next_sz - 1] += pref[next_sz - 1];
            res[next_sz - 1] += pref[psz] / pw2[next_sz - psz]
        }
        // for sz in 0..next_sz {
        //     let mut pr_here = sz_last[sz];
        //     if next_sz - sz >= sz {
        //         res[next_sz - 1] += pr_here / pw2[next_sz - sz - 1];
        //     }
        // }
        pref[next_sz] = pref[next_sz - 1] * inv2;
        pref[next_sz] += sz_last[next_sz];
        res[next_sz - 1] *= pr_winning[next_sz];
    }
    let mut sum = Mod::ZERO;
    for i in 0..(n - 1) {
        sum += res[i];
    }
    res[n - 1] = Mod::ONE - sum;
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    if true {
        out_line!(solve2(n));
        return;
    }
    let mut res = vec![Mod::ZERO; n];
    let pw2 = Mod::gen_powers(Mod::TWO, n + 2);
    // 1 - goes left
    for mask in 0..(1 << n) {
        if ((1 << (n - 1)) & mask) == 0 {
            continue;
        }
        let mut cnt_left = 0;
        let mut more = 0;
        let mut who = 0;
        for i in 0..n {
            if ((1 << i) & mask) != 0 {
                let right_side = more + 1;
                if right_side >= cnt_left {
                    cnt_left = cnt_left + right_side;
                    who = i;
                } else {
                    cnt_left += more + 1;
                }
                more = 0;
            } else {
                more += 1;
            }
        }
        res[who] += Mod::ONE / pw2[n - 1];
    }
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
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
