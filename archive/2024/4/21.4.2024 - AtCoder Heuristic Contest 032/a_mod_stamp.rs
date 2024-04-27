//{"name":"A - Mod Stamp","group":"AtCoder - AtCoder Heuristic Contest 032","url":"https://atcoder.jp/contests/ahc032/tasks/ahc032_a","interactive":false,"timeLimit":2000,"tests":[{"input":"9 20 81\n24323530 980293589 859258684 185499104 894688371 236405725 111530575 250271104 495624658\n769596495 264300425 88876278 146578260 565437828 737999180 725732147 57726456 323844609\n40096771 928203404 501627737 804865949 814572382 849529199 189832922 910184599 467494517\n962420139 432607222 59818053 858072870 914485919 446805915 138548911 345246064 245004268\n477044564 12166358 931360092 799278793 865992483 339109407 614502753 736626962 801948371\n281576446 640350568 771040910 823574138 350308411 930294372 585808288 700370699 426021090\n289960346 566527193 119082954 148354804 902944838 516738876 930961873 812731496 172242940\n921564237 662077279 49476329 593121834 377147282 862136854 791213996 686329230 7403815\n501340655 979965930 839183331 303883398 490179686 492481098 160122974 114672637 82049594\n975741402 918762324 476374754\n906657349 359110092 978536040\n84599745 368692094 744129488\n261705356 216870728 556481274\n317767465 457532475 532110106\n125703669 839188333 425571806\n291667039 37052662 1276219\n305291998 653050074 220563016\n332525785 400712871 520185762\n393148157 178758620 933441647\n205044518 579917402 498932315\n411369672 664953833 274696537\n654712800 802006144 682742340\n864455037 533661060 207561332\n605472509 577911453 942938903\n576270626 688256275 33493069\n481710779 902547317 817131623\n291465541 863597953 772086608\n417987422 136453150 615090472\n760882895 841541285 914039365\n359505208 780663578 774735965\n188919347 431579412 464452916\n854985721 70294202 663019966\n157776983 3557297 439447307\n621014939 759908222 932643321\n184225959 884108948 693640679\n361651737 846036661 975413204\n479224933 700946167 622558051\n495003914 325785117 513339213\n70238660 857642866 297571112\n374937799 48000646 849682071\n528095305 232520890 469018467\n952599070 610262715 232403912\n316958602 24859140 385411996\n304561106 853230688 859071983\n266806117 99442261 881952734\n708824083 752081152 915353520\n261135036 48934653 945657700\n255395109 742827901 445178710\n906120195 565840603 316740986\n736297599 447489530 680619574\n654670835 694926131 897183420\n958993686 813942152 196144122\n324334792 928014325 852381591\n194958307 642660824 128931372\n303306950 687790222 930130148\n591510740 614681348 113389792\n160195595 683240268 555351204\n218729338 196609467 724290289\n47413572 552092134 337674489\n410209863 549012244 186533965\n452647000 449090484 733453206\n106059177 888943736 940915649\n692940521 382797569 893532614\n52383100 783583840 634565824\n168433778 751831139 356971915\n870682287 872212766 75893565\n262231629 844472478 843213274\n499286296 502562654 725538734\n467780532 720085509 907848638\n","output":"4\n0 1 6\n6 6 6\n18 6 1\n16 1 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AModStamp"}}}

use std::io::Write;
use std::time::Instant;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::min_priority_queue::MinPriorityQueue;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable2, RecursiveFunction2};

type Mod = Mod_998_244_353;
type Stamp = [[Mod; 3]; 3];

#[derive(Clone)]
struct Test {
    a: Array2D<Mod>,
    stamps: Vec<Stamp>,
    max_ops: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Op {
    stamp_id: usize,
    i: usize,
    j: usize,
}

struct Res {
    score: i64,
    ops: Vec<Op>,
}

fn apply(a: &mut Array2D<Mod>, stamp: &Stamp, rev: bool, start_i: usize, start_j: usize) {
    if rev {
        for i in 0..3 {
            for j in 0..3 {
                a[start_i + i][start_j + j] -= stamp[i][j];
            }
        }
    } else {
        for i in 0..3 {
            for j in 0..3 {
                a[start_i + i][start_j + j] += stamp[i][j];
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    i: usize,
    j: usize,
}

#[derive(Clone)]
struct State {
    test: Test,
    a: Array2D<Mod>,
    ops: Vec<Op>,
}

impl State {
    fn apply(&mut self, res: &Res) {
        for op in &res.ops {
            self.ops.push(*op);
            apply(
                &mut self.a,
                &self.test.stamps[op.stamp_id],
                false,
                op.i,
                op.j,
            );
        }
    }

    fn to_res(&self) -> Res {
        let mut score = 0;
        for i in 0..self.a.len() {
            for j in 0..self.a.len() {
                score += self.a[i][j].i64();
            }
        }
        assert!(self.ops.len() <= self.test.max_ops);
        Res {
            score,
            ops: self.ops.clone(),
        }
    }

    fn local_solve(&mut self, starts: &[Pos], to_optimize: &[Pos]) -> Res {
        if starts.is_empty() {
            let mut res = 0;
            for p in to_optimize {
                res += self.a[p.i][p.j].i64();
            }
            Res {
                score: res,
                ops: vec![],
            }
        } else {
            let start_pos = starts[0];
            let mut best_res = self.local_solve(&starts[1..], to_optimize);
            for stamp_id in 0..self.test.stamps.len() {
                apply(
                    &mut self.a,
                    &self.test.stamps[stamp_id],
                    false,
                    start_pos.i,
                    start_pos.j,
                );
                let mut res = self.local_solve(&starts[1..], to_optimize);
                if res.score > best_res.score {
                    res.ops.push(Op {
                        stamp_id,
                        i: start_pos.i,
                        j: start_pos.j,
                    });
                    best_res = res;
                }
                apply(
                    &mut self.a,
                    &self.test.stamps[stamp_id],
                    true,
                    start_pos.i,
                    start_pos.j,
                );
            }
            best_res
        }
    }
}

fn solve_top_left_5x5(state: &mut State) {
    const PER_LAYER: usize = 100;
    let mut cur_layer = vec![(0, state.clone())];
    for start_i in 0..5 {
        for start_j in 0..5 {
            let mut next_layer = vec![];
            for (score, cur_state) in cur_layer.iter() {
                for stamp_id in 0..state.test.stamps.len() {
                    let mut new_state = cur_state.clone();
                    apply(
                        &mut new_state.a,
                        &state.test.stamps[stamp_id],
                        false,
                        start_i,
                        start_j,
                    );
                    new_state.ops.push(Op {
                        stamp_id,
                        i: start_i,
                        j: start_j,
                    });
                    let new_score = score + new_state.a[start_i][start_j].i64();
                    next_layer.push((new_score, new_state));
                }
            }
            next_layer.sort_by_key(|x| -x.0);
            next_layer.truncate(PER_LAYER);
            cur_layer = next_layer;
        }
    }
    *state = cur_layer[0].1.clone();
}

fn solve_first_part(state: &mut State, rnd: &mut Random) {
    let n = state.a.len();
    state.ops.clear();
    state.a = state.test.a.clone();
    for _it in 0..1 {
        let stamp_id = rnd.gen(0..state.test.stamps.len());
        state.apply(&Res {
            score: 0,
            ops: vec![Op {
                stamp_id,
                i: 0,
                j: 0,
            }],
        });
    }
    solve_top_left_5x5(state);
    for start_i in 0..5 {
        let mut to_optimize = vec![];
        for j in 5..n {
            to_optimize.push(Pos { i: start_i, j });
        }
        let starts = vec![
            Pos { i: start_i, j: 5 },
            Pos { i: start_i, j: 5 },
            Pos { i: start_i, j: 6 },
            Pos { i: start_i, j: 6 },
        ];
        let res = state.local_solve(&starts, &to_optimize);
        state.apply(&res);
    }
    for start_j in 0..6 {
        let mut to_optimize = vec![];
        for i in 5..n {
            to_optimize.push(Pos { i, j: start_j });
        }
        let starts = vec![
            Pos { i: 5, j: start_j },
            Pos { i: 5, j: start_j },
            Pos { i: 6, j: start_j },
            Pos { i: 6, j: start_j },
        ];
        let res = state.local_solve(&starts, &to_optimize);
        state.apply(&res);
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct BeamState {
    score: i64,
    a: Array2D<Mod>,
    ops: Vec<Op>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SmallBeamState {
    score: i64,
    prev_ops: usize,
    prev_idx: usize,
    applied_ops_idx: usize,
}

#[derive(Clone)]
struct SameCellOps {
    ops: Vec<Op>,
    add: Vec<Mod>,
}

#[derive(Clone)]
struct MaxLimitedQueue<T: Ord + Clone> {
    max_size: usize,
    // sorted from best to worst
    v: MinPriorityQueue<T>,
}

impl<T: Clone + PartialOrd + Ord> MaxLimitedQueue<T> {
    fn new(max_size: usize) -> Self {
        Self {
            max_size,
            v: MinPriorityQueue::new(),
        }
    }

    fn maybe_insert(&mut self, x: &T) -> usize {
        if self.v.len() == self.max_size && self.v.peek().unwrap() >= x {
            return 0;
        }
        let mut res = 1;
        self.v.push(x.clone());
        if self.v.len() > self.max_size {
            self.v.pop();
        }
        // for i in (0..self.v.len() - 1).rev() {
        //     if self.v[i] < self.v[i + 1] {
        //         self.v.swap(i, i + 1);
        //         res += 1;
        //     } else {
        //         break;
        //     }
        // }
        // self.v.truncate(self.max_size);
        res
    }
}

fn solve_first_part2(state: &mut State, _rnd: &mut Random) {
    let test = &state.test;
    let num_stamps = state.test.stamps.len();
    let max_ops = state.test.max_ops;
    const MAX_QUEUE_SIZE: usize = 80;
    let mut queues = vec![Vec::new(); max_ops + 1];
    queues[0].push(BeamState {
        score: 0,
        a: state.a.clone(),
        ops: vec![],
    });
    let mut updates = 0;
    let mut sum_updates_len = 0;
    let mut run_beam_search = |pos: Pos, queues: &[Vec<BeamState>]| -> Vec<Vec<BeamState>> {
        let mut new_queues =
            vec![MaxLimitedQueue::<SmallBeamState>::new(MAX_QUEUE_SIZE); queues.len()];

        let mut score_positions = vec![];
        let max_i = if pos.i == 6 { 9 } else { pos.i + 1 };
        let max_j = if pos.j == 6 { 9 } else { pos.j + 1 };
        for i in pos.i..max_i {
            for j in pos.j..max_j {
                score_positions.push(Pos { i, j });
            }
        }
        // dbg!(score_positions);

        let mut same_cell_ops = vec![];
        let max_cell_ops = if pos.i >= 5 && pos.j >= 6 { 4 } else { 2 };
        let mut ops = vec![];
        RecursiveFunction2::new(|f, more: usize, from: usize| {
            if more > 0 {
                for i in from..num_stamps {
                    ops.push(Op {
                        stamp_id: i,
                        i: pos.i,
                        j: pos.j,
                    });
                    f.call(more - 1, i);
                    ops.pop();
                }
            }
            let mut a = Array2D::new(Mod::ZERO, state.a.len(), state.a.len());
            for op in ops.iter() {
                apply(&mut a, &state.test.stamps[op.stamp_id], false, pos.i, pos.j);
            }
            let mut add = vec![Mod::ZERO; score_positions.len()];
            for i in 0..score_positions.len() {
                add[i] = a[score_positions[i].i][score_positions[i].j];
            }
            same_cell_ops.push(SameCellOps {
                ops: ops.clone(),
                add,
            });
        })
        .call(max_cell_ops, 0);
        // dbg!(same_cell_ops.len());

        let mut cur_state = vec![Mod::ZERO; score_positions.len()];
        for prev_ops in 0..queues.len() {
            let cur_queue = &queues[prev_ops];
            for (prev_idx, beam_state) in cur_queue.iter().enumerate() {
                for i in 0..score_positions.len() {
                    cur_state[i] = beam_state.a[score_positions[i].i][score_positions[i].j];
                }
                let sz = cur_state.len();
                for (applied_ops_idx, same_cell_op) in same_cell_ops.iter().enumerate() {
                    let mut score = beam_state.score;
                    for (&x, &y) in cur_state[..sz].iter().zip(same_cell_op.add[..sz].iter()) {
                        score += (x + y).i64();
                    }
                    let new_ops = prev_ops + same_cell_op.ops.len();
                    if new_ops < max_ops {
                        let upd_len = new_queues[new_ops].maybe_insert(&SmallBeamState {
                            score,
                            prev_ops,
                            prev_idx,
                            applied_ops_idx,
                        });
                        if upd_len > 0 {
                            sum_updates_len += upd_len;
                            updates += 1;
                        }
                    }
                }
            }
        }

        let mut applied_new_queues = vec![Vec::new(); max_ops + 1];
        for ops in 0..new_queues.len() {
            for small_beam_state in new_queues[ops].v.iter() {
                let mut state =
                    queues[small_beam_state.prev_ops][small_beam_state.prev_idx].clone();
                for &op in same_cell_ops[small_beam_state.applied_ops_idx].ops.iter() {
                    apply(&mut state.a, &test.stamps[op.stamp_id], false, pos.i, pos.j);
                    state.ops.push(op);
                }
                state.score = small_beam_state.score;
                applied_new_queues[ops].push(state);
            }
        }

        applied_new_queues
    };
    for diag in 0..=6 {
        for row in diag..=6 {
            queues = run_beam_search(Pos { i: row, j: diag }, &queues);
        }
        // if diag != 5 {
        for col in diag + 1..=6 {
            queues = run_beam_search(Pos { i: diag, j: col }, &queues);
        }
        // }
    }
    dbg!(sum_updates_len);
    dbg!(updates);
    let mut best: Option<BeamState> = None;
    for q in queues.iter() {
        for x in q.iter() {
            if best.is_none() || x.score > best.as_ref().unwrap().score {
                best = Some(x.clone());
            }
        }
    }
    let best = best.unwrap();
    dbg!(best.score / 100_000_000);
    state.a = best.a;
    state.ops = best.ops;
}

fn solve_case(test: &Test) -> Res {
    let mut state = State {
        test: test.clone(),
        a: test.a.clone(),
        ops: vec![],
    };
    let n = test.a.len();
    let mut rnd = Random::new(787788);
    let mut best = (0, state.clone());
    let start = Instant::now();
    while start.elapsed().as_millis() < 1000 {
        solve_first_part2(&mut state, &mut rnd);
        let mut pref_score = 0;
        for i in 0..n {
            for j in 0..n {
                if i >= 5 && j >= 6 {
                    continue;
                }
                pref_score += state.a[i][j].i64();
            }
        }
        if pref_score > best.0 {
            best = (pref_score, state.clone());
            dbg!(pref_score / 100_000_000);
            // TODO: remove
            break;
        }
    }
    state = best.1;

    // {
    //     let start = Pos { i: n - 4, j: n - 3 };
    //     let mut to_optimize = vec![];
    //     for j in n - 3..n {
    //         to_optimize.push(Pos { i: n - 4, j });
    //     }
    //     let res = state.local_solve(&[start, start, start, start], &to_optimize);
    //     state.apply(&res);
    // }

    // {
    //     let start = Pos { i: n - 3, j: n - 3 };
    //     let mut to_optimize = vec![];
    //     for i in n - 3..n {
    //         for j in n - 3..n {
    //             to_optimize.push(Pos { i, j });
    //         }
    //     }
    //     let res = state.local_solve(&[start, start, start, start], &to_optimize);
    //     state.apply(&res);
    // }

    // let extra_score = meet_in_the_middle(&mut state);
    // // dbg!(best.0, extra_score);
    // let expected_score = best.0 + extra_score;
    // assert_eq!(expected_score, state.to_res().score);

    state.to_res()
}

fn meet_in_the_middle(state: &mut State) -> i64 {
    let mut common = vec![];
    for i in 6..8 {
        for j in 6..9 {
            common.push(Pos { i, j });
        }
    }
    assert_eq!(common.len(), 6);
    let up = calc_half(
        state,
        Pos { i: 5, j: 6 },
        &[Pos { i: 5, j: 6 }, Pos { i: 5, j: 7 }, Pos { i: 5, j: 8 }],
        &common,
        state.a.clone(),
    );
    let mut down_start_a = state.a.clone();
    for p in common.iter() {
        down_start_a[p.i][p.j] = Mod::ZERO;
    }
    let down = calc_half(
        state,
        Pos { i: 6, j: 6 },
        &[Pos { i: 8, j: 6 }, Pos { i: 8, j: 7 }, Pos { i: 8, j: 8 }],
        &common,
        down_start_a,
    );
    let mut best = (0, up[0].clone(), down[0].clone());
    for up in up.iter() {
        for down in down.iter() {
            let mut score = up.my_score + down.my_score;
            // dbg!(up.my_score, down.my_score);
            for i in 0..6 {
                score += (up.common[i] + down.common[i]).i64();
            }
            // dbg!(score);
            if score > best.0 {
                best = (score, up.clone(), down.clone());
            }
        }
    }
    for half in [best.1, best.2].iter() {
        for op in half.ops.iter() {
            apply(
                &mut state.a,
                &state.test.stamps[op.stamp_id],
                false,
                op.i,
                op.j,
            );
            state.ops.push(*op);
        }
    }
    // {
    //     let mut tmp_score = 0;
    //     for i in 5..9 {
    //         for j in 6..9 {
    //             tmp_score += state.a[i][j].i64();
    //         }
    //     }
    //     dbg!(tmp_score);
    // }
    best.0
}

fn calc_half(
    state: &State,
    start: Pos,
    my: &[Pos],
    common: &[Pos],
    mut start_a: Array2D<Mod>,
) -> Vec<Half> {
    let mut res = vec![];
    let sz = state.test.stamps.len();
    let mut to_check = vec![];
    for i1 in 0..sz {
        for i2 in i1..sz {
            for i3 in i2..sz {
                for i4 in i3..sz {
                    for i5 in i4..sz {
                        for i6 in i5..sz {
                            for i7 in i6..sz {
                                let ids = [i1, i2, i3, i4, i5, i6, i7];
                                let mut ops = vec![];
                                for &id in ids.iter() {
                                    ops.push(Op {
                                        stamp_id: id,
                                        i: start.i,
                                        j: start.j,
                                    });
                                }
                                to_check.push(ops);
                            }
                        }
                    }
                }
            }
        }
    }

    for ops in to_check.iter() {
        for &op in ops.iter() {
            apply(
                &mut start_a,
                &state.test.stamps[op.stamp_id],
                false,
                start.i,
                start.j,
            );
        }
        let mut my_score = 0;
        for &p in my.iter() {
            my_score += start_a[p.i][p.j].i64();
        }
        let mut common_cells = [Mod::ZERO; 6];
        for (i, &p) in common.iter().enumerate() {
            common_cells[i] = start_a[p.i][p.j];
        }
        res.push(Half {
            my_score,
            common: common_cells,
            ops: ops.clone(),
        });

        for &op in ops.iter() {
            apply(
                &mut start_a,
                &state.test.stamps[op.stamp_id],
                true,
                start.i,
                start.j,
            );
        }
    }

    res.sort_by_key(|x| -x.my_score);
    // TODO: make bigger
    const LEFT: usize = 5000;
    res.truncate(LEFT);
    res
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Half {
    my_score: i64,
    common: [Mod; 6],
    ops: Vec<Op>,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let max_ops = input.usize();
    let a = Array2D::new_f(n, n, |_, _| Mod::new(input.i32()));
    let stamps = gen_vec(m, |_| {
        let mut stamp = [[Mod::new(0); 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                stamp[i][j] = Mod::new(input.i32());
            }
        }
        stamp
    });
    let test = Test { a, stamps, max_ops };
    let res = solve_case(&test);
    // dbg!(res.score / 1_000_000_000);
    out.println(res.ops.len());
    for op in res.ops {
        out.println(format!("{} {} {}", op.stamp_id, op.i, op.j));
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn gen(seed: u64) -> Test {
    use rand::Rng;
    use rand::SeedableRng;

    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let N = 9;
    let M = 20;
    let K = 81;
    let mut a = Array2D::new(Mod::ZERO, N, N);
    const MOD: i64 = 998_244_353;
    for i in 0..N {
        for j in 0..N {
            a[i][j] = Mod::new(rng.gen_range(0..MOD));
        }
    }
    let mut s = vec![[[Mod::ZERO; 3]; 3]; M];
    for i in 0..M {
        for j in 0..3 {
            for k in 0..3 {
                s[i][j][k] = Mod::new(rng.gen_range(0..MOD));
            }
        }
    }
    Test {
        a,
        stamps: s,
        max_ops: K as usize,
    }
}

fn stress() {
    let mut sum_scores = 0;
    const SHIFT: u64 = 0;
    for seed in SHIFT..1 + SHIFT {
        let test = gen(seed);
        let res = solve_case(&test);
        sum_scores += res.score;
        dbg!(seed, res.score / 100_000_000);
        {
            let mut f = Output::new_file("a_mod_stamp/res.txt");
            f.println(res.ops.len());
            for op in res.ops {
                f.println(format!("{} {} {}", op.stamp_id, op.i, op.j));
            }
            f.flush();
        }
    }
    dbg!(sum_scores / 1_000_000_000);
}

fn main() {
    const PROBLEM_NAME: &str = "a_mod_stamp";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
