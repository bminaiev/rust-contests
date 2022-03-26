//{"name":"F. Paper Grading","group":"Codeforces - The 2019 ICPC Asia Nanjing Regional Contest","url":"https://codeforces.com/gym/103466/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\naaa\nbbb\naac\n2 aasdd 2 1 3\n2 aab 1 1 2\n1 2 3\n2 aat 2 1 2\n","output":"2\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPaperGrading"}}}

use std::cmp::min;
use std::time::Instant;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::fenwick::Fenwick;
use algo_lib::strings::suffix_array::SuffixArray;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

enum Oper {
    Swap(usize, usize),
    Query(Vec<u8>, usize, usize),
}

#[derive(Clone, Debug)]
struct Subquery {
    id: usize,
    mult: i32,
}

fn solve_case(s: &[Vec<u8>], queries: &[Oper]) -> Vec<usize> {
    let start = Instant::now();
    let mut big_string = vec![];
    let mut indexes = vec![];
    for s in s.iter() {
        indexes.push(big_string.len());
        big_string.append(&mut s.clone());
        big_string.push(b'$');
    }
    let mut query_indexes = vec![];
    for q in queries.iter() {
        match q {
            Oper::Swap(_, _) => query_indexes.push(0),
            Oper::Query(str, _, _) => {
                query_indexes.push(big_string.len());
                big_string.append(&mut str.clone());
                big_string.push(b'$');
            }
        }
    }

    let sa = SuffixArray::new(big_string);

    dbg!("finished sa", start.elapsed().as_millis());

    let mut sa_ranges = vec![];
    for (it, q) in queries.iter().enumerate() {
        match q {
            Oper::Swap(_, _) => sa_ranges.push((0, 0)),
            Oper::Query(str, _, _) => {
                let my_pos_in_sa = sa.get_pos_in_array(query_indexes[it]);
                let left = binary_search_first_true(0..my_pos_in_sa, |check_pos| {
                    sa.lcp(check_pos, my_pos_in_sa) >= str.len()
                });
                let right = binary_search_first_true(my_pos_in_sa..sa.len(), |check_pos| {
                    sa.lcp(check_pos, my_pos_in_sa) < str.len()
                });
                sa_ranges.push((left, right));
            }
        }
    }

    const BUBEN: usize = 2000;
    let mut perm = gen_vec(s.len(), id);
    let mut used_in_block = vec![false; s.len()];
    let mut used_in_block_vec = vec![];

    let mut res = vec![0i32; queries.len()];

    let mut where_perm = gen_vec(perm.len(), id);

    let mut fenw = Fenwick::new(sa.len());

    let mut subqueries = vec![vec![]; perm.len() + 1];

    let mut cc = 0;

    for start in (0..queries.len()).step_by(BUBEN) {
        let end = min(start + BUBEN, queries.len());

        for x in used_in_block.iter_mut() {
            *x = false;
        }

        for q in queries[start..end].iter() {
            if let &Oper::Swap(p1, p2) = q {
                used_in_block[perm[p1]] = true;
                used_in_block[perm[p2]] = true;
            }
        }

        used_in_block_vec.clear();
        for (id, &val) in used_in_block.iter().enumerate() {
            if val {
                cc += 1;
                used_in_block_vec.push((id, sa.get_pos_in_array(indexes[id])));
            }
        }

        for x in subqueries.iter_mut() {
            x.clear();
        }

        for (id, q) in queries[start..end].iter().enumerate() {
            let id = id + start;
            if let &Oper::Swap(p1, p2) = q {
                used_in_block[perm[p1]] = true;
                used_in_block[perm[p2]] = true;
                perm.swap(p1, p2);
                where_perm[perm[p1]] = p1;
                where_perm[perm[p2]] = p2;
            } else if let Oper::Query(_str, l, r) = q {
                for &(another, in_sa) in used_in_block_vec.iter() {
                    let position = where_perm[another];
                    if position >= *l && position < *r {
                        // let in_sa = sa.get_pos_in_array(indexes[another]);
                        if in_sa >= sa_ranges[id].0 && in_sa < sa_ranges[id].1 {
                            res[id] += 1;
                        }
                    }
                }
                subqueries[*l].push(Subquery { id, mult: -1 });
                subqueries[*r].push(Subquery { id, mult: 1 });
            }
        }

        fenw.clear();

        for pos in 0..=perm.len() {
            for subq in subqueries[pos].iter() {
                let id = subq.id;
                let (fr, to) = sa_ranges[id];
                res[id] += subq.mult * (fenw.get_range_sum(fr..to));
            }
            if pos == perm.len() {
                break;
            }
            let what = perm[pos];
            if !used_in_block[what] {
                cc += 1;
                let in_sa = sa.get_pos_in_array(indexes[what]);
                fenw.add(in_sa, 1);
            }
        }
    }

    // dbg!(cc);

    let mut real_res = vec![];
    for i in 0..res.len() {
        if let Oper::Query(_, _, _) = queries[i] {
            real_res.push(res[i] as usize);
        }
    }
    real_res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let s = gen_vec(n, |_| input.string());
    let queries = gen_vec(m, |_| {
        let q_type = input.usize();
        if q_type == 1 {
            let p1 = input.usize() - 1;
            let p2 = input.usize() - 1;
            Oper::Swap(p1, p2)
        } else {
            assert_eq!(q_type, 2);
            let mut s = input.string();
            let pref_len = input.usize();
            s.truncate(pref_len);
            let l = input.usize() - 1;
            let r = input.usize();
            Oper::Query(s, l, r)
        }
    });
    let res = solve_case(&s, &queries);
    for res in res.iter() {
        out_line!(*res);
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

fn stress() {
    let mut rnd = Random::new(787788);
    const MAX: usize = 200_000;
    let s = gen_vec(MAX, |_| rnd.gen_vec(1, b'a'..b'z'));
    let queries = gen_vec(MAX, |_| {
        if rnd.gen_bool() {
            let p1 = rnd.gen_in_range(0..s.len());
            let p2 = rnd.gen_in_range(0..s.len());
            Oper::Swap(p1, p2)
        } else {
            let str = rnd.gen_vec(1, b'a'..b'z');
            let range = rnd.gen_nonempty_range(s.len());
            Oper::Query(str, range.start, range.end)
        }
    });
    solve_case(&s, &queries);
}

fn main() {
    tester::run_tests();
    // tester::run_stress(stress);
    // tester::run_single_test("1");
}
//END MAIN
