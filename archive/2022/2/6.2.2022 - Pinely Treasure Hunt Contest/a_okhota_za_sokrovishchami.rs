//{"name":"A. Охота за сокровищами","group":"Codeforces - Pinely Treasure Hunt Contest","url":"https://codeforces.com/contest/1639/problem/A","interactive":true,"timeLimit":5000,"tests":[],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AOkhotaZaSokrovishchami"}}}

use algo_lib::collections::bit_set::BitSet;
use algo_lib::collections::partial_cmp::PartialCmp;
use algo_lib::collections::peek_random::PeekRandom;
use algo_lib::collections::random_bag::RandomBag;
use algo_lib::collections::sorted::SortedTrait;
use algo_lib::graph::bfs::bfs;
use algo_lib::graph::compressed_graph::CompressedGraph;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::graph_readers::config::{Directional, Indexation};
use algo_lib::graph::graph_readers::simple::read_graph;
use algo_lib::graph::graph_trait::GraphTrait;
use algo_lib::graph::simple_algorithms::GraphAlgorithms;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::current_package::get_current_package_name;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::min_max::FindMinMaxPos;
use algo_lib::misc::rand::Random;
use algo_lib::strings::utils::byte2str;
use algo_lib::{dbg, out, out_line};

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub struct AdjVertex {
    seen: bool,
    degree: usize,
}

pub enum Move {
    EndGame,
    CurState(Vec<AdjVertex>),
}

pub trait Interactor {
    fn next_move_info(&mut self, input: &mut Input) -> Move;
    fn make_move(&mut self, pos: usize);
}

type Graph = CompressedGraph<SimpleEdge>;

struct FakeInteractor {
    graph: Graph,
    cur_v: usize,
    base_move_count: usize,
    seen: Vec<bool>,
    rnd: Random,
    next: Vec<usize>,
    done_moves: usize,
}

impl FakeInteractor {
    pub fn new(graph: Graph, start: usize, base_move_count: usize) -> Self {
        let n = graph.num_vertices();
        let mut seen = vec![false; n];
        seen[start] = true;
        Self {
            graph,
            cur_v: start,
            base_move_count,
            seen,
            rnd: Random::new(787788),
            next: vec![],
            done_moves: 0,
        }
    }
}

impl Interactor for FakeInteractor {
    fn next_move_info(&mut self, _input: &mut Input) -> Move {
        self.done_moves += 1;
        if self.done_moves > 2 * self.base_move_count {
            return Move::EndGame;
        }
        if self.seen.iter().all(|&x| x) {
            return Move::EndGame;
        }
        let perm = self.rnd.gen_permutation(self.graph.degree(self.cur_v));
        self.next.clear();
        let mut res = vec![];
        for &p in perm.iter() {
            let to = self.graph.adj(self.cur_v)[p].to();

            self.next.push(to);
            res.push(AdjVertex {
                seen: self.seen[to],
                degree: self.graph.degree(to),
            });
        }
        return Move::CurState(res);
    }

    fn make_move(&mut self, pos: usize) {
        let to = self.next[pos];
        self.cur_v = to;
        self.seen[to] = true;
    }
}

struct RealInteractor {}

impl Interactor for RealInteractor {
    fn next_move_info(&mut self, input: &mut Input) -> Move {
        let type_ = input.string()[0];
        if type_ == b'R' {
            let size = input.usize();
            let near = gen_vec(size, |_| {
                let deg = input.usize();
                let seen = input.usize() == 1;
                AdjVertex { degree: deg, seen }
            });
            return Move::CurState(near);
        } else if type_ == b'A' || type_ == b'F' {
            return Move::EndGame;
        } else {
            panic!("Strange move!");
        }
    }

    fn make_move(&mut self, d: usize) {
        out_line!(d + 1);
        output().flush();
    }
}

impl RealInteractor {
    pub fn new_real() -> Self {
        Self {}
    }
}

#[derive(Hash, Clone, Eq, PartialOrd, PartialEq)]
struct World {
    seen: BitSet,
    cur_v: usize,
    maybe_not_remove_duplicates: usize,
}

fn solve_interactor(
    graph: &Graph,
    interactor: &mut impl Interactor,
    input: &mut Input,
    start: usize,
) {
    let n = graph.num_vertices();

    let mut rnd = Random::new_time_seed();

    const MAX_WORLDS: usize = 20;
    let mut worlds = RandomBag::new(MAX_WORLDS, rnd.gen_u64());
    let mut world = World {
        seen: BitSet::new(n),
        cur_v: start,
        maybe_not_remove_duplicates: 0,
    };
    world.seen.set(start, true);
    worlds.insert(world);

    loop {
        match interactor.next_move_info(input) {
            Move::EndGame => {
                return;
            }
            Move::CurState(near) => {
                let near_sorted = near.sorted();
                worlds.filter(|world| -> bool {
                    let mut expect = vec![];
                    for e in graph.adj(world.cur_v) {
                        let to = e.to();
                        expect.push(AdjVertex {
                            degree: graph.degree(to),
                            seen: world.seen.get(to),
                        })
                    }
                    near_sorted == expect.sorted()
                });

                let go_pos = {
                    if worlds.is_empty() {
                        let mut candidates = vec![];

                        #[derive(Ord, PartialOrd, Eq, PartialEq)]
                        struct Candidate {
                            degree: usize,
                            rnd: u64,
                            pos: usize,
                        }

                        for (pos, near_v) in near.iter().enumerate() {
                            if !near_v.seen {
                                candidates.push(Candidate {
                                    degree: near_v.degree,
                                    rnd: rnd.gen_u64(),
                                    pos,
                                });
                            }
                        }
                        if let Some(best) = candidates.iter().min() {
                            best.pos
                        } else {
                            rnd.gen_index(&near)
                        }
                    } else {
                        let mut scores = vec![0; near.len()];
                        for _ in 0..5 {
                            let rand_world = worlds.peek_random().unwrap();
                            let bfs_state = bfs(rand_world.cur_v, graph);

                            #[derive(Ord, PartialOrd, Eq, PartialEq)]
                            struct Candidate {
                                dist: u32,
                                degree: usize,
                                rnd: u64,
                                v: usize,
                            }

                            let mut candidates = vec![];
                            for v in 0..n {
                                if !rand_world.seen.get(v) {
                                    candidates.push(Candidate {
                                        dist: bfs_state.dist[v],
                                        degree: graph.degree(v),
                                        rnd: rnd.gen_u64(),
                                        v,
                                    });
                                }
                            }
                            let need_v = candidates.iter().min().unwrap().v;
                            let chain = bfs_state.get_path(need_v).unwrap();
                            let next_v = chain[1];
                            let mut good_pos = vec![];
                            for i in 0..near.len() {
                                if near[i].seen == rand_world.seen.get(next_v)
                                    && graph.degree(next_v) == near[i].degree
                                {
                                    good_pos.push(i);
                                }
                            }
                            scores[*good_pos.peek_random_exn(&mut rnd)] += 1;
                        }
                        scores.position_of_max()
                    }
                };
                interactor.make_move(go_pos);

                let expect_seen = near[go_pos].seen;
                let expect_deg = near[go_pos].degree;

                {
                    let mut next_worlds = RandomBag::new(MAX_WORLDS, rnd.gen_u64());
                    let mut near_sorted = near.clone();
                    near_sorted.sort();
                    for world in worlds.iter() {
                        for e in graph.adj(world.cur_v) {
                            let to = e.to();
                            if graph.degree(to) == expect_deg && world.seen.get(to) == expect_seen {
                                let mut new_world = world.clone();
                                new_world.cur_v = to;
                                new_world.seen.set(to, true);
                                new_world.maybe_not_remove_duplicates =
                                    rnd.next_in_range(0, 100000);
                                next_worlds.insert(new_world);
                            }
                        }
                    }
                    worlds = next_worlds;
                }
            }
        }
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let start = input.usize() - 1;
    let _base_move_count = input.usize();
    let graph = read_graph(input, n, m, Directional::Undirected, Indexation::FromOne);

    let mut interactor = RealInteractor::new_real();
    solve_interactor(&graph, &mut interactor, input, start);
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

//START MAIN
mod tester;

fn score(moves: usize, base_move_count: usize, n: usize) -> f64 {
    if moves > 2 * base_move_count {
        return 0.0;
    }
    let moves = moves as f64;
    let base_move_count = base_move_count as f64;
    if moves > base_move_count {
        return 20.0 - (moves + 1.0) * 10.0 / (base_move_count + 1.0);
    }
    let n = n as f64;
    let sol_fraction = (moves + 1.0) / n;
    let base_frac = (base_move_count + 1.0) / n;
    let c = 90.0 / (base_frac - 1.0).sqrt();
    100.0 - c * (sol_fraction - 1.0).sqrt()
}

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_locally();
    // let c = b'k';

    let package = get_current_package_name();

    for test in b'd'..=b'd' {
        let test = byte2str(test);
        dbg!(test);
        let mut file = std::fs::File::open(format!("{}/inputs/{}", package, test)).unwrap();
        let mut input = Input::new(&mut file);
        let tc = input.usize();
        const CNT: usize = 10;

        let mut inputs = vec![];

        for _ in 0..tc {
            // dbg!(inside_test);
            let n = input.usize();
            let m = input.usize();
            let start = input.usize() - 1;
            // assert_eq!(start, 0);
            let base_move_count = input.usize();
            let graph = read_graph(
                &mut input,
                n,
                m,
                Directional::Undirected,
                Indexation::FromOne,
            );
            assert!(!graph.any_self_loops());
            inputs.push((graph, start, base_move_count));
        }

        let mut potential_scores = vec![];
        for _ in 0..CNT {
            let mut sum_scores = 0.0;
            for inside_test in 0..tc {
                let (graph, start, base_move_count) = inputs[inside_test].clone();

                let mut interactor = FakeInteractor::new(graph.clone(), start, base_move_count);
                solve_interactor(&graph, &mut interactor, &mut input, start);
                let moves = interactor.done_moves - 1;
                let score = score(moves, base_move_count, graph.num_vertices());
                sum_scores += score;
                // dbg!(cc, score);
                // dbg!(moves, base_move_count);
            }
            potential_scores.push(sum_scores);
        }
        potential_scores.sort_partial_cmp();
        for score in potential_scores.iter() {
            println!("{}", score);
        }
    }
}
//END MAIN
