//{"name":"l","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"l"}}}

use std::collections::BTreeSet;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::UpdateMinMax;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Elem {
    a: i64,
    b: i64,
    id: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Res {
    cost: i64,
    pos: usize,
}

#[derive(Clone, Copy, Debug)]
struct Query {
    min_pos: usize,
    max_pos: usize,
    need_cnt: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = gen_vec(n, |id| Elem {
        a: input.read(),
        b: input.read(),
        id,
    });
    a.sort_by_key(|e| e.b);
    let mut res = vec![
        Res {
            cost: i64::MAX,
            pos: 0
        };
        n + 1
    ];
    {
        let mut sum_a = 0;
        for i in 0..n {
            sum_a += a[i].a;
            res[1].update_min(Res {
                cost: a[i].a + a[i].b,
                pos: i,
            });
        }
        res[n].update_min(Res {
            cost: sum_a + a[n - 1].b,
            pos: n - 1,
        });
    }

    loop {
        let mut queries = vec![];
        {
            let mut i = 0;
            while i + 1 < n {
                if res[i].cost != i64::MAX && res[i + 1].cost == i64::MAX {
                    let mut j = i + 1;
                    while res[j].cost == i64::MAX {
                        j += 1;
                    }
                    let mid = (i + j) / 2;
                    queries.push(Query {
                        min_pos: res[i].pos,
                        max_pos: res[j].pos,
                        need_cnt: mid,
                    });
                    i = j;
                } else {
                    i += 1;
                }
            }
        }
        if queries.is_empty() {
            break;
        }
        let mut set_smaller = BTreeSet::new();
        let mut sum_smaller = 0;
        let mut set_bigger = BTreeSet::new();
        let mut q_it = 0;
        for i in 0..n {
            let elem = a[i];
            sum_smaller += elem.a;
            set_smaller.insert(elem);
            while q_it != queries.len() {
                let need_k = queries[q_it].need_cnt;
                if set_smaller.len() > need_k {
                    let elem = *set_smaller.iter().next_back().unwrap();
                    set_smaller.remove(&elem);
                    sum_smaller -= elem.a;
                    set_bigger.insert(elem);
                }
                while set_smaller.len() < need_k && !set_bigger.is_empty() {
                    let elem = *set_bigger.iter().next().unwrap();
                    set_bigger.remove(&elem);
                    sum_smaller += elem.a;
                    set_smaller.insert(elem);
                }
                if set_smaller.len() == need_k {
                    res[queries[q_it].need_cnt].update_min(Res {
                        cost: sum_smaller + elem.b,
                        pos: i,
                    });
                }
                if queries[q_it].max_pos == i {
                    q_it += 1;
                    assert!(q_it == queries.len() || queries[q_it].min_pos >= i)
                } else {
                    break;
                }
            }
        }
        assert_eq!(q_it, queries.len());
    }
    for i in 1..(n - 1) {
        assert!(res[i].pos <= res[i + 1].pos);
    }
    for i in 1..=n {
        out_line!(res[i].cost);
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
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
