//{"name":"D. Возвращение робота-пылесоса","group":"Codeforces - Codeforces Round #763 (Div. 2)","url":"http://codeforces.com/contest/1623/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 2 1 1 2 1 25\n3 3 1 2 2 2 25\n10 10 1 1 10 10 75\n10 10 10 10 1 1 75\n5 5 1 3 2 2 10\n97 98 3 5 41 43 50\n","output":"3\n3\n15\n15\n332103349\n99224487\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DVozvrashchenieRobotaPilesosa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod7;
use algo_lib::{dbg, out, out_line};

type Mod = Mod7;

struct RobotRow {
    cur: i32,
    delta: i32,
    max: i32,
}

impl RobotRow {
    fn make_move(&mut self) {
        if self.cur + self.delta > self.max || self.cur + self.delta < 1 {
            self.delta *= -1;
        }
        self.cur += self.delta;
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.i32();
    let m = input.i32();
    let x1 = input.i32();
    let y1 = input.i32();
    let x2 = input.i32();
    let y2 = input.i32();
    let p = Mod::new(input.i32()) / Mod::new(100);
    let mut r1 = RobotRow {
        cur: x1,
        delta: 1,
        max: n,
    };
    let mut r2 = RobotRow {
        cur: y1,
        delta: 1,
        max: m,
    };
    let cycle_len = (n - 1) * (m - 1) * 2;
    let mut prob_alive = Mod::ONE;
    let mut ev = Mod::ZERO;
    for move_id in 0..cycle_len {
        if r1.cur == x2 || r2.cur == y2 {
            ev += p * prob_alive * Mod::new(move_id);
            prob_alive *= Mod::ONE - p;
        }
        r1.make_move();
        r2.make_move();
    }
    let ans = (ev + prob_alive * Mod::new(cycle_len)) / (Mod::ONE - prob_alive);
    out_line!(ans);
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
}
//END MAIN
