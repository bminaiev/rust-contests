//{"name":"task_e","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"task_e"}}}

use std::collections::HashMap;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::strings::utils::vec2str;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

const C: usize = 26;

fn gen_cnt(w: &[u8]) -> Vec<usize> {
    let mut res = vec![0; C];
    for c in w.iter() {
        res[(*c - b'a') as usize] += 1;
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let run_type_generate = input.string()[0] == b'p';
    let mut queries = vec![];
    if !run_type_generate {
        let cnt_queries = input.usize();
        queries = gen_vec(cnt_queries, |_| (input.string(), input.string()));
    }

    let mut to_gen: Vec<Vec<u8>> = vec![];

    if run_type_generate {
        let n = input.usize();
        to_gen = gen_vec(n, |_| input.string());
    }

    let n8 = input.usize();
    let words8 = gen_vec(n8, |_| input.string());
    let n4 = input.usize();
    let words4 = gen_vec(n4, |_| input.string());

    let mut words_ids = HashMap::new();
    for i in 0..words8.len() {
        words_ids.insert(words8[i].clone(), i);
    }

    let mut cnt4s = vec![];
    for i in 0..words4.len() {
        cnt4s.push(gen_cnt(&words4[i]));
    }

    let mut at_least_3 = vec![];
    let mut g = vec![vec![]; words8.len()];

    let mut used_in_8 = vec![0; words4.len()];

    for i in 0..words8.len() {
        let my_cnt = gen_cnt(&words8[i]);
        for j in 0..cnt4s.len() {
            let mut ok = true;
            let word = &words4[j];
            let word_cnt = &cnt4s[j];
            for c in word.iter() {
                let pos = (*c - b'a') as usize;
                ok &= word_cnt[pos] <= my_cnt[pos];
            }
            if ok {
                g[i].push(j);
                used_in_8[j] += 1;
            }
        }
        if g[i].len() >= 3 {
            at_least_3.push(i);
        }
    }

    let r = |x1: usize, x2: usize| -> (usize, usize) {
        if x1 < x2 {
            (x1, x2)
        } else {
            (x2, x1)
        }
    };

    let mut ids = gen_vec(words8.len(), id);
    ids.sort_by_key(|&id| g[id].len());
    for &bad in [6852, 6938].iter() {
        ids.remove(ids.iter().position(|x| *x == bad).unwrap());
        ids.insert(0, bad);
    }

    struct Mapping {
        from_key: HashMap<(usize, usize), usize>,
        used_4words: Vec<Vec<usize>>,
    }

    let gen = |seed: u64| -> Option<Mapping> {
        let mut from_key: HashMap<(usize, usize), usize> = HashMap::new();
        let mut rnd = Random::new(seed);

        rnd.gen_double();
        let bound = rnd.gen_double();
        let mut used_4words = vec![vec![]; words8.len()];

        for (iter, &i) in ids.iter().enumerate() {
            if g[i].len() < 3 {
                continue;
            }
            let can_use = &g[i];
            let next_word = if iter < 500 && rnd.gen_double() <= bound {
                0
            } else {
                1
            };
            let mut found = false;
            for i1 in 0..can_use.len() {
                for i2 in i1 + next_word..can_use.len() {
                    for i3 in i2 + next_word..can_use.len() {
                        if found {
                            break;
                        }
                        let x1 = can_use[i1];
                        let x2 = can_use[i2];
                        let x3 = can_use[i3];

                        let p1 = r(x1, x2);
                        let p2 = r(x1, x3);
                        let p3 = r(x2, x3);

                        if !from_key.contains_key(&p1)
                            && !from_key.contains_key(&p2)
                            && !from_key.contains_key(&p3)
                        {
                            found = true;
                            from_key.insert(p1, i);
                            from_key.insert(p2, i);
                            from_key.insert(p3, i);
                            used_4words[i].push(x1);
                            used_4words[i].push(x2);
                            used_4words[i].push(x3);
                        }
                    }
                }
            }
            if !found {
                return None;
            }
        }
        Some(Mapping {
            from_key,
            used_4words,
        })
    };

    let mapping = gen(1208).unwrap();

    if run_type_generate {
        for s in to_gen.iter() {
            let &id = words_ids.get(s).unwrap();
            let ids4 = &mapping.used_4words[id];
            for &cur_id in ids4.iter() {
                let to_print = vec2str(&words4[cur_id]);
                out!(to_print, "");
            }
            out_line!();
        }
    } else {
        let mut words_ids = HashMap::new();
        for i in 0..words4.len() {
            words_ids.insert(words4[i].clone(), i);
        }

        for (s1, s2) in queries.iter() {
            let id1 = *words_ids.get(s1).unwrap();
            let id2 = *words_ids.get(s2).unwrap();
            let key = r(id1, id2);

            let &word_id = mapping.from_key.get(&key).unwrap();
            out_line!(vec2str(&words8[word_id]));
        }
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
}
//END MAIN
