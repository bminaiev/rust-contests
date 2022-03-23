//{"name":"petr_10_f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"file","fileName":"hash.in","pattern":null},"output":{"type":"file","fileName":"hash.out","pattern":null},"languages":{"java":{"taskClass":"petr_10_f"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

const LEN: usize = 30;
const TABLE_SIZE: usize = 700_000;

fn find_collision_old(base: i64, m: i64) -> (i32, i32) {
    let mut rnd = Random::new(7877881);
    let mut mult = vec![1; LEN];
    for i in 1..LEN {
        mult[i] = ((mult[i - 1] as i128 * base as i128) % (m as i128)) as i64;
    }
    mult.reverse();
    const PARTS: usize = 2;
    assert_eq!(LEN % PARTS, 0);
    const PART_SIZE: usize = LEN / PARTS;
    let part_hashes = gen_vec(PARTS, |part_id| {
        gen_vec(1 << PART_SIZE, |mask| -> i64 {
            let mut res = 0;
            for bit in 0..LEN / 2 {
                if ((1 << bit) & mask) != 0 {
                    res += mult[bit + PART_SIZE * part_id];
                }
            }
            res % m
        })
    });
    let calc_hash = |key: i32| -> i64 {
        const PART_MASK: i32 = (1 << PART_SIZE) - 1;
        let mut res = 0;
        res += part_hashes[0][(key & PART_MASK) as usize];
        res += part_hashes[1][((key >> PART_SIZE) & PART_MASK) as usize];
        if res >= m {
            res - m
        } else {
            res
        }

        /*for part_id in 0..PARTS {
            let pos = (key >> (part_id * PART_SIZE)) & PART_MASK;
            res += part_hashes[part_id][pos as usize];
            if res >= m {
                res -= m;
            }
        }*/
        // res
    };

    let mut seen = vec![0i32; TABLE_SIZE];

    loop {
        let new_key = (rnd.gen_u64() & ((1 << LEN) - 1) as u64) as i32;
        let new_hash = calc_hash(new_key);
        let pos = (new_hash % (TABLE_SIZE as i64)) as usize;
        let old_key = seen[pos];
        if old_key != 0 && old_key != new_key && calc_hash(old_key) == new_hash {
            return (old_key, new_key);
        } else {
            seen[pos] = new_key;
        }
    }
}

fn find_collision_small(init_hashes: &[i64], m: i64) -> (i32, i32) {
    const LEN: usize = 30;
    assert_eq!(init_hashes.len(), LEN);

    let mut rnd = Random::new(7877881);
    const PARTS: usize = 2;
    assert_eq!(LEN % PARTS, 0);
    const PART_SIZE: usize = LEN / PARTS;
    let part_hashes = gen_vec(PARTS, |part_id| {
        gen_vec(1 << PART_SIZE, |mask| -> i64 {
            let mut res = 0;
            for bit in 0..LEN / 2 {
                if ((1 << bit) & mask) != 0 {
                    res += init_hashes[bit + PART_SIZE * part_id];
                }
            }
            res % m
        })
    });
    let calc_hash = |key: i32| -> i64 {
        const PART_MASK: i32 = (1 << PART_SIZE) - 1;
        let mut res = 0;
        res += part_hashes[0][(key & PART_MASK) as usize];
        res += part_hashes[1][((key >> PART_SIZE) & PART_MASK) as usize];
        if res >= m {
            res - m
        } else {
            res
        }
    };

    let mut seen = vec![0i32; TABLE_SIZE];

    loop {
        let new_key = (rnd.gen_u64() & ((1 << LEN) - 1) as u64) as i32;
        let new_hash = calc_hash(new_key);
        let pos = (new_hash % (TABLE_SIZE as i64)) as usize;
        let old_key = seen[pos];
        if old_key != 0 && old_key != new_key && calc_hash(old_key) == new_hash {
            return (old_key, new_key);
        } else {
            seen[pos] = new_key;
        }
    }
}

#[derive(Clone)]
enum NodeInner {
    Single(usize),
    Sub(Box<NodeInner>, Box<NodeInner>),
}

#[derive(Clone)]
struct Node {
    hash: i64,
    inner: NodeInner,
}

impl NodeInner {
    pub fn save_res(&self, idx: usize, res: &mut [Vec<i64>]) {
        match self {
            NodeInner::Single(pos) => {
                res[idx][*pos] ^= 1;
            }
            NodeInner::Sub(lhs, rhs) => {
                lhs.save_res(idx, res);
                rhs.save_res(1 - idx, res);
            }
        }
    }
}

fn find_collision(base: i64, m: i64) -> (Vec<i64>, Vec<i64>) {
    const MAX_LEN: usize = 1000;
    let mut mult = vec![1; MAX_LEN];
    for i in 1..mult.len() {
        mult[i] = ((mult[i - 1] as i128 * base as i128) % (m as i128)) as i64;
    }
    mult.reverse();
    let mut nodes = gen_vec(MAX_LEN, |pos| Node {
        hash: mult[pos],
        inner: NodeInner::Single(pos),
    });
    loop {
        nodes.sort_by_key(|node| node.hash);
        if nodes.is_empty() {
            unreachable!();
        }
        let median_pow10 = (nodes[nodes.len() / 2].hash as f64).log10();
        dbg!(nodes.len(), nodes.last().unwrap().hash, median_pow10);
        if nodes[0].hash == 0 {
            let mut res = vec![vec![0; MAX_LEN]; 2];
            nodes[0].inner.save_res(0, &mut res);
            return (res[0].clone(), res[1].clone());
        }
        // if nodes.len() < 70 {
        //     dbg!(nodes.len());
        //     for n in nodes.iter() {
        //         dbg!(n.hash);
        //     }
        // }
        if nodes.len() == 31 {
            nodes.pop();
            assert_eq!(nodes.len(), 30);
            let init_hashes = gen_vec(nodes.len(), |id| nodes[id].hash);
            let (k1, k2) = find_collision_small(&init_hashes, m);
            let mut res = vec![vec![0; MAX_LEN]; 2];
            for bit in 0..nodes.len() {
                if ((1 << bit) & k1) != 0 {
                    nodes[bit].inner.save_res(0, &mut res);
                }
                if ((1 << bit) & k2) != 0 {
                    nodes[bit].inner.save_res(1, &mut res);
                }
            }
            return (res[0].clone(), res[1].clone());
        }

        let mut next_nodes = vec![];
        for i in (0..nodes.len() - 1).step_by(2) {
            let new_hash = nodes[i + 1].hash - nodes[i].hash;
            let inner = NodeInner::Sub(
                Box::new(nodes[i + 1].inner.clone()),
                Box::new(nodes[i].inner.clone()),
            );
            next_nodes.push(Node {
                hash: new_hash,
                inner,
            });
        }
        nodes = next_nodes;
    }
}

fn stress() {
    let m = 100000000000031;
    let base = 2;
    assert!(base + 2 <= m);
    let (k1, k2) = find_collision(base, m);
    dbg!(k1, k2);
}

fn solve(input: &mut Input, _test_case: usize) {
    let (base, m) = input.read();
    let (k1, k2) = find_collision(base, m);
    for key in [k1, k2].iter() {
        for &bit in key.iter() {
            out!(bit);
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

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
        input: TaskIoType::File("hash.in".to_string()),
        output: TaskIoType::File("hash.out".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_locally();
    tester::run_stress(stress);
}
//END MAIN
