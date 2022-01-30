//{"name":"A. Automatic Ticket Control","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/?nc=A4PYsqrj","interactive":false,"timeLimit":2000,"tests":[{"input":"3 9\n4\nsnws\n5\n","output":"snwy\nsnxd\nsnxm\nsnxv\nsnya\n"},{"input":"1083 600\n6\nzzzzyz\n10\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAutomaticTicketControl"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a = input.i64();
    let b = input.i64();
    let n = input.usize();
    let start = input.string_as_vec();
    let need_cnt = input.usize();
    let mut seen = vec![vec![]; b as usize];
    // (pos, cur_hash, can_use_all)
    let mut iter = 0;
    if let Some(res) = RecursiveFunction3::new(|f, pos, hash, can_use_all| {
        if pos == n {
            let hash = hash as usize;
            seen[hash].push(iter);
            if seen[hash].len() >= need_cnt {
                return Some(hash);
            }
            iter += 1;
            None
        } else {
            let start = if can_use_all { 0 } else { start[pos] - b'a' };
            for x in start..26 {
                if let Some(hash) = f.call(
                    pos + 1,
                    (hash * a + (x as i64)) % b,
                    can_use_all || x > start,
                ) {
                    return Some(hash);
                }
            }
            None
        }
    })
    .call(0, 0, false)
    {
        iter = 0;
        seen[res].clear();

        let mut str = vec![];
        RecursiveFunction3::new(|f, pos, hash, can_use_all| {
            if pos == n {
                let hash = hash as usize;
                seen[hash].push(iter);
                if hash == res {
                    out_line!(vec2str(&str));

                    if seen[hash].len() >= need_cnt {
                        return true;
                    }
                }
                iter += 1;
                false
            } else {
                let start = if can_use_all { 0 } else { start[pos] - b'a' };
                for x in start..26 {
                    str.push(x + b'a');
                    if f.call(
                        pos + 1,
                        (hash * a + (x as i64)) % b,
                        can_use_all || x > start,
                    ) {
                        return true;
                    }
                    str.pop();
                }
                false
            }
        })
        .call(0, 0, false);

    } else {
        out_line!(-1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
