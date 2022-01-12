//{"name":"F. Еда по воздуху","group":"Yandex - SNWS-2022, Round 1","url":"https://contest.yandex.ru/snws2022/contest/23957/problems/F/","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5 1\n0..2.\n.XXX.\n.....\n.....\n.X.X1\n","output":"9\n"},{"input":"2 2 2\n03\n12\n","output":"4\n"},{"input":"1 5 1\n.0X21\n","output":"8\n"},{"input":"3 4 1\nX0XX\nX.X2\n1XXX\n","output":"-1\n"},{"input":"3 9 5\n...1...2.\n..3..4..5\nXX06XXXXX\n","output":"21\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FYedaPoVozdukhu"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::iters::shifts::SHIFTS_9;
use algo_lib::misc::digits::{char_from_digit, digit_from_char};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};
use std::collections::VecDeque;

type Pos = PointT<i32>;

#[derive(Debug)]
struct Node {
    prev: Pos,
    next: Pos,
    seen: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let m = input.usize();
    let k = input.usize() + 1;
    let field = gen_vec(n, |_| input.string_as_vec());
    let mut positions = vec![Pos::new(-1, -1); k + 1];
    for i in 0..n {
        for j in 0..m {
            let c = field[i][j];
            if c >= b'0' && c <= b'9' {
                positions[digit_from_char(c) as usize] = Pos::new(i as i32, j as i32);
            }
        }
    }
    for p in positions.iter() {
        assert_ne!(p.x, -1);
        assert_ne!(p.y, -1);
    }
    let conv_pos = |p: &Pos| p.x as usize * m + p.y as usize;
    let shift = n * m;
    let shift_seen = k + 2;
    let conv_node = |node: &Node| {
        (conv_pos(&node.prev) * shift + conv_pos(&node.next)) * shift_seen + node.seen as usize
    };
    let get_pos_by_id = |id: usize| {
        assert!(id < shift);
        let y = id % m;
        let x = id / m;
        Pos::new(x as i32, y as i32)
    };
    let get_node_by_id = |mut id: usize| {
        let seen = (id % shift_seen);
        id /= shift_seen;
        let next = get_pos_by_id(id % shift);
        id /= shift;
        let prev = get_pos_by_id(id);
        Node { prev, next, seen }
    };
    let max_sz = shift * shift * shift_seen;
    let start_node = Node {
        prev: positions[0],
        next: positions[0],
        seen: 1,
    };
    let mut queue = VecDeque::with_capacity(max_sz);
    let mut dist = vec![u32::MAX; max_sz];
    let start_id = conv_node(&start_node);
    queue.push_back(start_id);
    dist[start_id] = 0;
    while let Some(v) = queue.pop_front() {
        let node = get_node_by_id(v);
        let old_delta = node.next - node.prev;
        for shift in SHIFTS_9.iter() {
            let new_delta = old_delta.apply_shift(shift);
            let new_next = node.next + new_delta;
            if let Some(&cell) = new_next.index_vec2d(&field) {
                if cell == b'X' {
                    continue;
                }
                let mut new_seen = node.seen;
                if cell == char_from_digit(node.seen as i32) {
                    new_seen += 1;
                }
                let new_node = Node {
                    prev: node.next,
                    next: new_next,
                    seen: new_seen,
                };
                let new_node_id = conv_node(&new_node);
                if dist[new_node_id] == u32::MAX {
                    dist[new_node_id] = dist[v] + 1;
                    queue.push_back(new_node_id);
                }
            }
        }
    }
    let last_node = Node {
        prev: positions[k],
        next: positions[k],
        seen: (k + 1),
    };
    let last_node_id = conv_node(&last_node);
    let res = dist[last_node_id];
    if res == u32::MAX {
        out_line!(-1);
    } else {
        out_line!(res);
    }
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
