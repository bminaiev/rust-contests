//{"name":"E. Expedition","group":"Yandex - SNWS-2022, Round 3","url":"https://contest.yandex.ru/snws2022/contest/23959/problems/E/","interactive":false,"timeLimit":2000,"tests":[{"input":"4 4 1 1\n1 2\n2 3\n1 3\n1 4\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EExpedition"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::graph::bfs::bfs;
use algo_lib::graph::edges::simple_edge::SimpleEdge;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::{dbg, out, out_line};

type Mod = Mod_998_244_353;

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let size_1 = input.usize();
    let size_2 = input.usize();
    let mut g_matrix = Array2D::new(true, n, n);
    for i in 0..n {
        g_matrix[i][i] = false;
    }
    for _ in 0..m {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        g_matrix[fr][to] = false;
        g_matrix[to][fr] = false;
    }
    let mut g = SimpleGraphT::new(n);
    for i in 0..n {
        for j in 0..n {
            if g_matrix[i][j] {
                g.add_edge(i, SimpleEdge::new(j));
            }
        }
    }
    let mut seen = vec![false; n];
    let mut cur_xor = vec![0; n];
    let mut pairs = vec![];
    for root in 0..n {
        if seen[root] {
            continue;
        }
        let bfs = bfs(root, &g);
        let mut sizes = vec![0; 2];
        for v in 0..n {
            let d = bfs.dist[v];
            if d == u32::MAX {
                continue;
            }
            cur_xor[v] = d % 2;
            sizes[(d % 2) as usize] += 1;
            seen[v] = true;
        }
        pairs.push(sizes);
    }
    for x in 0..n {
        for y in 0..n {
            if g_matrix[x][y] && (cur_xor[x] ^ cur_xor[y] == 0) {
                out_line!(0);
                return;
            }
        }
    }
    let mut dp = Array2D::new(Mod::ZERO, n + 1, n + 1);
    dp[0][0] = Mod::ONE;
    for pair in pairs.iter() {
        let mut ndp = Array2D::new(Mod::ZERO, n + 1, n + 1);
        for x in 0..=n {
            for y in 0..=n {
                let last = dp[x][y];
                if last == Mod::ZERO {
                    continue;
                }
                ndp[x + pair[0]][y + pair[1]] += last;
                ndp[x + pair[1]][y + pair[0]] += last;
                if pair[1] == 0 {
                    ndp[x + pair[0]][y + pair[0]] += last;
                }
            }
        }
        dp = ndp;
    }
    let mut res = Mod::ZERO;
    for x in size_1..=n {
        for y in size_2..=n {
            res += dp[x][y];
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
