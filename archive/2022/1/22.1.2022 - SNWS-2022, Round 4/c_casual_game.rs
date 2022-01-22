//{"name":"C. Casual Game","group":"Yandex - SNWS-2022, Round 4","url":"https://contest.yandex.ru/snws2022/contest/23960/problems/C/","interactive":false,"timeLimit":4000,"tests":[{"input":"9 21\n12 20 12 13 12 20 16 14 15\n2 4\n1 77\n2 64\n","output":"758646864\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCasualGame"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::math::ntt::NTT;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::pref_sum::PrefSum;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

struct Point {
    pos: usize,
    prob_of_change: Mod,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let total_len = input.usize();
    let time_per_person = input.read_vec::<i64>(n);
    let num_points = input.usize();
    let change: i64 = input.read();
    let points = gen_vec(num_points, |_| {
        let pos = input.usize();
        let prob_of_change = Mod::new(input.i32()) / Mod::new(100);
        Point {
            pos,
            prob_of_change,
        }
    });
    let mut polynomials = vec![vec![Mod::ONE]];
    let mut total_shift = 0;
    for i in 0..points.len() {
        let interval_end = if i + 1 == points.len() {
            total_len
        } else {
            points[i + 1].pos
        };
        let interval_len = interval_end - points[i].pos;
        let mut cur_poly = vec![Mod::ZERO; interval_len * 2 + 1];
        total_shift += interval_len;
        let prob_diff = points[i].prob_of_change * (Mod::ONE - points[i].prob_of_change);
        cur_poly[0] = prob_diff;
        cur_poly[interval_len * 2] = prob_diff;
        cur_poly[interval_len] = Mod::ONE - prob_diff * Mod::TWO;
        polynomials.push(cur_poly);
    }
    let overall = NTT::new().multiply_all(polynomials);
    let overall_pref = overall.pref_sum();

    assert_eq!(*overall_pref.last_exn(), Mod::ONE);
    let mut res = Mod::ONE;
    for i in 1..n {
        let my_expected_time = (total_len as i64) * time_per_person[0];
        let their_expected_time = (total_len as i64) * time_per_person[i];
        let first_pos_to_win = binary_search_first_true(0..overall.len(), |pos| -> bool {
            let their_total_time = pos as i64 * change * 2 + their_expected_time;
            let mine_total_time = (total_shift as i64) * change * 2 + my_expected_time;
            mine_total_time <= their_total_time
        });
        res += overall_pref[first_pos_to_win];
    }
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
    tester::run_single_test("1");
}
//END MAIN
