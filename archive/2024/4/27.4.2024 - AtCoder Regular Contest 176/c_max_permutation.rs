//{"name":"C - Max Permutation","group":"AtCoder - AtCoder Regular Contest 176","url":"https://atcoder.jp/contests/arc176/tasks/arc176_c","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 2 4\n2 3 2\n","output":"2\n"},{"input":"6 3\n1 4 3\n2 5 6\n3 4 2\n","output":"8\n"},{"input":"20 17\n9 16 13\n5 14 20\n15 20 14\n5 13 17\n18 20 14\n14 20 20\n6 13 11\n12 16 19\n2 15 10\n6 17 11\n7 18 7\n8 18 12\n8 16 13\n6 16 13\n2 18 10\n9 10 15\n7 14 20\n","output":"1209600\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMaxPermutation"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let mut per_numbers = vec![vec![]; n];
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let value = input.usize() - 1;
        per_numbers[value].push((fr, to));
    }
    let mut res = Mod::ONE;
    let mut can_set = 0;
    let mut used = vec![false; n];
    for num in 0..n {
        if per_numbers[num].is_empty() {
            can_set += 1;
        } else if per_numbers[num].len() == 1 {
            let (fr, to) = per_numbers[num][0];
            if used[fr] && used[to] {
                out.println(0);
                return;
            }
            if !used[fr] && !used[to] {
                res = res + res;
                res *= Mod::new(can_set);
                can_set -= 1;
            }
            used[fr] = true;
            used[to] = true;
        } else {
            let mut all_pos = vec![];
            for &(fr, to) in per_numbers[num].iter() {
                all_pos.push(fr);
                all_pos.push(to);
            }
            all_pos.sort();
            let mut several = vec![];
            for i in 0..all_pos.len() - 1 {
                if i == 0 || all_pos[i - 1] != all_pos[i] {
                    if all_pos[i] == all_pos[i + 1] {
                        several.push(all_pos[i]);
                    }
                }
            }
            if several.len() != 1 || used[several[0]] {
                out.println(0);
                return;
            }
            for &(fr, to) in per_numbers[num].iter() {
                let another = fr + to - several[0];
                if !used[another] {
                    res *= Mod::new(can_set);
                    can_set -= 1;
                }
                used[fr] = true;
                used[to] = true;
            }
        }
    }
    for i in 0..can_set {
        res *= Mod::new(i + 1);
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
    const PROBLEM_NAME: &str = "c_max_permutation";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
