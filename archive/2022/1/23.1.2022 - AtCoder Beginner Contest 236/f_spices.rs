//{"name":"F - Spices","group":"AtCoder - AtCoder Beginner Contest 236","url":"https://atcoder.jp/contests/abc236/tasks/abc236_f","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 5 3\n","output":"7\n"},{"input":"4\n9 7 9 7 10 4 3 9 4 8 10 5 6 3 8\n","output":"15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSpices"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Elem {
    cost: i64,
    xor: usize,
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let mut elems = gen_vec((1 << n) - 1, |id| Elem {
        cost: input.i64(),
        xor: id + 1,
    });
    let mut basis: Vec<Option<usize>> = vec![None; n];
    elems.sort();
    let mut res = 0;
    for elem in elems.iter() {
        let mut my_xor = elem.xor;
        for bit in 0..n {
            if ((1 << bit) & my_xor) != 0 {
                if let Some(another) = basis[bit] {
                    my_xor ^= another;
                } else {
                    basis[bit] = Some(my_xor);
                    res += elem.cost;
                    break;
                }
            }
        }
    }
    out_line!(res);
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
