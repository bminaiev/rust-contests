//{"name":"J. Regular String","group":"Codeforces - Testing Constructor Cup","url":"https://codeforces.com/gym/503340/problem/J","interactive":false,"timeLimit":3500,"tests":[{"input":"6 2\nbaobab\n","output":"2\n"},{"input":"3 2\nuwu\n","output":"1\n"},{"input":"6 3\nbaobab\n","output":"3\n"},{"input":"13 5\ncjacjbcjccjde\n","output":"4\n"},{"input":"5 1\neeeee\n","output":"2\n"},{"input":"10 0\ncodeforces\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JRegularString"}}}

use std::collections::BTreeSet;
use std::mem;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::strings::suffix_array::SuffixArray;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let to_remove = input.usize();
    let mut res = 1;
    let s = input.string();
    let sa = SuffixArray::new(s);
    let mut sets = vec![BTreeSet::new(); n];
    for i in 0..n {
        sets[i].insert(sa[i + 1]);
    }
    let mut dsu = Dsu::new(n);
    let mut lcps = vec![];
    for i in 0..(n - 1) {
        let lcp = sa.lcp[i + 1] as usize;
        lcps.push((lcp, i));
    }
    lcps.sort();
    lcps.reverse();
    let mut lcps_it = 0;
    let mut cached = vec![usize::MAX; n];
    for part_len in (1..=n).rev() {
        while lcps_it < lcps.len() && lcps[lcps_it].0 >= part_len {
            let (_lcp, i) = lcps[lcps_it];
            let left = dsu.get(i);
            let right = dsu.get(i + 1);
            dsu.unite(i, i + 1);
            if dsu.get(i) == left {
                let mut tmp_set = BTreeSet::new();
                mem::swap(&mut tmp_set, &mut sets[right]);
                sets[left].append(&mut tmp_set);
            } else {
                let mut tmp_set = BTreeSet::new();
                mem::swap(&mut tmp_set, &mut sets[left]);
                sets[right].append(&mut tmp_set);
            }
            cached[dsu.get(i)] = usize::MAX;
            lcps_it += 1;
        }
        if (n - to_remove) % part_len != 0 {
            continue;
        }
        let mut i = 0;
        while i != n {
            let mut j = i;
            while j != n && dsu.get(j) == dsu.get(i) {
                j += 1;
            }
            let num_parts = (n - to_remove) / part_len;
            if j - i >= num_parts {
                let root = dsu.get(i);
                if cached[root] == usize::MAX {
                    let mut cnt = 0;
                    let mut ok_from = 0;
                    for &p in sets[root].iter() {
                        if p >= ok_from {
                            cnt += 1;
                            ok_from = p + part_len + 1;
                        }
                    }
                    cached[root] = cnt;
                }
                if cached[root] >= num_parts {
                    res = num_parts;
                }
            }
            i = j;
        }
    }
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "j_regular_string";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
