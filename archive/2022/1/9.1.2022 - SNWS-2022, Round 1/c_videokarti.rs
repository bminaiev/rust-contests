//{"name":"C. Видеокарты","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/C/","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n18 73 98\n3 2 1\n","output":"190\n"},{"input":"4\n10 8 36 6\n2 4 3 1\n","output":"120\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVideokarti"}}}

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let cnt: Vec<i64> = input.read_vec(n);
    let remove: Vec<usize> = input.read_vec(n).sub_from_all(1);
    let mut res = 0;
    let mut alive = vec![false; n];

    let mut dsu = Dsu::new(n);
    let mut cur_sum = 0;
    let mut cnt_chains = 0;

    let mut sum_root: Vec<_> = cnt.clone();

    fn up(x: i64) -> i64 {
        (x + 9) / 10 * 10
    }

    for &add_pos in remove.iter().rev() {
        cur_sum += up(sum_root[add_pos]);
        cnt_chains += 1;

        let mut join = |another: usize| {
            let id1 = dsu.get(another);
            let id2 = dsu.get(add_pos);
            let sum1 = sum_root[id1];
            let sum2 = sum_root[id2];
            cur_sum -= up(sum1);
            cur_sum -= up(sum2);
            dsu.unite(id1, id2);
            let new_root = dsu.get(id1);
            sum_root[new_root] = sum1 + sum2;
            cur_sum += up(sum_root[new_root]);
            cnt_chains -= 1;
        };
        if add_pos > 0 && alive[add_pos - 1] {
            join(add_pos - 1);
        }
        if add_pos + 1 < n && alive[add_pos + 1] {
            join(add_pos + 1);
        }
        alive[add_pos] = true;
        res.update_max(cnt_chains * cur_sum);
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
    tester::run_tests();
}
//END MAIN
