//{"name":"D. Dividing the Kingdom","group":"Codeforces - 2021-2022 ACM-ICPC Brazil Subregional Programming Contest","url":"https://codeforces.com/gym/103388/problem/D","interactive":false,"timeLimit":1500,"tests":[{"input":"9 7\n1 2 3\n2 3 3\n3 4 3\n1 3 2\n2 4 2\n6 7 1\n8 9 1\n","output":"2\n3\n"},{"input":"4 4\n1 2 5\n2 3 6\n1 3 7\n3 4 7\n","output":"IMPOSSIBLE\n"},{"input":"2 1\n1 2 10\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDividingTheKingdom"}}}

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::iters::pairs_iter::PairsIterTrait;
use algo_lib::misc::func::id;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::ordered_pair::OrderedPair;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

struct Edge {
    fr: usize,
    to: usize,
    cost: i32,
}

struct Dsu {
    parent: Vec<usize>,
    xor: Vec<usize>,
}

impl Dsu {
    pub fn new(n: usize) -> Self {
        let parent = gen_vec(n, id);
        let xor = vec![0; n];
        Self { parent, xor }
    }

    pub fn get_root(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            let new_parent = self.get_root(self.parent[v]);
            self.xor[v] ^= self.xor[self.parent[v]];
            self.parent[v] = new_parent;
        }
        self.parent[v]
    }

    pub fn get_root_and_color(&mut self, v: usize) -> (usize, usize) {
        let root = self.get_root(v);
        (root, self.xor[v])
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let x_root = self.get_root(x);
        let y_root = self.get_root(y);
        let need_xor = self.xor[x] ^ self.xor[y] ^ 1;
        if x_root == y_root {
            return need_xor == 0;
        }
        self.xor[x_root] = need_xor;
        self.parent[x_root] = y_root;
        return true;
    }
}

fn good_pair(p1: &OrderedPair<usize>, p2: &OrderedPair<usize>) -> bool {
    if p1.min != p2.min && p1.min != p2.max {
        if p1.max != p2.min && p1.max != p2.max {
            return true;
        }
    }
    return false;
}

fn exist_good_pair_slow(pairs1: &[OrderedPair<usize>], pairs2: &[OrderedPair<usize>]) -> bool {
    // TODO: optimize
    for p1 in pairs1.iter() {
        for p2 in pairs2.iter() {
            if p1.min != p2.min && p1.min != p2.max {
                if p1.max != p2.min && p1.max != p2.max {
                    return true;
                }
            }
        }
    }
    false
}

fn exist_good_pair_old(pairs: &[OrderedPair<usize>]) -> bool {
    {
        let mut same_max = true;
        for i in 1..pairs.len() {
            if pairs[i].max != pairs[0].max {
                same_max = false;
            }
        }
        if same_max {
            return false;
        }
    }
    let mut intervals = vec![];
    {
        let mut it = 0;
        while it != pairs.len() {
            let mut nit = it + 1;
            while nit != pairs.len() && pairs[it].min == pairs[nit].min {
                nit += 1;
            }
            intervals.push(it..nit);
            it = nit;
        }
    }
    for (i1, i2) in intervals.iter().pairs() {
        if exist_good_pair_slow(&pairs[i1.clone()], &pairs[i2.clone()]) {
            return true;
        }
    }
    false
}

fn exist_good_pair(pairs: &[OrderedPair<usize>]) -> bool {
    {
        let mut same_max = true;
        for i in 1..pairs.len() {
            if pairs[i].max != pairs[0].max {
                same_max = false;
            }
        }
        if same_max {
            return false;
        }
    }
    {
        let mut same_min = true;
        for i in 1..pairs.len() {
            if pairs[i].min != pairs[0].min {
                same_min = false;
            }
        }
        if same_min {
            return false;
        }
    }
    let mut rnd = Random::new(78987);
    const BUBEN: usize = 20;
    for i in 0..pairs.len() {
        for _ in 0..BUBEN {
            let j = rnd.gen_in_range(0..pairs.len());
            if j != i && good_pair(&pairs[i], &pairs[j]) {
                return true;
            }
        }
    }
    false
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    assert!(m > 0);
    let mut edges = gen_vec(m, |_| {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let cost = input.read();
        assert!(cost > 0);
        Edge { fr, to, cost }
    });
    edges.sort_by_key(|e| -e.cost);
    let mut dsu = Dsu::new(n);
    let mut res = vec![];
    let mut iter = 0;

    let mut two_colored = true;
    while iter != edges.len() {
        let mut next_it = iter;
        while next_it != edges.len() && edges[next_it].cost == edges[iter].cost {
            next_it += 1;
        }
        {
            let mut ok_edges = vec![];
            for e in edges[iter..next_it].iter() {
                let (r1, c1) = dsu.get_root_and_color(e.fr);
                let (r2, c2) = dsu.get_root_and_color(e.to);
                if r1 == r2 && c1 != c2 {
                    continue;
                }

                ok_edges.push(OrderedPair::new(r1 * 2 + c1, r2 * 2 + c2));
            }
            ok_edges.sort();
            ok_edges.dedup();
            if exist_good_pair(&ok_edges) {
                res.push(edges[iter].cost);
            }
        }

        for e in edges[iter..next_it].iter() {
            if !dsu.unite(e.fr, e.to) {
                two_colored = false;
                break;
            }
        }
        if !two_colored {
            break;
        }
        iter = next_it;
    }
    if two_colored {
        res.push(0);
    }
    res.reverse();
    if res.is_empty() {
        out_line!("IMPOSSIBLE");
        return;
    }
    for &x in res.iter() {
        out_line!(x);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    // input.skip_whitespace();
    // input.peek().is_none()
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
}
//END MAIN
