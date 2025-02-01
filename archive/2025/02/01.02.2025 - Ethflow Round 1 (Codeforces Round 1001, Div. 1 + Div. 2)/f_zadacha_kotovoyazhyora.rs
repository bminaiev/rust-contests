//{"name":"F. Задача котовояжёра","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n0 2\n2 1\n3 3\n5\n2 7\n7 5\n6 3\n1 8\n7 5\n8\n899167687 609615846\n851467150 45726720\n931502759 23784096\n918190644 196992738\n142090421 475722765\n409556751 726971942\n513558832 998277529\n294328304 434714258\n","output":"4 9\n10 22 34 46\n770051069 1655330585 2931719265 3918741472 5033924854 6425541981 7934325514\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FZadachaKotovoyazhyora"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone, Copy, Debug)]
struct City {
    base: i64,
    extra: i64,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut right = vec![];
        let mut up = vec![];
        let mut zeros = vec![];
        for id in 0..n {
            let a = input.i64();
            let b = input.i64();
            if a == b {
                zeros.push(City { base: a, extra: 0 });
            } else if a < b {
                right.push(City {
                    base: a,
                    extra: b - a,
                });
            } else {
                up.push(City {
                    base: b,
                    extra: a - b,
                });
            }
        }
        let mut res = vec![i64::MAX; n + 1];
        let mut right_with_zeros = right.clone();
        right_with_zeros.extend(zeros.iter().copied());
        let mut up_with_zeros = up.clone();
        up_with_zeros.extend(zeros.iter().copied());
        let mut one_side = |cities: &mut Vec<City>| {
            cities.sort_by_key(|c| c.base + c.extra);
            dbg!(cities);
            for i in 0..cities.len() {
                let mut cur_cnt = 1;
                let mut cur_cost = cities[i].base;
                for j in 0..cities.len() {
                    if i == j || (cities[j].extra < cities[i].extra) {
                        continue;
                    }
                    cur_cnt += 1;
                    cur_cost += cities[j].base + cities[j].extra;
                    dbg!(i, j, cur_cnt, cur_cost);
                    res[cur_cnt] = res[cur_cnt].min(cur_cost);
                }
            }
        };
        one_side(&mut right_with_zeros);

        one_side(&mut up_with_zeros);
        let mut two_side = |right: &mut Vec<City>, up: &mut Vec<City>| {
            right.sort_by_key(|c| c.base + c.extra);
            up.sort_by_key(|c| c.base + c.extra);
            for cnt_right in 1..=right.len() {
                let mut cost = 0;
                for city in right.iter().take(cnt_right) {
                    cost += city.base + city.extra;
                }
                for cnt_up in 1..=up.len() {
                    cost += up[cnt_up - 1].base + up[cnt_up - 1].extra;
                    let cnt = cnt_right + cnt_up;
                    res[cnt] = res[cnt].min(cost);
                }
            }
        };
        two_side(&mut right_with_zeros, &mut up);
        out.println(res[2..].to_vec());
        break;
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "f_zadacha_kotovoyazhyora";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
