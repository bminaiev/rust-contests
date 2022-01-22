//{"name":"C - Row Column Sums","group":"AtCoder - AtCoder Regular Contest 133","url":"https://atcoder.jp/contests/arc133/tasks/arc133_c","interactive":false,"timeLimit":2000,"tests":[{"input":"2 4 3\n0 2\n1 2 2 0\n","output":"11\n"},{"input":"3 3 4\n0 1 2\n1 2 3\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRowColumnSums"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, dbg};
use algo_lib::math::modulo::ModWithValue;

// WA ...

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let k = input.i64();
    let by_row = input.read_vec::<i64>(n);
    let by_col = input.read_vec::<i64>(m);


    let rows_sum = by_row.iter().sum::<i64>() % k;
    let cols_sum = by_col.iter().sum::<i64>() % k;
    if rows_sum != cols_sum {
        out_line!(-1);
        return;
    }
    if n == 1 {
        let res : i64 = by_col.iter().sum();
        out_line!(res);
        return;
    }
    if m == 1 {
        let res : i64 = by_row.iter().sum();
        out_line!(res);
        return;
    }
    let mut res = ((n - 1) as i64 * (m - 1) as i64) * (k - 1);
    let mut sum_first_column= 0;
    for row in 1..n {
        let total = ((m - 1) as i64) * (k - 1) % k;
        let rem = (by_row[row] + k - total) % k;
        sum_first_column += rem;
        res += rem;
    }
    sum_first_column %= k;
    res += (by_col[0] + k - sum_first_column) % k;
    for col in 1..m {
        let total = ((n - 1) as i64) * (k - 1) % k;
        let rem = (by_col[col] + k - total) % k;
        res += rem;
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
