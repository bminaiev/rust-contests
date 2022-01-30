//{"name":"C. Concentration For Two","group":"Yandex - SNWS-2022, Round 5","url":"https://contest.yandex.ru/snws2022/contest/23961/problems/C/","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CConcentrationForTwo"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut cards: Vec<Option<usize>> = vec![None; n * 2];
    let mut free = gen_vec(n, id);
    let mut duplicates_free = vec![];
    for _ in 0..2 * n - 1 {
        let pos = input.usize() - 1;
        if let Some(x) = cards[pos] {
            out_line!(x + 1);
        } else {
            if let Some(x) = free.pop() {
                cards[pos] = Some(x);
                out_line!(x + 1);

                duplicates_free.push(cards[pos].unwrap());
            } else {
                let x = duplicates_free.pop().unwrap();
                cards[pos] = Some(x);
                out_line!(x + 1);
            }
        }
        output().flush();
        let old_card = cards[pos].unwrap();
        let pos2 = input.usize() - 1;
        let next_card = if let Some(x) = cards[pos2] {
            x
        } else {
            if !duplicates_free.is_empty() && *duplicates_free.last_exn() != old_card {
                let res = *duplicates_free.last_exn();
                duplicates_free.pop();
                res
            } else if duplicates_free.len() >= 2
                && duplicates_free[duplicates_free.len() - 2] != old_card
            {
                let res = duplicates_free[duplicates_free.len() - 2];
                duplicates_free.remove(duplicates_free.len() - 2);
                res
            } else {
                if let Some(x) = free.pop() {
                    duplicates_free.push(x);
                    x
                } else {
                    duplicates_free.pop().unwrap()
                }
            }
        };
        cards[pos2] = Some(next_card);
        out_line!(next_card + 1);
        output().flush();
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
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_locally();
}
//END MAIN
