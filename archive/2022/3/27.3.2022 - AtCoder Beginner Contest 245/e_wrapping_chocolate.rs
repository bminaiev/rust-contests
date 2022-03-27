//{"name":"E - Wrapping Chocolate","group":"AtCoder - AtCoder Beginner Contest 245","url":"https://atcoder.jp/contests/abc245/tasks/abc245_e","interactive":false,"timeLimit":4000,"tests":[{"input":"2 3\n2 4\n3 2\n8 1 5\n2 10 5\n","output":"Yes\n"},{"input":"2 2\n1 1\n2 2\n100 1\n100 1\n","output":"No\n"},{"input":"1 1\n10\n100\n100\n10\n","output":"No\n"},{"input":"1 1\n10\n100\n10\n100\n","output":"Yes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EWrappingChocolate"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::fenwick::Fenwick;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Element {
    w: usize,
    h: usize,
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let m = input.usize();
    let a = input.vec::<usize>(n);
    let b = input.vec::<usize>(n);
    let c = input.vec::<usize>(m);
    let d = input.vec::<usize>(m);
    let mut coords = vec![];
    coords.append(&mut b.clone());
    coords.append(&mut d.clone());
    coords.sort();
    let mut need = gen_vec(n, |id| Element {
        w: a[id],
        h: coords.binary_search(&b[id]).unwrap(),
    });
    let mut boxes = gen_vec(m, |id| Element {
        w: c[id],
        h: coords.binary_search(&d[id]).unwrap(),
    });
    need.sort();
    need.reverse();
    boxes.sort();
    boxes.reverse();
    let mut fenw = Fenwick::<i32>::new(coords.len());
    let mut it = 0;
    for need in need.iter() {
        while it != boxes.len() && boxes[it].w >= need.w {
            fenw.add(boxes[it].h, 1);
            it += 1;
        }
        let need_h = need.h;
        let first = binary_search_first_true(need_h..fenw.len(), |sz| {
            fenw.get_range_sum(need_h..sz + 1) > 0
        });
        if first == fenw.len() {
            out_line!("No");
            return;
        }
        fenw.add(first, -1);
    }
    out_line!("Yes");
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
