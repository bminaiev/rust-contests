//{"name":"B. Bard","group":"Yandex - SNWS-2022, Round 2","url":"https://contest.yandex.ru/snws2022/contest/23958/problems/B/","interactive":false,"timeLimit":4000,"tests":[{"input":"6 5 1\nn a\na m\nm e\nm ee\nx n\nthe names\nname\n1 9 1\nnames name\nname\n2 6 2\nxa aaxa\naxa abx\naa\nab\n0 0 0\n","output":"3\n0\n0\n"}],"testType":"multiEof","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBard"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::id_map::IdMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::Mod_998_244_353;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::strings::aho_corasick::AhoCorasick;
use algo_lib::strings::trie::{NodeId, Trie};
use algo_lib::strings::utils::vec2str;
use algo_lib::{dbg, out, out_line};
use std::collections::HashMap;

type Mod = Mod_998_244_353;
type String = Vec<u8>;

#[derive(Default, Copy, Clone, Debug)]
struct Edge {
    seen: usize,
    next: NodeId,
}

fn solve(input: &mut Input, _test_case: usize) {
    let num_pairs = input.usize();
    let n = input.usize();
    let people = input.usize();
    if n == 0 {
        return;
    }
    let mut words = IdMap::<String>::new();

    let pairs: Vec<(Vec<u8>, Vec<u8>)> = gen_vec(num_pairs, |_| {
        (input.string_as_vec(), input.string_as_vec())
    });
    let peoples = gen_vec(people, |_| input.string_as_vec());
    let mut aho_corasick: AhoCorasick<usize> = AhoCorasick::new(26, &peoples);
    for person in peoples.iter() {
        let node = aho_corasick.go(NodeId::ROOT, person);
        aho_corasick[node] += 1;
    }
    for node_id in aho_corasick.all_node_ids() {
        let from_suf_link = aho_corasick[aho_corasick.get_suf_link(node_id)];
        aho_corasick[node_id] += from_suf_link;
    }

    let go_word = |mut cur_node: NodeId, word: &Vec<u8>| -> Edge {
        let mut seen = 0;
        for c in word.iter() {
            cur_node = aho_corasick.go(cur_node, &[*c]);
            seen += aho_corasick[cur_node];
        }
        Edge {
            seen,
            next: cur_node,
        }
    };

    let pairs_ids: Vec<_> = pairs
        .iter()
        .map(|(w1, w2)| (words.get_or_add(w1), words.get_or_add(w2)))
        .collect();

    let mut res = Mod::ZERO;

    let n_nodes = aho_corasick.len();
    let n_words = words.len();

    let node_ids_by_word: Vec<Vec<_>> = words
        .iter()
        .map(|(_, word)| {
            (0..n_nodes)
                .filter(|&id| {
                    let node_word = aho_corasick.get_full_word(NodeId::from_raw(id));
                    node_word.ends_with(word) || word.ends_with(&node_word)
                })
                .map(NodeId::from_raw)
                .collect()
        })
        .collect();

    // [len][last_word_id][local_node][seen?]
    let mut dp = gen_vec(n + 1, |_| {
        gen_vec(n_words, |word_id| {
            Array2D::new(Mod::ZERO, node_ids_by_word[word_id].len(), 2)
        })
    });
    for (id, word) in words.iter() {
        let e = go_word(NodeId::ROOT, word);
        if e.seen <= 1 && word.len() <= n {
            let local_next_node = node_ids_by_word[id].binary_search(&e.next).unwrap();
            dp[word.len()][id][local_next_node][e.seen] += Mod::ONE;
        }
    }

    let mut go_word_cache = Array2D::new(Edge::default(), n_nodes, n_words);
    for node in aho_corasick.all_node_ids() {
        for word in 0..n_words {
            go_word_cache[node.id()][word] = go_word(node, &words[word]);
        }
    }


    // n = 500
    // n_nodes = 20 * 30 = 600
    // pairs = 250
    for len in 1..n {
        for last_seen in 0..=1 {
            for &(last_word, second) in pairs_ids.iter() {
                for (local_last_node, &global_last_node) in
                node_ids_by_word[last_word].iter().enumerate()
                {
                    let last = dp[len][last_word][local_last_node][last_seen];

                    let edge = go_word_cache[global_last_node.id()][second];
                    let next_seen = last_seen + edge.seen;
                    let next_len = len + words[second].len();
                    if next_seen <= 1 && next_len <= n {
                        let local_next_node =
                            node_ids_by_word[second].binary_search(&edge.next).unwrap();
                        dp[next_len][second][local_next_node][next_seen] += last;
                    }
                }
            }
        }
    }
    for last_word in 0..n_words {
        for node in 0..node_ids_by_word[last_word].len() {
            res += dp[n][last_word][node][1];
        }
    }
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let mut i = 1usize;
    while input.peek().is_some() {
        solve(&mut input, i);
        i += 1;
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
