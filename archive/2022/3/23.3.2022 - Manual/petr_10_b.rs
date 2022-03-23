//{"name":"petr_10_b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"file","fileName":"combi","pattern":null},"output":{"type":"file","fileName":"combi.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_b"}}}

use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo_runtime::ModRuntime;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Debug)]
enum Result {
    Full(i128),
    LastDigits(i128),
}

const BIG: i128 = 10_000_000_000;

fn calc_power_in_fact(n: i128, base: i128) -> i128 {
    let mut res = 0;
    let mut cur_base = base;
    while cur_base <= n {
        res += n / cur_base;
        cur_base *= base;
    }
    res
}

struct Precalc {
    values: Vec<Vec<i32>>,
}

impl Precalc {
    pub fn get(&mut self, without: i32, up_to: i32) -> ModRuntime {
        while self.values.len() <= without as usize {
            self.values.push(vec![1]);
        }
        let m = without.pow(10);
        let cur = &mut self.values[without as usize];
        while cur.len() <= up_to as usize {
            let last = *cur.last_exn();
            let next = cur.len() as i32;
            let next_val = if next % without == 0 {
                last
            } else {
                ((last as i64) * (next as i64) % (m as i64)) as i32
            };
            cur.push(next_val);
        }
        ModRuntime::new(cur[up_to as usize], m)
    }
}

fn calc_fact_modulo_helper(n: i128, m: i32, without: i32, p: &mut Precalc) -> ModRuntime {
    let till = (n % (m as i128)) as i32;
    let mut res = p.get(without, till);
    let full = p.get(without, m);
    let cnt_full = n / (m as i128);
    let full_part = full.pow_i128(cnt_full);
    res *= full_part;
    res
}

fn calc_fact_modulo(n: i128, m: i32, without: i32, p: &mut Precalc) -> ModRuntime {
    if n <= 1 {
        ModRuntime::new(1, m)
    } else {
        calc_fact_modulo_helper(n, m, without, p)
            * calc_fact_modulo(n / without as i128, m, without, p)
    }
}

fn calc_modulo(n: i128, k: i128, m: i32, p: &mut Precalc) -> ModRuntime {
    let pw = m.pow(10);
    let mut res = calc_fact_modulo(n, pw, m, p);
    res /= calc_fact_modulo(k, pw, m, p);
    res /= calc_fact_modulo(n - k, pw, m, p);
    res
}

fn power(base: i128, pw: i128, m: i128) -> i128 {
    if pw == 0 {
        1
    } else if pw == 1 {
        base % m
    } else {
        let half = power(base, pw / 2, m);
        let mut res = (half * half) % m;
        if pw & 1 == 1 {
            res = (res * base) % m;
        }
        res
    }
}

fn chinese_reminder_theorem(a1: i32, p1: i32, a2: i32, p2: i32) -> i128 {
    let x1 = a1 as i128;
    let p1_rev = ModRuntime::new(p1, p2).inv();
    let x2 = (ModRuntime::new(a2, p2) - ModRuntime::new(x1 as i32, p2)) * p1_rev;
    let res = x1 + (x2.value() as i128) * (p1 as i128);
    assert!(res < (p1 as i128) * (p2 as i128));
    assert_eq!(res % p1 as i128, a1 as i128);
    assert_eq!(res % p2 as i128, a2 as i128);
    res
}

fn solve_big(n: i128, k: i128) -> i128 {
    let pw2 = calc_power_in_fact(n, 2) - calc_power_in_fact(k, 2) - calc_power_in_fact(n - k, 2);
    assert!(pw2 >= 0);
    let pw5 = calc_power_in_fact(n, 5) - calc_power_in_fact(k, 5) - calc_power_in_fact(n - k, 5);
    assert!(pw5 >= 0);
    let mut res = power(2, pw2, BIG) * power(5, pw5, BIG) % BIG;
    let mut p = Precalc { values: vec![] };
    let mod2_10 = calc_modulo(n, k, 2, &mut p) / (ModRuntime::new(5, 2i32.pow(10)).pow_i128(pw5));
    let mod5_10 = calc_modulo(n, k, 5, &mut p) / (ModRuntime::new(2, 5i32.pow(10)).pow_i128(pw2));
    let from_chinese =
        chinese_reminder_theorem(mod2_10.value(), 2i32.pow(10), mod5_10.value(), 5i32.pow(10));
    res *= from_chinese;
    res %= BIG;
    res
}

fn solve_n_k(n: i128, k: i128) -> Result {
    let mut res = 1;
    let k = if n - k < k { n - k } else { k };
    for i in 1..=k {
        res *= n + 1 - i;
        res /= i;
        if res >= BIG {
            return Result::LastDigits(solve_big(n, k));
        }
    }
    Result::Full(res)
}

fn stress() {
    for n in 1.. {
        for k in 0..=n {
            dbg!(n, k);
            let res = solve_n_k(n, k);
            dbg!(res);
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k) = input.read();
    match solve_n_k(n, k) {
        Result::Full(res) => {
            out_line!(res);
        }
        Result::LastDigits(digits) => {
            let s = format!("...{:010}", digits);
            out_line!(s);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::File("combi.in".to_string()),
        output: TaskIoType::File("combi.out".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("3");
    // tester::run_stress(stress);
}
//END MAIN
