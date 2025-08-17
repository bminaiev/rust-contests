//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::vec;

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;

type Mod = Mod_998_244_353;

#[derive(Debug, Clone)]
struct Object {
    value: i64,
    pos: usize,
}

#[derive(Debug, Clone)]
struct Query {
    l: usize,
    r: usize,
    k: i64,
    id: usize,
}

#[derive(Debug, Clone)]
struct Block {
    cnt: i64,
    sum: i64,
    min_value: i64,
    sum_mod: Mod,
    sum_inv: Mod,
}

impl Block {
    pub fn add_value(&mut self, value: i64, delta: i64, value_mod: Mod, value_inv: Mod) {
        self.cnt += delta;
        self.sum += value * delta;
        if delta == 1 {
            self.sum_mod += value_mod;
            self.sum_inv += value_inv;
        } else {
            self.sum_mod -= value_mod;
            self.sum_inv -= value_inv;
        }
    }
}

struct Decomposition {
    shift: usize,
    alive: Vec<i64>,
    values: Vec<i64>,
    values_inv: Vec<Mod>,
    blocks: Vec<Block>,
}

impl Decomposition {
    fn new(mut values: Vec<i64>, shift: usize) -> Self {
        let block_size = 1 << shift;
        let cnt_blocks = (values.len() + block_size - 1) / block_size;
        let mut values_inv = vec![Mod::ZERO; values.len()];
        for i in 0..values.len() {
            values_inv[i] = Mod::new(values[i] as i32).inv();
        }
        let mut blocks = vec![
            Block {
                cnt: 0,
                sum: 0,
                min_value: 0,
                sum_mod: Mod::ZERO,
                sum_inv: Mod::ZERO,
            };
            cnt_blocks + 1
        ];
        for i in 0..blocks.len() - 1 {
            blocks[i].min_value = values[i * block_size];
        }
        const INF: i64 = 1e10 as i64;
        blocks[cnt_blocks].min_value = INF;
        values.push(INF);
        Self {
            alive: vec![0; values.len()],
            blocks,
            values,
            shift,
            values_inv,
        }
    }

    fn add_value(&mut self, pos: usize, delta: i64) {
        self.alive[pos] += delta;
        let block_id = pos >> self.shift;
        self.blocks[block_id].add_value(
            self.values[pos],
            delta,
            Mod::new(self.values[pos] as i32),
            self.values_inv[pos],
        );
    }

    fn calc_sum_inv(&self, from_pos: usize) -> Mod {
        let block_id = from_pos >> self.shift;
        let to = ((block_id + 1) << self.shift).min(self.values_inv.len());
        let mut res = Mod::ZERO;
        for pos in from_pos..to {
            if self.alive[pos] > 0 {
                res += self.values_inv[pos];
            }
        }
        for block_id in block_id + 1..self.blocks.len() {
            res += self.blocks[block_id].sum_inv;
        }
        res
    }

    fn calc_ans(&self, mut k: i64) -> Mod {
        let mut cur_cnt = 0;
        for block_id in 0..self.blocks.len() - 1 {
            let next_min_value = self.blocks[block_id + 1].min_value;

            let here_cnt = self.blocks[block_id].cnt;

            let mut full_cost = cur_cnt * (next_min_value - self.blocks[block_id].min_value);
            full_cost += here_cnt * next_min_value - self.blocks[block_id].sum;

            if full_cost <= k {
                k -= full_cost;
                cur_cnt += here_cnt;
            } else {
                let cur_block_start = block_id << self.shift;
                let next_block_start = ((block_id + 1) << self.shift).min(self.values.len());
                let mut cur_value = self.blocks[block_id].min_value;
                for pos in cur_block_start..=next_block_start {
                    let next_value = self.values[pos];

                    let full_cost = cur_cnt * (next_value - cur_value);
                    if full_cost > k {
                        assert!(cur_cnt > 0);
                        let full_incr = k / cur_cnt;
                        cur_value += full_incr;
                        k -= full_incr * cur_cnt;

                        let mut res = self.calc_sum_inv(pos);
                        res += Mod::new(cur_value as i32).inv() * Mod::new((cur_cnt - k) as i32);
                        res += Mod::new((cur_value + 1) as i32).inv() * Mod::new(k as i32);
                        return res;
                    } else {
                        k -= full_cost;
                        if self.alive[pos] > 0 {
                            cur_cnt += 1;
                        }

                        cur_value = next_value;
                    }
                }
                unreachable!();
            }
        }
        unreachable!()
    }
}

const SHIFT: usize = 8;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let a = gen_vec(n, |pos| Object {
        value: input.i64(),
        pos,
    });
    let mut sorted = a.clone();
    sorted.sort_by_key(|x| x.value);
    let sorted_values = sorted.iter().map(|x| x.value).collect::<Vec<_>>();

    let mut pos_in_sorted = vec![0; n];
    for i in 0..n {
        pos_in_sorted[sorted[i].pos] = i;
    }

    let q = input.usize();
    let mut queries = vec![];
    for i in 0..q {
        queries.push(Query {
            l: input.usize() - 1,
            r: input.usize(),
            k: input.i64(),
            id: i,
        });
    }

    let mut decomp = Decomposition::new(sorted_values, SHIFT);

    const BLOCK_SIZE: usize = 320;
    queries.sort_by_key(|q| (q.l / BLOCK_SIZE, q.r));
    let mut ans = vec![Mod::ZERO; q];
    let mut cur_l = 0;
    let mut cur_r = 0;
    for query in queries.iter() {
        while cur_r < query.r {
            decomp.add_value(pos_in_sorted[cur_r], 1);
            cur_r += 1;
        }
        while cur_l > query.l {
            cur_l -= 1;
            decomp.add_value(pos_in_sorted[cur_l], 1);
        }
        while cur_l < query.l {
            decomp.add_value(pos_in_sorted[cur_l], -1);
            cur_l += 1;
        }
        while cur_r > query.r {
            cur_r -= 1;
            decomp.add_value(pos_in_sorted[cur_r], -1);
        }
        let sum_inv = decomp.calc_ans(query.k);
        ans[query.id] = sum_inv.inv() * Mod::new((query.r - query.l - 1) as i32);
    }
    for &x in ans.iter() {
        out.println(x);
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "j";
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
}
//END MAIN
