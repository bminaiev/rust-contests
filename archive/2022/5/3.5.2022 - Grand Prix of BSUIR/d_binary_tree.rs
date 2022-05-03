//{"name":"D. Binary tree","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/D/","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 2 0\n2 0 0\n","output":"1\nswap 2\n"},{"input":"3\n1 2 3\n3 0 0\n2 0 0\n","output":"3\nswap 2\nrotate 3\nswap 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBinaryTree"}}}

use algo_lib::collections::sorted::SortedTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, RecursiveFunction};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut a = vec![0; n];
    let mut left = vec![n; n];
    let mut right = vec![n; n];
    let mut parent = vec![n; n];
    for i in 0..n {
        a[i] = input.i32();
        left[i] = {
            let x = input.usize();
            if x == 0 {
                n
            } else {
                x - 1
            }
        };
        right[i] = {
            let x = input.usize();
            if x == 0 {
                n
            } else {
                x - 1
            }
        };
    }
    for v in 0..n {
        if left[v] != n {
            parent[left[v]] = v;
        }
        if right[v] != n {
            parent[right[v]] = v;
        }
    }
    let mut root = 0;
    while parent[root] != n {
        root = parent[root];
    }
    let mut cur_perm = vec![];
    RecursiveFunction::new(|f, v| {
        if left[v] != n {
            f.call(left[v]);
        }
        cur_perm.push(v);
        if right[v] != n {
            f.call(right[v]);
        }
    })
    .call(root);

    let sorted_a = a.sorted();

    let mut rnd = Random::new(787788);
    let to_check_perm = rnd.gen_permutation(n);

    let mut need_swap = vec![];

    loop {
        let mut changed = false;

        for &v in to_check_perm.iter() {
            let value = a[v];

            let mut pos = 0;
            while cur_perm[pos] != v {
                pos += 1;
            }

            let mut expected_pos = 0;
            while sorted_a[expected_pos] != value {
                expected_pos += 1;
            }

            if pos != expected_pos {
                let u = cur_perm[expected_pos];
                need_swap.push((v, u));
                a.swap(u, v);
                changed = true;
            }
        }

        if !changed {
            break;
        }
    }

    for i in 0..n {
        assert_eq!(a[cur_perm[i]], sorted_a[i]);
    }

    let mut ops = vec![];

    let set_parent = |v: usize,
                      p: usize,
                      as_left: bool,
                      parent: &mut [usize],
                      left: &mut [usize],
                      right: &mut [usize]| {
        if v != n {
            parent[v] = p;
        }
        if as_left {
            left[p] = v;
        } else {
            right[p] = v;
        }
    };

    let rotate = |v: usize, parent: &mut [usize], left: &mut [usize], right: &mut [usize]| {
        assert!(parent[v] != n);
        let l = left[v];
        let r = right[v];

        let p1 = parent[v];
        let p2 = parent[p1];
        if p2 != n {
            if right[p2] == p1 {
                set_parent(v, p2, false, parent, left, right);
            } else {
                set_parent(v, p2, true, parent, left, right);
            }
        } else {
            parent[v] = n;
        }

        if left[p1] == v {
            set_parent(p1, v, false, parent, left, right);
            set_parent(r, p1, true, parent, left, right);
        } else {
            set_parent(p1, v, true, parent, left, right);
            set_parent(l, p1, false, parent, left, right);
        }
    };

    let make_root = |v: usize,
                     parent: &mut [usize],
                     left: &mut [usize],
                     right: &mut [usize],
                     ops: &mut Vec<Operation>| {
        while parent[v] != n {
            rotate(v, parent, left, right);
            ops.push(Operation::Rotate(v));
        }
    };

    for (u, v) in need_swap.into_iter() {
        make_root(u, &mut parent, &mut left, &mut right, &mut ops);
        make_root(v, &mut parent, &mut left, &mut right, &mut ops);
        ops.push(Operation::Swap(u));

        // let l = left[u];
        // let r = right[u];
        // if left[v] == u {
        //     set_parent(right[v], u, false, &mut parent, &mut left, &mut right);
        //     set_parent(v, u, true, &mut parent, &mut left, &mut right);
        // } else {
        //     set_parent(left[v], u, true, &mut parent, &mut left, &mut right);
        //     set_parent(v, u, false, &mut parent, &mut left, &mut right);
        // }
        // set_parent(l, v, true, &mut parent, &mut left, &mut right);
        // set_parent(r, v, false, &mut parent, &mut left, &mut right);
        // parent[u] = n;
    }

    assert!(ops.len() <= 300000);
    out_line!(ops.len());
    for op in ops.iter() {
        match op {
            Operation::Swap(v) => out_line!("swap", v + 1),
            Operation::Rotate(v) => out_line!("rotate", v + 1),
        }
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Swap(usize),
    Rotate(usize),
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
}
//END MAIN
