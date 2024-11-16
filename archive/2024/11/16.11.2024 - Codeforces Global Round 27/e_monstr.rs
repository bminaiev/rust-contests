//{"name":"E. Монстр","group":"Codeforces - Codeforces Global Round 27","url":"https://codeforces.com/contest/2035/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 3 5 5\n10 20 40 5\n1 60 100 10\n60 1 100 10\n","output":"12\n190\n280\n160\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMonstr"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::rand::Random;

fn solve_case(
    incr_cost: i64,
    attack_cost: i64,
    at_most_times: usize,
    mut health: i64,
    slow: bool,
) -> i64 {
    let mut res = i64::MAX;
    let mut cur_cost = 0;
    let mut cur_attack_dmg = 0;
    loop {
        let f = |cnt_incr_more: usize| -> i64 {
            let expected_attack = cur_attack_dmg + cnt_incr_more as i64;
            assert!(expected_attack > 0);
            let cnt_attack = (health + expected_attack - 1) / expected_attack;
            cnt_incr_more as i64 * incr_cost + cnt_attack * attack_cost
        };
        let min_incr = if cur_attack_dmg == 0 { 1 } else { 0 };
        let max_incr = at_most_times;
        // first such i, that f(i + 1) > f(i)
        // let i = binary_search_first_true(min_incr..max_incr, |i| f(i + 1) > f(i));
        // res = res.min(cur_cost + f(i));
        if slow {
            for i in min_incr..=max_incr {
                let cost = cur_cost + f(i);
                res = res.min(cost);
                // dbg!(cur_attack_dmg, i,)
            }
        } else {
            let mut now_cost = cur_cost;
            let mut now_dmg = cur_attack_dmg;
            let max_allowed_dmg = cur_attack_dmg + at_most_times as i64;
            if min_incr == 1 {
                now_cost += incr_cost;
                now_dmg += 1;
            }
            loop {
                let cnt_attacks_needed = (health + now_dmg - 1) / now_dmg;
                res = res.min(cnt_attacks_needed * attack_cost + now_cost);
                if cnt_attacks_needed == 1 {
                    break;
                }
                // now I want to need at most (cnt_attacks_needed - 1) attacks
                // so my dmg should be at least ceil(health / (cnt_attacks_needed - 1))
                let need_dmg = (health + cnt_attacks_needed - 2) / (cnt_attacks_needed - 1);
                if need_dmg > max_allowed_dmg {
                    break;
                }
                now_cost += incr_cost * (need_dmg - now_dmg);
                now_dmg = need_dmg;
            }
        }
        cur_cost += incr_cost * at_most_times as i64;
        cur_attack_dmg += at_most_times as i64;
        health -= cur_attack_dmg;
        cur_cost += attack_cost;
        if health <= 0 {
            res = res.min(cur_cost);
            break;
        }
        if cur_cost >= res {
            break;
        }
    }
    res
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let incr_cost = input.i64();
    let attack_cost = input.i64();
    let health = input.i64();
    let at_most_times = input.usize();
    let res = solve_case(incr_cost, attack_cost, at_most_times, health, false);
    out.println(res);
}

fn stress() {
    for it in 3.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MX: i64 = 1000;
        let incr_cost = rnd.gen(1..MX);
        let attack_cost = rnd.gen(1..MX);
        let health = rnd.gen(1..MX);
        let at_most_times = rnd.gen(1..MX);
        let slow_res = solve_case(incr_cost, attack_cost, at_most_times as usize, health, true);
        let fast_res = solve_case(
            incr_cost,
            attack_cost,
            at_most_times as usize,
            health,
            false,
        );
        if slow_res != fast_res {
            dbg!(
                incr_cost,
                attack_cost,
                health,
                at_most_times,
                slow_res,
                fast_res
            );
            break;
        }
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, &mut output, i + 1);
    }
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "e_monstr";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
