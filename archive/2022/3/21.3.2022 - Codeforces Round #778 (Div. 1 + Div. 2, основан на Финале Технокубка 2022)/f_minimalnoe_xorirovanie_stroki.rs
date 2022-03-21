//{"name":"F. Минимальное XOR-ирование строки","group":"Codeforces - Codeforces Round #778 (Div. 1 + Div. 2, основан на Финале Технокубка 2022)","url":"http://codeforces.com/contest/1654/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"2\nacba\n","output":"abca\n"},{"input":"3\nbcbaaabb\n","output":"aabbbcba\n"},{"input":"4\nbdbcbccdbdbaaccd\n","output":"abdbdccacbdbdccb\n"},{"input":"5\nccfcffccccccffcfcfccfffffcccccff\n","output":"cccccffffcccccffccfcffcccccfffff\n"},{"input":"1\nzz\n","output":"zz\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMinimalnoeXORIrovanieStroki"}}}

use std::cmp::Ordering;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::Mod9;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type Mod = Mod9;

fn powers(n: usize, mul: Mod) -> Vec<Mod> {
    Mod::gen_powers(mul, n + 1)
}

#[derive(Default, Clone)]
struct Node {
    hashes: Vec<Mod>,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        let sz = l.hashes.len();
        let mut max_bit = 0;
        while (1 << max_bit) != sz {
            max_bit += 1;
        }
        let hashes = gen_vec(sz * 2, |xor: usize| {
            let part_xor = xor & (sz - 1);
            let left = l.hashes[part_xor];
            let right = r.hashes[part_xor];
            if xor & (1 << max_bit) == 0 {
                left * context[sz] + right
            } else {
                right * context[sz] + left
            }
        });
        Self { hashes }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        todo!()
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        todo!()
    }

    type Update = ();

    type Context = Vec<Mod>;
}

fn solve(input: &mut Input, _test_case: usize) {
    let h = input.usize();
    let n = 1 << h;
    let s = input.string();
    let powers = powers(n, Mod::new(239i32));
    let mut seg_tree = LazySegTree::new_f_with_context(
        n,
        &|pos| {
            let hash = Mod::new(s[pos]);
            Node { hashes: vec![hash] }
        },
        powers,
    );
    let best_xor = (0..n)
        .min_by(|xor1, xor2| -> Ordering {
            let mut node1 = 0;
            let mut node2 = 0;
            let mut diff_pos = 0;
            for cur_h in (0..h).rev() {
                let left_node_1 = if (1 << cur_h) & xor1 == 0 {
                    seg_tree.expert_get_left_node(node1)
                } else {
                    seg_tree.expert_get_right_node(node1)
                };
                let left_node_2 = if (1 << cur_h) & xor2 == 0 {
                    seg_tree.expert_get_left_node(node2)
                } else {
                    seg_tree.expert_get_right_node(node2)
                };
                let mask = (1 << (cur_h)) - 1;
                let full_left_node_hash_1 =
                    seg_tree.expert_get_node(left_node_1).hashes[xor1 & mask];
                let full_left_node_hash_2 =
                    seg_tree.expert_get_node(left_node_2).hashes[xor2 & mask];

                if full_left_node_hash_1 == full_left_node_hash_2 {
                    diff_pos |= 1 << cur_h;
                    node1 = seg_tree.expert_get_left_node(node1)
                        ^ seg_tree.expert_get_right_node(node1)
                        ^ left_node_1;
                    node2 = seg_tree.expert_get_left_node(node2)
                        ^ seg_tree.expert_get_right_node(node2)
                        ^ left_node_2;
                } else {
                    node1 = left_node_1;
                    node2 = left_node_2;
                }
            }
            s[diff_pos ^ xor1].cmp(&s[diff_pos ^ xor2])
        })
        .unwrap();
    let res = gen_vec(n, |pos| s[pos ^ best_xor]);
    out_line!(vec2str(&res));
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
