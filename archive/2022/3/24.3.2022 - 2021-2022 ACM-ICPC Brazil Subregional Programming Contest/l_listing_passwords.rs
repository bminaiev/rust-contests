//{"name":"L. Listing Passwords","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/L","interactive":false,"timeLimit":3000,"tests":[{"input":"5 2\n1??0?\n1 3\n2 4\n","output":"2\n"},{"input":"3 2\n???\n1 1\n1 3\n","output":"4\n"},{"input":"5 2\n1???0\n1 3\n3 5\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LListingPasswords"}}}

use std::mem;

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::modulo::{Mod7, Mod9};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

type HashMod = Mod9;

#[derive(Clone, Default, Copy, Debug)]
struct HashNode {
    hash: HashMod,
    hash_rev: HashMod,
    len: usize,
}

struct Context {
    pow: Vec<HashMod>,
}

impl LazySegTreeNodeSpec for HashNode {
    fn unite(l: &Self, r: &Self, context: &Context) -> Self {
        Self {
            hash: l.hash * context.pow[r.len] + r.hash,
            hash_rev: r.hash_rev * context.pow[l.len] + l.hash_rev,
            len: l.len + r.len,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.hash = HashMod::new(*update as i32);
        node.hash_rev = HashMod::new(*update as i32);
    }

    #[allow(unused)]
    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        *current = *add;
    }

    type Update = usize;
    type Context = Context;
}

fn solve_input(s: Vec<u8>, restrictions: &[(usize, usize)]) -> Mod7 {
    let n = s.len();

    let mut dsu = Dsu::new(n);
    let mut rnd = Random::new(787788);
    for &(fr, to) in restrictions.iter() {
        let mut not_changed = 0;
        while not_changed < 20 {
            not_changed += 1;
            let x = rnd.gen_in_range(fr..to);
            let y = to - (x - fr) - 1;
            if dsu.unite(x, y) {
                not_changed = 0;
            }
        }
    }

    let mut set_ids = gen_vec(n, |id| dsu.get(id));
    let mut inside_set = vec![vec![]; n];
    for v in 0..n {
        inside_set[set_ids[v]].push(v);
    }

    let powers = HashMod::gen_powers(HashMod::new(239017i32), n + 1);
    let context = Context { pow: powers };
    let mut seg_tree = LazySegTree::new_f_with_context(
        s.len(),
        &|pos| HashNode {
            hash: HashMod::new(set_ids[pos]),
            hash_rev: HashMod::new(set_ids[pos]),
            len: 1,
        },
        context,
    );

    for &(l, r) in restrictions.iter() {
        let len = r - l;
        let full = seg_tree.get(l..r);
        if full.hash != full.hash_rev {
            loop {
                let first_different = binary_search_first_true(0..len, |sz| -> bool {
                    // true if different
                    let h1 = seg_tree.get(l..l + sz + 1).hash;
                    let h2 = seg_tree.get(r - sz - 1..r).hash_rev;
                    h1 != h2
                });
                if first_different == len {
                    break;
                }
                let id1 = set_ids[l + first_different];
                let id2 = set_ids[r - first_different - 1];
                assert_ne!(id1, id2);
                let (smallest, largest) = if inside_set[id1].len() < inside_set[id2].len() {
                    (id1, id2)
                } else {
                    (id2, id1)
                };
                for &pos in inside_set[smallest].iter() {
                    set_ids[pos] = largest;
                    seg_tree.update(pos..pos + 1, largest);
                }
                let mut old = mem::replace(&mut inside_set[smallest], vec![]);
                inside_set[largest].append(&mut old);
            }
        }
    }

    type Mod = Mod7;
    let mut res = Mod::ONE;
    for set in inside_set.iter() {
        if set.is_empty() {
            continue;
        }
        let mut exist0 = false;
        let mut exist1 = false;
        for &v in set.iter() {
            if s[v] == b'1' {
                exist1 = true;
            }
            if s[v] == b'0' {
                exist0 = true;
            }
        }
        if exist0 && exist1 {
            return Mod::ZERO;
        }
        if exist0 || exist1 {
            continue;
        }
        res *= Mod::TWO;
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let _n = input.usize();
    let m = input.usize();
    let s = input.string();

    let restrictions = gen_vec(m, |_| (input.usize() - 1, input.usize()));

    let res = solve_input(s, &restrictions);

    out_line!(res);
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

fn stress() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = 300000; //rnd.gen_in_range(1..10);
        let mut s = vec![];
        for _ in 0..n {
            let x = rnd.gen_in_range(0..3);
            if x == 0 {
                s.push(b'0');
            } else if x == 1 {
                s.push(b'1');
            } else {
                s.push(b'?');
            }
        }
        assert_eq!(s.len(), n);
        let restrictions = gen_vec(300_000, |_| {
            let range = rnd.gen_nonempty_range(n);
            (range.start, range.end)
        });
        solve_input(s, &restrictions);
    }
}

fn stress2() {
    for it in 1.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        let n = rnd.gen_in_range(1..10);
        let mut s = vec![];
        for _ in 0..n {
            let x = rnd.gen_in_range(0..3);
            if x == 0 {
                s.push(b'0');
            } else if x == 1 {
                s.push(b'1');
            } else {
                s.push(b'?');
            }
        }
        assert_eq!(s.len(), n);
        let restrictions = gen_vec(rnd.gen_in_range(1..10), |_| {
            let range = rnd.gen_nonempty_range(n);
            (range.start, range.end)
        });
        solve_input(s, &restrictions);
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
}
//END MAIN
