//{"name":"C. Cleric","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"7 2\n0.5\n2 3\n3 6\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCleric"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::num_traits::HasConstants;
use algo_lib::misc::ord_f64::OrdF64;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let p = input.f64();
    let mut jump = gen_vec(n + 1, |i| i);
    for _ in 0..k {
        let from = input.usize() - 1;
        let to = input.usize() - 1;
        jump[from] = to;
    }
    let mut go = Array2D::new(OrdF64::ZERO, n, n);
    for start in 0..n {
        for next in start + 1..start + 7 {
            let real_next = if next >= n - 1 { n - 1 } else { jump[next] };
            go[start][real_next] += OrdF64(1.0 / 6.0);
        }
    }

    let res = binary_search_first_true(0..2e8 as usize, |moves| -> bool {
        let init = Array2D::gen(1, n, |r, c| if c == 0 { OrdF64::ONE } else { OrdF64::ZERO });
        let final_probs = &init * &go.pown(moves);

        final_probs[0][n - 1] >= p
    });
    out_line!(res);
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
}
//END MAIN
