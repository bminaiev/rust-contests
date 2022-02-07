//{"name":"E. Честный делёж","group":"Codeforces - Codeforces Round #770 (Div. 2)","url":"https://codeforces.com/contest/1634/problem/E","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n2\n1 2\n4\n1 2 3 3\n6\n1 1 2 2 3 3\n","output":"YES\nRL\nLRLR\nRLLRRL\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EChestniiDelyozh"}}}

use algo_lib::collections::id_map::IdMap;
use algo_lib::flows::dinic::FlowDinic;
use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rec_function::{Callable2, Callable3, RecursiveFunction2, RecursiveFunction3};
use algo_lib::{dbg, out, out_line};

#[derive(Clone)]
struct Pos {
    row: usize,
    col: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let mut map: IdMap<i32> = IdMap::new();
    let mut a = vec![vec![]; n];
    for row in 0..n {
        let size = input.usize();
        let cur = input.read_vec::<i32>(size);
        for &val in cur.iter() {
            let id = map.get_or_add(&val);
            a[row].push(id);
        }
    }
    let max_val = map.len();
    let mut cnt = vec![0; max_val];
    for row in a.iter() {
        for val in row.iter() {
            cnt[*val] += 1;
        }
    }
    for x in cnt.iter() {
        if x % 2 != 0 {
            out_line!("NO");
            return;
        }
    }
    let mut by_val = vec![vec![]; max_val];
    for row in 0..n {
        for (col, val) in a[row].iter().enumerate() {
            by_val[*val].push(Pos { row, col });
        }
    }

    let mut res_right: Vec<Vec<Option<bool>>> = gen_vec(n, |id| vec![None; a[id].len()]);

    let mut iters = vec![0; n];

    for row in 0..n {
        RecursiveFunction2::new(|f, row: usize, side| {
            while iters[row] != a[row].len() && res_right[row][iters[row]].is_some() {
                iters[row] += 1;
            }
            if a[row].len() != iters[row] {
                let col = iters[row];
                res_right[row][col] = Some(side);
                let val = a[row][col];
                loop {
                    let pos = by_val[val].pop().unwrap();
                    if res_right[pos.row][pos.col].is_some() {
                        continue;
                    }
                    res_right[pos.row][pos.col] = Some(!side);
                    f.call(pos.row, side);
                    break;
                }

                f.call(row, side);
            }
        })
        .call(row, true);
    }

    out_line!("YES");
    for i in 0..n {
        for j in 0..res_right[i].len() {
            if res_right[i][j].unwrap() {
                out!("R");
            } else {
                out!("L");
            }
        }
        out_line!();
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
}
//END MAIN
