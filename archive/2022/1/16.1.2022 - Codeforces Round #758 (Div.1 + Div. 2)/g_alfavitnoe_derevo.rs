//{"name":"G. Алфавитное дерево","group":"Codeforces - Codeforces Round #758 (Div.1 + Div. 2)","url":"https://codeforces.com/contest/1608/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"2 5 3\n1 2 a\naab\nabab\naaa\nb\na\n2 1 1 5\n1 2 1 3\n2 1 3 5\n","output":"8\n7\n4\n"},{"input":"9 5 6\n1 2 a\n2 7 c\n1 3 b\n3 4 b\n4 6 b\n3 5 a\n5 8 b\n5 9 c\nababa\ncabbb\nbac\nbbbac\nabacaba\n2 7 1 4\n2 5 1 5\n6 3 4 4\n6 9 4 5\n5 7 3 5\n5 3 1 5\n","output":"3\n4\n2\n1\n1\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GAlfavitnoeDerevo"}}}

use algo_lib::graph::edges::edge_with_info::EdgeWithInfo;
use algo_lib::graph::simple_graph::SimpleGraphT;
use algo_lib::graph::trees::calc_edge_to_parent::calc_edge_to_parent;
use algo_lib::graph::trees::heavy_light::{GoDirection, HeavyLight};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::binary_search::binary_search_first_true;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::seg_trees::fenwick::Fenwick;
use algo_lib::strings::suffix_array::SuffixArray;
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};
use std::cmp::{max, min};
use std::ops::Range;
use std::process::exit;
use std::time::Instant;

type Edge = EdgeWithInfo<u8>;

struct Query {
    id: usize,
    fr_v: usize,
    to_v: usize,
    words_range: Range<usize>,
}

const SEPARATOR: u8 = b'$';

#[derive(Debug)]
struct Node {
    pos_in_long_string: Vec<usize>,
    rev_pos_in_long_string: Vec<usize>,
}

fn find_range_in_suf_array(
    suf_array: &SuffixArray,
    query: &Query,
    heavy_light: &HeavyLight<Node>,
) -> Range<usize> {
    let mut already_len = 0;
    let mut res = 0..suf_array.len();
    heavy_light.go_path(query.fr_v, query.to_v, |sub_path, range, dir| {
        let pos_in_long_str = match dir {
            GoDirection::RightToLeft => {
                // TODO: indexing?
                sub_path.extra.rev_pos_in_long_string[range.end - 1]
            }
            GoDirection::LeftToRight => sub_path.extra.pos_in_long_string[range.start],
        };
        let len = range.len();
        // TODO: naming???
        let search_near_pos = suf_array.get_pos_in_array(pos_in_long_str);
        let next_start = binary_search_first_true(res.clone(), |whole_str_check_pos| {
            let pos_in_string = suf_array[whole_str_check_pos];
            let pos_to_compare = (pos_in_string + already_len) % suf_array.len();
            let in_suf_array = suf_array.get_pos_in_array(pos_to_compare);
            in_suf_array >= search_near_pos || suf_array.lcp(in_suf_array, search_near_pos) >= len
        });
        let next_end = binary_search_first_true(next_start..res.end, |pos_in_suf_array| {
            let pos_in_string = suf_array[pos_in_suf_array];
            let pos_to_compare = (pos_in_string + already_len) % suf_array.len();
            let in_suf_array = suf_array.get_pos_in_array(pos_to_compare);
            suf_array.lcp(in_suf_array, search_near_pos) < len
        });
        res = next_start..next_end;
        already_len += len;
    });
    res
}

fn solve_input(
    vertices: usize,
    edges: &[(usize, usize, u8)],
    words: &[Vec<u8>],
    queries: &[Query],
) -> Vec<i64> {
    let mut graph = SimpleGraphT::<Edge>::new(vertices);
    for &(fr, to, c) in edges {
        graph.add_edge(fr, Edge::new(to, c));
        graph.add_edge(to, Edge::new(fr, c));
    }

    let mut long_string = vec![];
    let mut word_pos_in_string = vec![0; words.len()];
    for (w_id, word) in words.iter().enumerate() {
        word_pos_in_string[w_id] = long_string.len();
        long_string.append(&mut word.clone());
        long_string.push(SEPARATOR);
    }

    let edge_to_parent = calc_edge_to_parent(&graph, Edge::new(0, SEPARATOR));
    let heavy_light = HeavyLight::new(&graph, 0, |vertices| {
        let mut pos_in_long_string = vec![];
        for &v in vertices.iter().skip(1) {
            pos_in_long_string.push(long_string.len());
            long_string.push(edge_to_parent[v].info);
        }
        let mut rev_pos_in_long_string = vec![];
        for &v in vertices.iter().skip(1).rev() {
            rev_pos_in_long_string.push(long_string.len());
            long_string.push(edge_to_parent[v].info);
        }
        rev_pos_in_long_string.reverse();
        Node {
            pos_in_long_string,
            rev_pos_in_long_string,
        }
    });
    let suf_array = SuffixArray::new(long_string.clone());
    let query_ranges: Vec<_> = queries
        .iter()
        .map(|query| find_range_in_suf_array(&suf_array, query, &heavy_light))
        .collect();
    let mut res = vec![0i64; queries.len()];
    let mut change_queries = vec![vec![]; words.len() + 1];
    for query in queries.iter() {
        change_queries[query.words_range.start].push((-1, query.id));
        change_queries[query.words_range.end].push((1, query.id));
    }
    let mut fenw = Fenwick::new(long_string.len() + 1);
    for id in 0..=words.len() {
        for &(delta, query) in change_queries[id].iter() {
            res[query] += delta * fenw.get_range_sum(query_ranges[query].clone());
        }
        if id == words.len() {
            break;
        }
        let start = word_pos_in_string[id];
        for offset in 0..words[id].len() {
            fenw.add(suf_array.get_pos_in_array(start + offset), 1);
        }
    }
    res
}

fn stress() -> bool {
    let mut rnd = Random::new(787788);
    let n = 100_000;
    let alph = b'a'..b'c';
    let edges = gen_vec(n - 1, |id| {
        let p = rnd.gen_range(0..id + 1);
        (p, id, rnd.gen_range(alph.clone()))
    });
    let cnt_words = 5;
    let sum_len = 100_000;
    let words = gen_vec(cnt_words, |_| {
        let len = rnd.gen_range(sum_len / cnt_words..sum_len * 2 / cnt_words);
        rnd.gen_vec(len, alph.clone())
    });
    dbg!(words.iter().map(|s| s.len()).sum::<usize>());
    let cnt_queries = 100_000;
    let queries = gen_vec(cnt_queries, |id| {
        let x = rnd.gen_range(0..words.len());
        let y = rnd.gen_range(0..words.len());
        Query {
            id,
            fr_v: rnd.gen_range(0..n),
            to_v: rnd.gen_range(0..n),
            words_range: min(x, y)..max(x, y) + 1,
        }
    });
    solve_input(n, &edges, &words, &queries);


    true
}

fn solve(input: &mut Input, _test_case: usize) {
    let vertices = input.usize();
    let num_words = input.usize();
    let q = input.usize();
    let edges = gen_vec(vertices - 1, |_| {
        let fr = input.usize() - 1;
        let to = input.usize() - 1;
        let c: u8 = input.string_as_vec()[0];
        (fr, to, c)
    });
    let words = gen_vec(num_words, |_| input.string_as_vec());
    let queries = gen_vec(q, |id| Query {
        id,
        fr_v: input.usize() - 1,
        to_v: input.usize() - 1,
        words_range: input.usize() - 1..input.usize(),
    });
    let res = solve_input(vertices, &edges, &words, &queries);
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
