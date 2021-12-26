//{"name":"G - Strongest Takahashi","group":"AtCoder - AtCoder Beginner Contest 233","url":"https://atcoder.jp/contests/abc233/tasks/abc233_g","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n##...\n.##..\n#.#..\n.....\n....#\n","output":"4\n"},{"input":"3\n...\n...\n...\n","output":"0\n"},{"input":"21\n.....................\n.....................\n...#.#...............\n....#.............#..\n...#.#...........#.#.\n..................#..\n.....................\n.....................\n.....................\n..........#.....#....\n......#..###.........\n........#####..#.....\n.......#######.......\n.....#..#####........\n.......#######.......\n......#########......\n.......#######..#....\n......#########......\n..#..###########.....\n.........###.........\n.........###.........\n","output":"19\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GStrongestTakahashi"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::UpdateMinMax;
use algo_lib::{dbg, out, out_line};
use std::cmp::max;
use std::ops::Range;

struct State {
    n: usize,
    cache: Array2D<Option<usize>>,
    field: Vec<Vec<u8>>,
}

impl State {
    fn new(n: usize, field: Vec<Vec<u8>>) -> Self {
        let one_dim_size = n * n;
        let cache = Array2D::new(None, one_dim_size, one_dim_size);
        Self { n, cache, field }
    }

    fn id(&self, r: &Range<usize>) -> usize {
        r.start + (r.end - 1) * self.n
    }

    fn calc(&mut self, x: &Range<usize>, y: &Range<usize>) -> usize {
        if x.len() == 0 || y.len() == 0 {
            return 0;
        }
        if x.len() == 1 && y.len() == 1 {
            let val = self.field[x.start][y.start];
            if val == b'#' {
                return 1;
            }
            assert_eq!(val, b'.');
            return 0;
        }
        let x_id = self.id(x);
        let y_id = self.id(y);
        if let Some(ans) = self.cache[x_id][y_id] {
            return ans;
        }
        let mut ans = max(x.len(), y.len());
        if x.len() > y.len() {
            // can split by x
            for x_split in x.start + 1..x.end {
                ans.update_min(self.calc(&(x.start..x_split), y) + self.calc(&(x_split..x.end), y));
            }
        } else {
            for y_split in y.start + 1..y.end {
                ans.update_min(self.calc(x, &(y.start..y_split)) + self.calc(x, &(y_split..y.end)));
            }
        }
        self.cache[x_id][y_id] = Some(ans);
        ans
    }
}

fn solve(input: &mut Input) {
    let n = input.usize();
    let field: Vec<Vec<_>> = (0..n).map(|_| input.string_as_vec()).collect();
    let mut state = State::new(n, field);
    out_line!(state.calc(&(0..n), &(0..n)));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
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
