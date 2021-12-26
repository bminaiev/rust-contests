//{"name":"C - Almost Sorted","group":"AtCoder - AtCoder Regular Contest 132","url":"https://atcoder.jp/contests/arc132/tasks/arc132_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n3 -1 1 -1\n","output":"2\n"},{"input":"5 1\n2 3 4 5 -1\n","output":"0\n"},{"input":"16 5\n-1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1 -1\n","output":"794673086\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAlmostSorted"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input) {
    let n = input.usize();
    let d = input.usize();
    let a: Vec<i32> = input.read_vec(n);
    let mut can_use = vec![true; n];
    for &x in a.iter() {
        if x != -1 {
            can_use[(x - 1) as usize] = false;
        }
    }
    can_use.resize(can_use.len() + d + 1, false);

    let cnt = (2 * d + 1) as usize;
    let mut dp = vec![Mod::ZERO; 1 << cnt];
    let mut start_mask = 0;
    for x in 0..=d {
        if can_use[x] {
            start_mask |= 1 << (x + d);
        }
    }
    dp[start_mask] = Mod::ONE;
    for value in 0..n {
        let mut ndp = vec![Mod::ZERO; 1 << cnt];
        for (mask, &cur) in dp.iter().enumerate() {
            let mut nmask = mask;
            if can_use[value + d + 1] {
                nmask |= 1 << cnt;
            }
            if a[value] == -1 {
                for use_it in 0..cnt {
                    if (1 << use_it) & mask != 0 {
                        let nmask = nmask ^ (1 << use_it);
                        if nmask & 1 == 0 {
                            ndp[nmask >> 1] += cur;
                        }
                    }
                }
            } else if nmask & 1 == 0 {
                ndp[nmask >> 1] += cur;
            }
        }

        dp = ndp;
    }
    out_line!(dp[0].to_string());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
