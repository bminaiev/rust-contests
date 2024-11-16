//{"name":"b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"b"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo_runtime::ModRuntime;
use algo_lib::misc::num_traits::HasConstants;

type Mod = ModRuntime;

pub fn gen_facts(n: usize, p: i32) -> Vec<Mod> {
    let mut res = Vec::with_capacity(n);
    res.push(Mod::new(1, p));
    for x in 1..=n {
        let num = Mod::new(x as i32, p);
        res.push(*res.last().unwrap() * num);
    }
    res
}

pub struct CombinationsFact {
    fact: Vec<Mod>,
    fact_inv: Vec<Mod>,
    invs: Vec<Mod>,
    p: i32,
}

impl CombinationsFact {
    #[allow(unused)]
    pub fn new(n: usize, p: i32) -> Self {
        let fact = gen_facts(n, p);
        let mut fact_inv = fact.clone();
        assert_eq!(fact_inv.len(), n + 1);
        fact_inv[n] = Mod::new(1, p) / fact_inv[n];
        for i in (1..n).rev() {
            fact_inv[i] = fact_inv[i + 1] * Mod::new((i + 1) as i32, p);
        }
        let mut invs = fact_inv.clone();
        for i in 1..=n {
            invs[i] *= fact[i - 1];
        }
        Self {
            fact,
            fact_inv,
            p,
            invs,
        }
    }

    pub fn fact(&self, n: usize) -> Mod {
        self.fact[n]
    }

    fn c(&self, n: usize, k: usize) -> Mod {
        if k > n {
            return Mod::new(0, self.p);
        }
        self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let p = input.i32();
        let mut b = ModRuntime::new(input.i32(), p);
        let A = ModRuntime::new(input.i32(), p);
        let B = ModRuntime::new(input.i32(), p);
        let mut a = vec![ModRuntime::new(0, p)];
        for i in 1..=2 * n {
            let a_last = *a.last().unwrap();
            b = A * b + B;
            a.push(a_last + ModRuntime::new(1, p) + b);
        }
        for i in 1..a.len() {
            let prev = a[i - 1];
            a[i] += prev;
        }
        let cnk = CombinationsFact::new(n * 2 + 2, p);
        let mut brackets = vec![Mod::new(1, p); n + 1];
        for i in 1..=n {
            // TODO: slow?
            brackets[i] = cnk.c(i * 2, i) * cnk.invs[i + 1];
        }
        let total_ways = brackets[n];
        let mut res = Mod::new(0, p);
        for k in 1..=n {
            let ways = brackets[k - 1] * brackets[n - k];
            let sz = k * 2;
            let cnt = 2 * n - (sz - 1);
            let sum_right = a[2 * n] - a[sz - 1];
            let sum_left = a[cnt] - a[0];
            let sum_dist = sum_right - sum_left;
            res += ways * sum_dist;
        }
        res /= total_ways;
        out.println(res);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "b";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
