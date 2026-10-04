use std::collections::BTreeSet;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::fenwick::Fenwick;

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let p = input.vec::<usize>(n).sub_from_all(1);
        let mut where_it = vec![0; n];
        for i in 0..n {
            where_it[p[i]] = i;
        }
        let mut alive = BTreeSet::new();
        let mut right = vec![n; n];
        for value in (0..n).rev() {
            let pos = where_it[value];
            let mut it = alive.range(pos..);
            if let Some(&next) = it.next() {
                right[pos] = next;
            }
            alive.insert(pos);
        }
        let mut res = 0i64;
        let mut suf_cnt = vec![0i64; n];
        let mut suf_sum = vec![0i64; n];
        let mut cnt_seen = vec![0; n];
        for pos in 0..n {
            if right[pos] != n {
                cnt_seen[right[pos]] += 1;
            }
        }
        let mut fenw = Fenwick::<i64>::new(n);
        for v in 0..n {
            if cnt_seen[v] > 0 {
                fenw.add(v, 1);
            }
        }
        let mut it = 0;
        for pos in (0..n).rev() {
            while it > pos && cnt_seen[it] == 0 {
                it -= 1;
            }
            {
                // go left
                res += pos as i64;
            }
            let mut check_from = pos + 1;

            let mut go_to = n;
            let mut go_to_len = 0;

            let mut mit = it;
            if right[pos] != n {
                go_to = right[pos];
                go_to_len = 1;

                // just it
                suf_sum[pos] += 1;
                suf_cnt[pos] += 1;

                // [pos + 1, right[pos])
                let inside = (right[pos] - pos - 1) as i64;
                suf_sum[pos] += 2 * inside;
                suf_cnt[pos] += inside;

                check_from = right[pos] + 1;

                {
                    let zz = right[right[pos]];
                    if zz != n && zz > mit {
                        mit = zz;
                    }
                }
            }

            if mit > pos && mit > check_from {
                go_to = mit;
                go_to_len = 2;

                let total_here = (mit + 1 - check_from) as i64;
                let mut reachable_by_two = fenw.get_range_sum(check_from..mit + 1);

                {
                    if right[pos] != n && right[right[pos]] <= mit {
                        let zz = right[right[pos]];
                        if fenw.get_range_sum(zz..zz + 1) == 0 {
                            reachable_by_two += 1;
                        }
                    }
                }

                suf_sum[pos] += reachable_by_two * 2 + (total_here - reachable_by_two) * 3;
                suf_cnt[pos] += total_here;
            }

            if go_to != n {
                suf_sum[pos] += suf_sum[go_to] + suf_cnt[go_to] * go_to_len;
                suf_cnt[pos] += suf_cnt[go_to];
            }

            res += suf_sum[pos];

            if right[pos] != n {
                cnt_seen[right[pos]] -= 1;
                if cnt_seen[right[pos]] == 0 {
                    fenw.add(right[pos], -1);
                }
            }
        }
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "d_aghabalasar_";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
