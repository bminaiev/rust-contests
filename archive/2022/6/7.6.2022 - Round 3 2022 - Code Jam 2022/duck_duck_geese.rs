//{"name":"Duck, Duck, Geese","group":"Google Coding Competitions - Round 3 2022 - Code Jam 2022","url":"https://codingcompetitions.withgoogle.com/codejam/round/00000000008779b4/0000000000b45244","interactive":false,"timeLimit":20000,"tests":[{"input":"3\n3 2\n1 1\n1 1\n1 1 2\n5 2\n1 1\n1 2\n1 2 1 2 2\n3 3\n1 2\n1 2\n2 2\n1 1 3\n","output":"Case #1: 2\nCase #2: 9\nCase #3: 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DuckDuckGeese"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Default, Copy, Debug)]
pub struct MaxValNode {
    pub max_val: i32,
    pub cnt: i64,
}

impl LazySegTreeNodeSpec for MaxValNode {
    #[allow(unused)]
    fn unite(l: &Self, r: &Self, context: &()) -> Self {
        if l.max_val > r.max_val {
            *l
        } else if l.max_val < r.max_val {
            *r
        } else {
            Self {
                max_val: l.max_val,
                cnt: l.cnt + r.cnt,
            }
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.max_val += *update;
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current += *add;
    }

    type Update = i32;
    type Context = ();
}

pub type SegTree = LazySegTree<MaxValNode>;

fn solve(input: &mut Input, test_case: usize) {
    let n = input.usize();
    let colors = input.usize();
    let mut req = gen_vec(colors, |_| (input.usize(), input.usize()));
    for i in 0..req.len() {
        if req[i].0 == 0 {
            req[i].0 = 1;
        }
    }
    let mut a = input.vec::<usize>(n).sub_from_all(1);
    for i in 0..(n - 1) {
        a.push(a[i]);
    }
    let mut by_color = vec![vec![]; n];
    for i in 0..a.len() {
        by_color[a[i]].push(i);
    }
    let mut res = 0;

    let get_pos = |color: usize, idx: usize| -> usize {
        if by_color[color].len() <= idx {
            a.len()
        } else {
            by_color[color][idx]
        }
    };

    // let mut st = vec![0i32; a.len()];

    let mut st = SegTree::new_f(a.len(), &|_| MaxValNode { max_val: 0, cnt: 1 });

    for color in 0..colors {
        let first = get_pos(color, 0);
        st.update(0..first, 1);
        let from = get_pos(color, req[color].0 - 1);
        let to = get_pos(color, req[color].1);
        st.update(from..to, 1);
    }
    let mut skipped = vec![0; colors];
    for start in 0..n {
        let st_res = st.get(start + 1..start + n - 1);
        if st_res.max_val == colors as i32 {
            res += st_res.cnt;
        }
        let color = a[start];
        let mut apply_delta = |delta: i32, skipped: &[usize]| {
            let first = get_pos(color, skipped[color]);
            st.update(start..first, delta);

            let from = get_pos(color, skipped[color] + req[color].0 - 1);
            let to = get_pos(color, skipped[color] + req[color].1);

            st.update(from..to, delta);
        };
        apply_delta(-1, &skipped);
        skipped[color] += 1;
        apply_delta(1, &skipped);
    }
    out_line!(format!("Case #{}: {}", test_case, res));
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
