//{"name":"B. Отрезок и разбиение","group":"Codeforces - Codeforces Round #768 (Div. 1)","url":"https://codeforces.com/contest/1630/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 1\n1 2\n4 2\n1 2 2 2\n11 3\n5 5 5 1 5 5 1 5 5 5 1\n","output":"1 2\n1 2\n2 2\n1 3\n4 4\n5 5\n1 1\n2 2\n3 11\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOtrezokIRazbienie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let a = input.read_vec::<usize>(n).sub_from_all(1);
    let mut a_with_positions = gen_vec(n, |pos| (a[pos], pos));
    a_with_positions.sort();

    let find_x_y = |cnt: usize| -> Option<(usize, usize)> {
        let mut it = 0;
        let mut sum = (n as i32) * -1;
        for start in 0..n {
            let x = a_with_positions[start].0;
            while it != n && x + cnt > a_with_positions[it].0 {
                it += 1;
                sum += 2;
            }
            if start != 0 {
                sum -= 2;
            }
            if sum >= k as i32 {
                return Some((x, x + cnt));
            }
        }
        return None;
    };

    let len = binary_search_first_true(0..n + 1, |cnt| -> bool { find_x_y(cnt).is_some() });
    let (x, y) = find_x_y(len).unwrap();
    out_line!(x + 1, y);
    let mut parts = vec![0];
    let mut cur_sum = 0;
    for i in 0..n {
        let delta = if a[i] >= x && a[i] < y { 1 } else { -1 };
        cur_sum += delta;
        if cur_sum > 0 {
            cur_sum = 0;
            parts.push(i + 1);
        }
    }
    parts.truncate(k);
    parts.push(n);
    for w in parts.windows(2) {
        out_line!(w[0] + 1, w[1]);
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
