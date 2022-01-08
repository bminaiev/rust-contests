//{"name":"E. Лексикографически достаточно малая","group":"Codeforces - Good Bye 2021: 2022 is NEAR","url":"http://codeforces.com/contest/1616/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\na\na\n3\nrll\nrrr\n3\ncaa\naca\n5\nababa\naabba\n","output":"-1\n0\n2\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ELeksikograficheskiDostatochnoMalaya"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::seg_trees::fenwick::Fenwick;
use algo_lib::{dbg, out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let s = input.string_as_vec();
    let t = input.string_as_vec();
    const M: usize = 26;
    let mut pos_by_char = vec![vec![]; M];
    for (pos, val) in s.iter().enumerate() {
        pos_by_char[(*val - b'a') as usize].push(pos);
    }
    let ni64 = n as i64;
    let max_ops = ni64 * ni64 + 10;
    let required_ops = binary_search_first_true(0..max_ops, |ops| -> bool {
        let mut used = Fenwick::new(n);
        let mut check = vec![];
        let mut more_ops = ops;
        let mut iter = vec![0; M];
        while check.len() != s.len() {
            for c in 0..M {
                if iter[c] == pos_by_char[c].len() {
                    continue;
                }
                let pos = pos_by_char[c][iter[c]];
                {
                    let real_pos = pos - (used.get_sum(pos) as usize);
                    if real_pos as i64 <= more_ops {
                        more_ops -= real_pos as i64;
                        used.add(pos, 1);
                        check.push(s[pos]);
                        iter[c] += 1;
                        break;
                    }
                }
            }
        }
        assert_eq!(check.len(), s.len());
        check < t
    });
    if required_ops == max_ops {
        out_line!(-1);
    } else {
        out_line!(required_ops);
    }
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

fn main() {
    tester::run_tests();
}
//END MAIN
