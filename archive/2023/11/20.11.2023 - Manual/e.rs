//{"name":"e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"e"}}}

use algo_lib::collections::permutation::Permutation;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Copy, Clone, Debug)]
struct Object {
    id: usize,
    a: i64,
    b: i64,
    c: i64,
}

impl Object {
    fn sign(&self) -> i32 {
        if self.a < self.b {
            -1
        } else if self.a > self.b {
            1
        } else {
            0
        }
    }
}

fn max_cost(a: &[Object]) -> i64 {
    let mut res = 0;
    for i in 0..a.len() {
        let mut cur = 0;
        for j in 0..a.len() {
            let add = if j < i {
                a[j].a
            } else if j == i {
                a[j].a + a[j].b
            } else {
                a[j].b
            };
            cur += add * a[i].c;
        }
        res = res.max(cur);
    }
    res
}

fn solve_fast(a: &mut Vec<Object>) {
    a.sort_by(|o1, o2| {
        let s1 = o1.sign();
        let s2 = o2.sign();
        if s1 != s2 {
            return s1.cmp(&s2);
        }
        let c1 = max_cost(&[o1.clone(), o2.clone()]);
        let c2 = max_cost(&[o2.clone(), o1.clone()]);
        c1.cmp(&c2)
    });
}

fn is_good_perm(a: &[Object]) -> bool {
    for w in a.windows(2) {
        let s1 = w[0].sign();
        let s2 = w[1].sign();
        if s1 > s2 {
            return false;
        }
    }
    true
}

fn solve_slow(a: &[Object]) -> (i64, Vec<Object>) {
    let mut res = (std::i64::MAX, vec![]);
    let mut perm = Permutation::new(a.len());
    loop {
        let mut b = vec![];
        for i in 0..a.len() {
            b.push(a[perm[i]]);
        }
        let cost = max_cost(&b);
        if cost < res.0 {
            res = (cost, b.clone());
        }
        if !perm.next() {
            break;
        }
    }
    res
}

fn exist_slow(a: &[Object], need_res: i64) -> bool {
    let mut perm = Permutation::new(a.len());
    loop {
        let mut b = vec![];
        for i in 0..a.len() {
            b.push(a[perm[i]]);
        }
        let cost = max_cost(&b);
        if cost == need_res {
            if is_good_perm(&b) {
                return true;
            }
        }
        if !perm.next() {
            break;
        }
    }
    false
}

fn solve(input: &mut Input, _test_case: usize) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut a = gen_vec(n, |id| {
            let a = input.i64();
            let b = input.i64();
            let c = input.i64();
            Object { id, a, b, c }
        });
        solve_fast(&mut a);
        let ids = a.iter().map(|o| o.id + 1).collect::<Vec<_>>();
        // let slow_res = solve_slow(&a);
        // dbg!(slow_res, max_cost(&a), ids);
        // assert_eq!(slow_res, max_cost(&a));
        out_line!(ids);
    }
}

fn stress() {
    for it in 5865.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..9);
        let mut aa = vec![];
        let mx = rnd.gen(2..30);
        for id in 0..n {
            let a = rnd.gen(1..mx);
            let b = rnd.gen(1..mx);
            let c = rnd.gen(1..mx);
            aa.push(Object { id, a, b, c });
        }
        let (slow_res, slow_p) = solve_slow(&aa);
        let exist = exist_slow(&aa, slow_res);
        assert!(exist);
    }
}

fn stress123() {
    for it in 5865.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen(1..4);
        let mut aa = vec![];
        let mx = rnd.gen(2..10);
        for id in 0..n {
            let a = rnd.gen(1..mx);
            let b = rnd.gen(1..mx);
            let c = rnd.gen(1..mx);
            aa.push(Object { id, a, b, c });
        }
        let (slow_res, slow_p) = solve_slow(&aa);
        solve_fast(&mut aa);
        let fast_res = max_cost(&aa);
        if slow_res != fast_res {
            dbg!(slow_res, fast_res);
            dbg!(aa);
            dbg!(slow_p);
            dbg!(max_cost(&[slow_p[0], slow_p[1]]));
            dbg!(max_cost(&[slow_p[1], slow_p[0]]));
            let mut zz = vec![slow_p[0], slow_p[1]];
            dbg!("PREV", zz);
            zz.sort_by(|o1, o2| {
                let s1 = o1.sign();
                let s2 = o2.sign();
                if s1 != s2 {
                    return s1.cmp(&s2);
                }

                dbg!(o1, o2);

                let c1 = max_cost(&[o1.clone(), o2.clone()]);
                let c2 = max_cost(&[o2.clone(), o1.clone()]);
                dbg!("???", c1, c2);
                let res = c1.cmp(&c2);
                dbg!("RES= ", res);
                res
            });
            dbg!(zz);
            assert_eq!(slow_res, fast_res);
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
