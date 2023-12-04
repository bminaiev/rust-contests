//{"name":"i","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"i"}}}

use std::collections::HashMap;
use std::hash::Hash;

use algo_lib::collections::permutation::Permutation;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn gen_hm(all: &[Vec<usize>], n: usize) -> HashMap<Number, Vec<usize>> {
    let mut single = vec![0; n];
    let mut first = vec![0; n];
    let mut second = vec![0; n];
    for cur in all.iter() {
        if cur.len() == 1 {
            single[cur[0]] += 1;
        } else {
            first[cur[0]] += 1;
            second[cur[1]] += 1;
        }
    }
    let mut numbers: HashMap<Number, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let num = Number {
            signle: single[i],
            first: first[i],
            second: second[i],
        };
        numbers.entry(num).or_default().push(i);
    }
    numbers
}

fn gen_correct(n: usize) -> Vec<Vec<usize>> {
    let mut all = vec![];
    for i in 0..n {
        for j in 0..n {
            let mut res = vec![];
            let x = i * j;
            if x < n {
                res.push(x);
            } else {
                res = vec![x / n, x % n];
            }
            all.push(res);
        }
    }
    all
}

fn check(n: usize) {
    let all = gen_correct(n);
    let numbers = gen_hm(&all, n);
    for (k, v) in numbers.iter() {
        if v.len() > 1 {
            dbg!(n, k, v);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Number {
    signle: usize,
    first: usize,
    second: usize,
}

fn stress() {
    for n in 2..=52 {
        dbg!(n);
        check(n);
    }
}

fn conv(c: u8) -> usize {
    if c >= b'a' && c <= b'z' {
        (c - b'a') as usize
    } else {
        (c - b'A') as usize + 26
    }
}

fn conv_back(x: usize) -> char {
    if x < 26 {
        (x as u8 + b'a') as char
    } else {
        (x as u8 - 26 + b'A') as char
    }
}

type HM = HashMap<Number, Vec<usize>>;

fn check_mapping(from_target: &Vec<usize>, a: &Vec<Vec<usize>>, n: usize) -> bool {
    let mut correct = gen_correct(n);
    for entry in correct.iter_mut() {
        for x in entry.iter_mut() {
            *x = from_target[*x];
        }
    }
    correct.sort();
    a == &correct
}

fn solve_case(mut a: Vec<Vec<usize>>, n: usize) {
    a.sort();
    let mut my_numbers = gen_hm(&a, n);
    let mut need_numbers = gen_hm(&gen_correct(n), n);

    // dbg!(my_numbers);
    // dbg!(need_numbers);

    let mut from_target = vec![n; n];
    for (k, v) in my_numbers.iter() {
        if v.len() == 1 {
            let v2 = need_numbers.get_mut(k).unwrap();
            assert_eq!(v2.len(), 1);
            from_target[v2[0]] = v[0];
        }
    }
    let mut target_not_used: Vec<usize> = vec![];
    let mut my_used = vec![false; n];
    for x in from_target.iter() {
        if *x != n {
            my_used[*x] = true;
        }
    }
    let mut my_not_used = vec![];
    for i in 0..n {
        if from_target[i] == n {
            target_not_used.push(i);
        }
        if !my_used[i] {
            my_not_used.push(i);
        }
    }
    let sz = target_not_used.len();
    // dbg!(target_not_used.len(), my_not_used.len());
    assert!(sz <= 4);
    let mut perm = Permutation::new(sz);
    loop {
        for i in 0..sz {
            from_target[target_not_used[i]] = my_not_used[perm[i]];
        }
        if check_mapping(&from_target, &a, n) {
            for x in from_target.iter() {
                out!(conv_back(*x));
            }
            out_line!();
            return;
        }
        if !perm.next() {
            break;
        }
    }
    panic!();
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = vec![];
    for _ in 0..n * n {
        let s = input.string();
        if s.len() == 1 {
            a.push(vec![conv(s[0])]);
        } else {
            assert_eq!(s.len(), 2);
            a.push(vec![conv(s[0]), conv(s[1])]);
        }
    }
    solve_case(a, n)
}

fn stress2() {
    let n = 49;
    let mut a = gen_correct(n);
    solve_case(a, n);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress2);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
