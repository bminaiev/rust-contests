//{"name":"m","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::rand::Random;

fn calc_cost(start: i64, pos: i64, step: i64, cost: i64) -> i64 {
    let dist = (pos - start).abs();
    let at_least = dist / step;
    let from_left = at_least * cost + (dist % step);
    let from_right = (at_least + 1) * cost + (step - (dist % step));
    // dbg!(start, pos, step, cost, from_left, from_right);
    from_left.min(from_right)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Event {
    x: i64,
    delta: i64,
}

fn solve_base(a: &[i64], step: i64, cost: i64, base: i64, stupid: bool) -> i64 {
    if stupid {
        let mut res = i64::MAX;
        for x in base - step..=base + step {
            let mut cur_cost = 0;
            for &start in a.iter() {
                cur_cost += calc_cost(start, x, step, cost);
            }
            res = res.min(cur_cost);
        }
        return res;
    }

    // dbg!(base, step);
    let mut events = Vec::new();
    let mut offset = 0;

    for &start in a.iter() {
        // dbg!(start);
        let mut check = vec![base - step, base + step];
        {
            let z = cost + (step - cost) / 2;
            for delta in [-z - 1, -z, 0, z, z + 1] {
                let x = start + delta;
                let muls = (base - x) / step;
                let cur_base = x + muls * step;
                for offset in -3..=3 {
                    let x = cur_base + offset * step;
                    if x >= base - step && x <= base + step {
                        check.push(x);
                    }
                }
            }
        }
        check.sort();
        check.dedup();

        let mut ys = Vec::new();
        for &x in check.iter() {
            let y = calc_cost(start, x, step, cost);
            // dbg!(x, y);
            ys.push(y);
        }
        offset += ys[0];
        for i in 0..ys.len() - 1 {
            let delta_x = check[i + 1] - check[i];
            let delta_y = ys[i + 1] - ys[i];
            // dbg!(delta_x, delta_y, check[i], check[i + 1]);
            assert!(delta_y == 0 || delta_y.abs() == delta_x);
            events.push(Event {
                x: check[i],
                delta: delta_y / delta_x,
            });
            events.push(Event {
                x: check[i + 1],
                delta: -delta_y / delta_x,
            });
        }
    }
    events.push(Event {
        x: base + step,
        delta: 0,
    });
    events.sort();
    let mut res = i64::MAX;
    let mut cur_x = base - step;
    let mut cur_y = offset;
    let mut cur_delta = 0;
    for event in events.iter() {
        // dbg!(event);
        let delta_x = event.x - cur_x;
        assert!(delta_x >= 0);
        cur_x += delta_x;
        cur_y += cur_delta * delta_x;
        cur_delta += event.delta;
        res = res.min(cur_y);
    }
    res
}

fn solve_case(a: &[i64], step: i64, cost: i64, stupid: bool) -> i64 {
    let cost = cost.min(step);
    assert!(a.is_sorted());
    let m1 = a[a.len() / 2];
    let mut res = solve_base(a, step, cost, m1, stupid);

    if a.len() % 2 == 0 {
        let m2 = a[(a.len() + 1) / 2];
        res = res.min(solve_base(a, step, cost, m2, stupid));
    }
    res
}

fn stress() {
    const MAX_V: i64 = 1000;
    const MAX_N: usize = 50;
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_range(1..MAX_N);
        let a = rnd.gen_range(1..MAX_V);
        let b = rnd.gen_range(1..MAX_V);
        let mut xs = rnd.gen_vec(n, 1..MAX_V);
        xs.sort();
        let res1 = solve_case(&xs, b, a, true);
        let res2 = solve_case(&xs, b, a, false);
        if res1 != res2 {
            dbg!(&a, b, res1, res2);
            assert!(false);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let step = input.i64();
        let cost = input.i64().min(step);
        let mut a = input.vec::<i64>(n);
        a.sort();
        let res = solve_case(&a, step, cost, false);
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
    const PROBLEM_NAME: &str = "m";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
