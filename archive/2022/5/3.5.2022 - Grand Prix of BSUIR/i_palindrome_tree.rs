//{"name":"I. Palindrome tree","group":"Yandex - Grand Prix of BSUIR","url":"https://official.contest.yandex.com/opencupXXII/contest/37753/problems/I/","interactive":false,"timeLimit":2000,"tests":[{"input":"3 2\naba\n1 2\n1 3\n","output":"3\n1 2 3\n"},{"input":"3 2\naba\n1 2\n2 3\n","output":"2\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"IPalindromeTree"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let s = input.string();
    let g = read_graph(
        input,
        n,
        n - 1,
        Directional::Undirected,
        Indexation::FromOne,
    );
    let mut parent = vec![0; n];
    let mut height = vec![0; n];
    RecursiveFunction2::new(|f, v: usize, p: usize| {
        parent[v] = p;
        for e in g.adj(v) {
            if e.to() == p {
                continue;
            }
            height[e.to()] = height[v] + 1;
            f.call(e.to(), v);
        }
    })
    .call(0, 0);
    let mut sorted = gen_vec(n, id);
    sorted.sort_by_key(|v| height[*v]);

    let mut first_on_path = Array2D::new(None, n, n);

    for &v in sorted.iter() {
        let mut path = vec![v];
        let mut cur = v;
        while cur != 0 {
            cur = parent[cur];
            path.push(cur);
        }
        for i in 1..path.len() {
            first_on_path[path[i]][v] = Some(path[i - 1]);
        }
    }

    let mut bad_pair = Array2D::new(false, n, n);
    let mut is_palindrom = Array2D::new(false, n, n);

    let mut pair_len = Array2D::new(0, n, n);
    for v in 0..n {
        is_palindrom[v][v] = true;
        pair_len[v][v] = 1;
    }

    for idx2 in 1..n {
        let v2 = sorted[idx2];
        for idx1 in 0..idx2 {
            let v1 = sorted[idx1];

            if let Some(next) = first_on_path[v1][v2] {
                if next == v2 {
                    pair_len[v1][v2] = 2;
                    pair_len[v2][v1] = 2;
                    if s[v1] == s[v2] {
                        is_palindrom[v1][v2] = true;
                        is_palindrom[v2][v1] = true;
                    }
                } else {
                    pair_len[v1][v2] = pair_len[next][parent[v2]] + 2;
                    pair_len[v2][v1] = pair_len[v1][v2];
                    if s[v1] == s[v2] && is_palindrom[next][parent[v2]] {
                        is_palindrom[v1][v2] = true;
                        is_palindrom[v2][v1] = true;
                    }
                }
            } else {
                let n1 = parent[v1];
                let n2 = parent[v2];
                pair_len[v1][v2] = pair_len[n1][n2] + 2;
                pair_len[v2][v1] = pair_len[v1][v2];
                if s[v1] == s[v2] && is_palindrom[n1][n2] {
                    is_palindrom[v1][v2] = true;
                    is_palindrom[v2][v1] = true;
                }
            }
        }
    }
    for v1 in 0..n {
        for v2 in 0..n {
            if v1 != v2 {
                assert!(pair_len[v1][v2] != 0);
            }
            assert_eq!(pair_len[v1][v2], pair_len[v2][v1]);
            if pair_len[v1][v2] > k && is_palindrom[v1][v2] {
                bad_pair[v1][v2] = true;
            }
        }
    }

    let mut never_good = vec![false; n];
    for v in 0..n {
        for v2 in 0..n {
            if let Some(_) = first_on_path[v2][v] {
                if bad_pair[v2][v] {
                    never_good[v] = true;
                }
            }
        }
    }

    let mut use_in_res = vec![false; n];
    use_in_res[0] = true;
    let mut queue = vec![0];
    let mut iter = vec![0; n];
    for v in 1..n {
        if use_in_res[v] {
            continue;
        }
        let mut path = vec![];
        let mut cur = v;
        while !use_in_res[cur] {
            path.push(cur);
            cur = parent[cur];
        }
        let mut ok = true;
        for &u in path.iter() {
            while iter[u] != queue.len() && !bad_pair[queue[iter[u]]][u] {
                iter[u] += 1;
            }
            if iter[u] != queue.len() {
                ok = false;
                break;
            }
            if never_good[u] {
                ok = false;
            }
        }
        if ok {
            for &u in path.iter() {
                use_in_res[u] = true;
                queue.push(u);
            }
        }
    }
    let mut cnt = 0;
    for i in 0..n {
        if use_in_res[i] {
            cnt += 1;
        }
    }
    out_line!(cnt);
    for v in 0..n {
        if use_in_res[v] {
            out!(v + 1, "");
        }
    }
    out_line!();
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
