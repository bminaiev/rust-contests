//{"name":"F. Робот на доске 2","group":"Codeforces - Codeforces Round #753 (Div. 3)","url":"https://codeforces.com/contest/1607/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n\n1 1\nR\n\n1 3\nRRL\n\n2 2\nDL\nRU\n\n2 2\nUD\nRU\n\n3 2\nDL\nUL\nRU\n\n4 4\nRRRD\nRUUD\nURUD\nULLR\n\n4 4\nDDLU\nRDDU\nUUUU\nRDLD\n","output":"1 1 1\n1 1 3\n1 1 4\n2 1 3\n3 1 5\n4 3 12\n1 1 4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRobotNaDoske2"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::shifts::shift_by_uldr;
use algo_lib::misc::matrix_id_converter::MatrixIdConverter;
use algo_lib::misc::min_max::FindMinMaxPos;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Point = PointT<i32>;

fn solve(input: &mut Input, _test_case: usize) {
    let rows = input.usize();
    let cols = input.usize();
    let matrix = input.read_string_vec(rows);
    let id_conv = MatrixIdConverter::new(rows, cols);
    let n = rows * cols;
    let mut next = vec![0; n];
    for r in 0..rows {
        for c in 0..cols {
            let p = Point::new(r as i32, c as i32);
            let c = matrix[r][c];
            let shift = shift_by_uldr(c);
            let next_p = p.apply_shift(&shift);
            let my_id = id_conv.get_id(&p);
            let next_id = id_conv.get_id_opt(&next_p).unwrap_or(my_id);
            next[my_id] = next_id;
        }
    }
    let mut cnt_inbound = vec![0; n];
    for &to in next.iter() {
        cnt_inbound[to] += 1;
    }
    let mut queue = vec![];
    for v in 0..n {
        if cnt_inbound[v] == 0 {
            queue.push(v);
        }
    }
    let mut it = 0;
    while it != queue.len() {
        let v = queue[it];
        it += 1;
        let to = next[v];
        cnt_inbound[to] -= 1;
        if cnt_inbound[to] == 0 {
            queue.push(to);
        }
    }
    let mut seen = vec![false; n];
    for &v in queue.iter() {
        seen[v] = true;
    }
    let mut res = vec![0; n];
    for v in 0..n {
        if !seen[v] {
            let mut cycle = vec![v];
            let mut cur = next[v];
            while cur != v {
                cycle.push(cur);
                seen[cur] = true;
                cur = next[cur];
            }
            for &cur in cycle.iter() {
                res[cur] = cycle.len();
            }
        }
    }
    for &v in queue.iter().rev() {
        res[v] = res[next[v]] + 1;
    }
    let best_v = res.position_of_max();
    let p = id_conv.conv_back(best_v);
    out_line!(p.x + 1, p.y + 1, res[best_v]);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
