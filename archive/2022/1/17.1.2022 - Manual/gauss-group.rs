//{"name":"gauss-group","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"gauss-group"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::gauss::gauss;
use algo_lib::math::modulo::{ConstValue, ModWithValue};
use algo_lib::{dbg, out, out_line};

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct Value_2();
impl ConstValue for Value_2 {
    const VAL: i32 = 2;
}
#[allow(non_camel_case_types)]
pub type Mod = ModWithValue<Value_2>;

fn solve(input: &mut Input, _test_case: usize) {
    let n = 15;
    let mut a = Array2D::new(Mod::ZERO, n, n);
    for row in 0..n {
        let cnt_ones = (n + 1) / 2 - (if row == n - 1 { 1 } else { 0 });
        let start = row;
        for column in start..start + cnt_ones {
            a[row][column % n] = Mod::ONE;
        }
    }
    for row in 0..n {
        for col in 0..n {
            out!(a[row][col]);
        }
        out_line!();
    }
    let sz = gauss(&mut a);
    dbg!(sz);
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
    tester::run_single_test("1");
}
//END MAIN
