//{"name":"D: Alphabet Adventuring","group":"Meta Coding Competitions - Meta Hacker Cup 2022 Final Round","url":"https://www.facebook.com/codingcompetitions/hacker-cup/2022/final-round/problems/D","interactive":false,"timeLimit":360000,"tests":[{"input":"3\n9\n1 2 M\n1 3 E\n1 4 T\n4 9 A\n2 5 T\n2 6 E\n3 7 A\n3 8 M\n6\n2 1 3 META\n2 3 3 TEAM\n2 9 3 MATE\n2 8 8 TEAM\n1 8 T\n2 8 8 TEAM\n5\n1 5 P\n5 2 C\n2 3 U\n4 2 P\n5\n2 1 2 CPU\n2 4 3 CUP\n1 5 U\n2 4 3 CUP\n2 3 4 PUCK\n4\n2 1 A\n2 3 C\n4 2 E\n3\n2 3 2 HACKER\n2 4 2 REACH\n2 1 1 OCEAN\n","output":"Case #1: 6 9 2 9 10\nCase #2: 2 1 6 4\nCase #3: 1 1 2\n"}],"testType":"multiNumber","input":{"type":"regex","fileName":null,"pattern":"alphabet_adventuring_.*input[.]txt"},"output":{"type":"file","fileName":"alphabet_adventuring_output.txt","pattern":null},"languages":{"java":{"taskClass":"DAlphabetAdventuring"}}}

use std::cmp::min;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::binary_search::binary_search_last_true;
use algo_lib::misc::rec_function::{Callable2, Callable3, RecursiveFunction2, RecursiveFunction3};
use algo_lib::seg_trees::lazy_seg_tree::{LazySegTree, LazySegTreeNodeSpec};
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[allow(unused)]
use algo_lib::misc::run_parallel::{run_parallel, ParallelJob};

fn solve(input: &mut Input) {
    // let tc = input.usize();
    // for test_case in 1..=tc {
    //     dbg!(test_case);

    //     out_line!(format!("Case #{}: ", test_case));
    // }
    run_parallel::<Job>(input, Some(8), &());
}

const C: usize = 26;

#[derive(Clone)]
enum Query {
    NewNode(usize, usize),
    Calc(usize, usize, Vec<u8>),
}

#[derive(Clone, Default)]
struct Job {
    n: usize,
    g: Vec<[Option<usize>; C]>,
    queries: Vec<Query>,
    res: Vec<usize>,
}

impl ParallelJob for Job {
    type Context = ();

    fn read_input(&mut self, input: &mut Input) {
        let n = input.usize();
        self.n = n;
        let mut g = vec![[None; C]; n];
        for _ in 0..(n - 1) {
            let fr = input.usize() - 1;
            let to = input.usize() - 1;
            let c = (input.string()[0] - b'A') as usize;
            g[fr][c] = Some(to);
            g[to][c] = Some(fr);
        }
        let q = input.usize();
        for _ in 0..q {
            let q_type = input.usize();
            if q_type == 1 {
                self.queries.push(Query::NewNode(
                    input.usize() - 1,
                    (input.string()[0] - b'A') as usize,
                ));
                g.push([None; C]);
            } else {
                assert_eq!(q_type, 2);
                self.queries.push(Query::Calc(
                    input.usize() - 1,
                    input.usize(),
                    input.string(),
                ));
            }
        }
        self.g = g;
    }

    fn solve(&mut self, context: &Self::Context) {
        let mut g_final = self.g.clone();
        let mut sz = self.n;
        for query in self.queries.iter() {
            if let Query::NewNode(v, c) = query {
                let v = *v;
                let c = *c;
                g_final[v][c] = Some(sz);
                g_final[sz][c] = Some(v);
                sz += 1;
            }
        }
        let mut final_size = vec![1; sz];
        let mut parent = vec![0; sz];
        let mut max_child = vec![0; sz];
        let mut order = vec![];

        let mut parent_c = vec![0; sz];

        RecursiveFunction2::new(|f, v: usize, p: usize| {
            order.push(v);
            parent[v] = p;
            max_child[v] = v;
            for i in 0..C {
                if let Some(to) = g_final[v][i] {
                    if to != p {
                        parent_c[to] = i;
                        f.call(to, v);
                        final_size[v] += final_size[to];
                        if max_child[v] == v || final_size[max_child[v]] < final_size[to] {
                            max_child[v] = to;
                        }
                    }
                }
            }
        })
        .call(0, 0);

        let mut all_paths = vec![];
        let mut path_ids = vec![0; sz];
        RecursiveFunction3::new(|f, v: usize, p: usize, start_new_path: bool| {
            let path_id = if start_new_path {
                all_paths.push(vec![]);
                all_paths.len() - 1
            } else {
                path_ids[p]
            };
            path_ids[v] = path_id;
            all_paths[path_id].push(v);
            for i in 0..C {
                if let Some(to) = g_final[v][i] {
                    if to != p {
                        let start_new_path = max_child[v] != to;
                        f.call(to, v, start_new_path);
                    }
                }
            }
        })
        .call(0, 0, true);

        let mut pos_in_path = vec![0; sz];
        for path in all_paths.iter() {
            for (pos, v) in path.iter().enumerate() {
                pos_in_path[*v] = pos;
            }
        }

        let mut main_child_c = vec![None; sz];
        for path in all_paths.iter() {
            for i in 1..path.len() {
                main_child_c[path[i - 1]] = Some(parent_c[path[i]]);
            }
        }

        let mut go_down: Vec<_> = all_paths
            .iter()
            .map(|path| LazySegTree::new(&Node::default(), path.len(), ()))
            .collect();

        let mut go_up: Vec<_> = all_paths
            .iter()
            .map(|path| LazySegTree::new(&Node::default(), path.len(), ()))
            .collect();

        let make_alive = |v: usize,
                          go_down: &mut Vec<LazySegTree<Node>>,
                          go_up: &mut Vec<LazySegTree<Node>>,
                          cur_g: &[[Option<usize>; C]]| {
            let seg_tree_id = path_ids[v];
            let pos = pos_in_path[v];
            let update = Update {
                require: [0; C],
                alive: true,
            };
            go_down[seg_tree_id].update(pos..pos + 1, update.clone());
            go_up[seg_tree_id].update(pos..pos + 1, update);
            {
                if v != 0 {
                    let p = parent[v];
                    if pos != 0 {
                        let mut require = [0; C];
                        let mut existing_mask = 0;
                        for i in 0..C {
                            if let Some(to) = cur_g[p][i] {
                                if to != parent[p] {
                                    existing_mask |= 1 << i;
                                }
                            }
                        }
                        require[parent_c[v]] = existing_mask;
                        go_down[seg_tree_id].update(
                            pos - 1..pos,
                            Update {
                                require,
                                alive: false,
                            },
                        );
                    } else {
                        if let Some(main_child_c) = main_child_c[p] {
                            let p_seg_tree_id = path_ids[p];
                            let p_pos = pos_in_path[p];
                            let mut require = [0; C];
                            require[main_child_c] = 1 << parent_c[v];
                            go_down[p_seg_tree_id].update(
                                p_pos..p_pos + 1,
                                Update {
                                    require,
                                    alive: false,
                                },
                            );
                        }
                    }
                }
            }
            {
                if v != 0 && pos == 0 {
                    let p = parent[v];
                    let p_seg_tree_id = path_ids[p];
                    let p_pos = pos_in_path[p];
                    let mut require = [0; C];
                    require[parent_c[p]] |= 1 << parent_c[v];
                    go_up[p_seg_tree_id].update(
                        p_pos..p_pos + 1,
                        Update {
                            require,
                            alive: false,
                        },
                    );
                }
            }
        };

        let mut cur_g = self.g.clone();
        for &v in order.iter() {
            if v < self.n {
                make_alive(v, &mut go_down, &mut go_up, &cur_g);
            }
        }
        let mut cur_alive = vec![false; sz];
        for i in 0..self.n {
            cur_alive[i] = true;
        }
        sz = self.n;
        let mut res = vec![];
        for query in self.queries.iter() {
            match query {
                Query::NewNode(prev, c) => {
                    let prev = *prev;
                    let c = *c;
                    cur_g[prev][c] = Some(sz);
                    cur_g[sz][c] = Some(prev);
                    make_alive(sz, &mut go_down, &mut go_up, &cur_g);
                    cur_alive[sz] = true;
                    sz += 1;
                }
                Query::Calc(v, mut len, str) => {
                    let mut v = *v;
                    let mut before = [u32::MAX; C];
                    for i in 0..str.len() {
                        for j in i + 1..str.len() {
                            let c1 = (str[i] - b'A') as usize;
                            let c2 = (str[j] - b'A') as usize;
                            before[c2] ^= 1 << c1;
                        }
                    }
                    let mut prev = v;
                    let mut should_go_up_first = v != 0;
                    for i in 0..C {
                        if let Some(to) = cur_g[v][i] {
                            if to != parent[v] {
                                if (before[parent_c[to]] & (1 << parent_c[v])) != 0 {
                                    should_go_up_first = false;
                                }
                            }
                        }
                    }
                    if should_go_up_first {
                        while v != 0 && len > 0 {
                            let mut ok = true;
                            for i in 0..C {
                                if let Some(to) = cur_g[v][i] {
                                    if to != parent[v] && to != prev {
                                        let potential_child = parent_c[to];
                                        if (before[potential_child] & (1 << parent_c[v])) != 0 {
                                            ok = false;
                                        }
                                    }
                                }
                            }
                            if ok {
                                prev = v;
                                v = parent[v];
                                len -= 1;
                            } else {
                                break;
                            }
                        }
                    }
                    {
                        let mut ok = true;
                        if len > 0 {
                            let mut cur_next = v;
                            for i in 0..C {
                                if let Some(to) = cur_g[v][i] {
                                    if parent[v] != to && to != prev {
                                        if cur_next == v
                                            || (before[parent_c[to]] & (1 << parent_c[cur_next]))
                                                != 0
                                        {
                                            cur_next = to;
                                        }
                                    }
                                }
                            }
                            if cur_next == v {
                                ok = false;
                            } else {
                                len -= 1;
                                prev = v;
                                v = cur_next;
                            }
                        }
                        while ok && len > 0 {
                            let seg_tree_id = path_ids[v];
                            let pos = pos_in_path[v];
                            let cnt_edges = binary_search_last_true(
                                0..min(len, all_paths[seg_tree_id].len() - pos),
                                |go_cnt| {
                                    if go_cnt == 0 {
                                        return true;
                                    }
                                    let node = go_down[seg_tree_id].get(pos..pos + go_cnt);
                                    if !node.all_alive {
                                        return false;
                                    }
                                    for i in 0..C {
                                        if (before[i] & node.require[i]) != node.require[i] {
                                            return false;
                                        }
                                    }
                                    return true;
                                },
                            )
                            .unwrap();
                            let mut cnt_edges = cnt_edges;
                            if !cur_alive[all_paths[seg_tree_id][pos + cnt_edges]] {
                                cnt_edges -= 1;
                            }
                            len -= cnt_edges;
                            v = all_paths[seg_tree_id][pos + cnt_edges];
                            assert!(cur_alive[v]);
                            if len > 0 {
                                let mut cur_next = v;
                                for i in 0..C {
                                    if let Some(to) = cur_g[v][i] {
                                        if parent[v] != to {
                                            if cur_next == v
                                                || (before[parent_c[to]]
                                                    & (1 << parent_c[cur_next]))
                                                    != 0
                                            {
                                                cur_next = to;
                                            }
                                        }
                                    }
                                }
                                if cur_next == v {
                                    break;
                                }
                                len -= 1;
                                v = cur_next;
                            }
                        }
                    }
                    res.push(v);
                }
            }
        }
        self.res = res;
    }

    fn write_output(&mut self, test_case: usize) {
        out!(format!("Case #{}:", test_case));
        for &x in self.res.iter() {
            out!(format!(" {}", x + 1));
        }
        out_line!();
    }
}

#[derive(Clone, Default)]
struct Node {
    require: [u32; C],
    all_alive: bool,
}

#[derive(Clone)]
struct Update {
    require: [u32; C],
    alive: bool,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self, context: &Self::Context) -> Self {
        let mut require = [0; C];
        for i in 0..C {
            require[i] = l.require[i] | r.require[i];
        }
        Self {
            require,
            all_alive: l.all_alive & r.all_alive,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        for i in 0..C {
            node.require[i] |= update.require[i];
        }
        node.all_alive |= update.alive;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        for i in 0..C {
            current.require[i] |= add.require[i];
        }
        current.alive |= add.alive;
    }

    type Update = Update;

    type Context = ();
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
        output: TaskIoType::File("alphabet_adventuring_output.txt".to_string()),
    };

    run_task(io, run)
}

//START MAIN
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    // tester::run_stress(stress);
    // tester::run_locally();
    tester::run_with_last_downloaded_file();
}
//END MAIN
