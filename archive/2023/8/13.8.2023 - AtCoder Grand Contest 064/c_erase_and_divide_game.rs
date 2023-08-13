//{"name":"C - Erase and Divide Game","group":"AtCoder - AtCoder Grand Contest 064","url":"https://atcoder.jp/contests/agc064/tasks/agc064_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n5 7\n1\n0 100\n10\n1312150450968413 28316250877914571\n74859962623690078 84324828731963974\n148049062628894320 252509054433933439\n269587449430302150 335408917861648766\n349993004923078531 354979173822804781\n522842184971407769 578223540024979436\n585335723211047194 615812229161735895\n645762258982631926 760713016476190622\n779547116602436424 819875141880895723\n822981260158260519 919845426262703496\n","output":"Aoki\nAoki\nTakahashi\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CEraseAndDivideGame"}}}

use std::ops::Range;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable, Callable2, RecursiveFunction, RecursiveFunction2};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct SubRange {
    prefix: i64,
    more_bits: usize,
}

const MAX_BIT: usize = 61;

fn split_range(r: Range<i64>) -> Vec<SubRange> {
    let mut res = vec![];
    // rec: prefix, more_bits
    let mut rec = RecursiveFunction2::new(|f, prefix: i64, more_bits: usize| {
        let from = prefix << more_bits;
        let to = from + (1 << more_bits);
        if to <= r.start {
            return;
        }
        if from >= r.end {
            return;
        }
        if from >= r.start && to <= r.end {
            res.push(SubRange { prefix, more_bits });
            return;
        }
        assert!(more_bits > 0);
        for bit in 0..2 {
            f.call(prefix * 2 + bit, more_bits - 1);
        }
    });
    rec.call(0, MAX_BIT);
    res
}

#[derive(Clone, Copy, Debug)]
struct Node {
    next: [i32; 2],
    alive: bool,
}

const NULL: i32 = -1;

#[derive(Default)]
struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new_node(&mut self) -> i32 {
        let id = self.nodes.len() as i32;
        self.nodes.push(Node {
            next: [NULL; 2],
            alive: false,
        });
        id
    }

    pub fn add(&mut self, mut root: i32, mut prefix: i64) {
        self.nodes[root as usize].alive = true;
        let mut start = 61;
        while prefix != 0 || start > 0 {
            let first_bit = prefix & 1;
            prefix /= 2;
            if self.nodes[root as usize].next[first_bit as usize] == NULL {
                self.nodes[root as usize].next[first_bit as usize] = self.new_node();
            }
            root = self.nodes[root as usize].next[first_bit as usize];
            self.nodes[root as usize].alive = true;
            start -= 1;
        }
    }
}

fn is_first_winning(segs: &[Range<i64>]) -> bool {
    let mut trie = Trie::default();
    let mut roots = vec![];
    for _bit in 0..=MAX_BIT {
        roots.push(trie.new_node());
    }
    for r in segs {
        for sr in split_range(r.clone()) {
            let root = roots[sr.more_bits];
            // dbg!(root, sr);
            trie.add(root, sr.prefix);
        }
    }
    // dbg!(trie.nodes.len());

    for lvl in (0..roots.len() - 1).rev() {
        let root = roots[lvl];
        let next_root = roots[lvl + 1];
        for bit in 0..2 {
            if trie.nodes[next_root as usize].alive {
                trie.nodes[root as usize].alive = true;
            }
            if trie.nodes[root as usize].next[bit] == NULL {
                trie.nodes[root as usize].next[bit] = next_root;
            } else {
                let v = trie.nodes[root as usize].next[bit];
                RecursiveFunction2::new(|f, v: i32, teleport: i32| {
                    if teleport == NULL {
                        return;
                    }
                    if trie.nodes[teleport as usize].alive {
                        trie.nodes[v as usize].alive = true;
                    }
                    for bit in 0..2 {
                        if trie.nodes[v as usize].next[bit] == NULL {
                            trie.nodes[v as usize].next[bit] =
                                trie.nodes[teleport as usize].next[bit];
                        } else {
                            f.call(
                                trie.nodes[v as usize].next[bit],
                                trie.nodes[teleport as usize].next[bit],
                            );
                        }
                    }
                })
                .call(v, next_root)
            }
        }
    }
    let mut is_first_cache = vec![0u8; trie.nodes.len()];
    let is_first_win = RecursiveFunction::new(|f, v: usize| -> bool {
        if v as i32 == NULL || !trie.nodes[v].alive {
            return false;
        }
        if is_first_cache[v] != 0 {
            return is_first_cache[v] == 1;
        }
        let mut is_first_win = false;
        for bit in 0..2 {
            let next = trie.nodes[v].next[bit];
            if next == NULL {
                is_first_win = true;
            }
            if !f.call(next as usize) {
                is_first_win = true;
            }
        }
        if is_first_win {
            is_first_cache[v] = 1;
        } else {
            is_first_cache[v] = 2;
        }
        // dbg!("is_first_win", v, is_first_win);
        is_first_win
    })
    .call(roots[0] as usize);

    is_first_win
}

#[test]
fn tests() {
    assert!(is_first_winning(&[0..4]));
    assert!(!is_first_winning(&[0..2]));
}

fn stress2() {
    dbg!(is_first_winning(&[1..3, 5..8]));
}

fn stress() {
    let mut rnd = Random::new(898899);
    let mut values = vec![];
    const MAX_V: u64 = 1e18 as u64;
    for _ in 0..2 * 10_000 {
        values.push((rnd.gen_u64() % MAX_V) as i64);
    }
    let mut segs = vec![];
    for i in (0..values.len()).step_by(2) {
        segs.push(values[i]..values[i + 1] + 1);
    }
    is_first_winning(&segs);
}

fn solve(input: &mut Input) {
    let tc = input.usize();
    for _ in 0..tc {
        let n = input.usize();
        let mut segs = vec![];
        for _i in 0..n {
            let fr = input.i64();
            let to = input.i64();
            segs.push(fr..to + 1);
        }
        if is_first_winning(&segs) {
            out_line!("Takahashi");
        } else {
            out_line!("Aoki");
        }
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
