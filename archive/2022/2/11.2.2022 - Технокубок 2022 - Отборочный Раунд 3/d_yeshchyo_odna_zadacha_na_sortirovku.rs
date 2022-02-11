//{"name":"D. Ещё одна задача на сортировку","group":"Codeforces - Технокубок 2022 - Отборочный Раунд 3","url":"http://codeforces.com/contest/1585/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n1\n2\n2 2\n2\n2 1\n3\n1 2 3\n3\n2 1 3\n3\n3 1 2\n4\n2 1 4 3\n","output":"YES\nYES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYeshchyoOdnaZadachaNaSortirovku"}}}

use algo_lib::collections::inversions_count;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};
use algo_lib::{io::input::Input, misc::gen_vector::gen_vec};
use algo_lib::{io::output::output, seg_trees::fenwick::Fenwick};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Element {
    elem: i32,
    pos: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |pos| Element {
        pos,
        elem: input.i32(),
    });
    a.sort();
    for w in a.windows(2) {
        if w[0].elem == w[1].elem {
            out_line!("YES");
            return;
        }
    }
    let positions: Vec<_> = a.iter().map(|x| x.pos).collect();
    if inversions_count(&positions) % 2 == 0 {
        out_line!("YES");
    } else {
        out_line!("NO");
    }
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

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
