use crate::collections::bit_set::BitSet;

pub fn bfs_bitsets(root: usize, graph: &[BitSet]) -> Vec<u32> {
    let n = graph.len();
    let mut res = vec![std::u32::MAX; n];
    let mut not_seen = BitSet::new(n);
    for v in 0..n {
        not_seen.set(v, true);
    }
    let mut cur_level = BitSet::new(n);
    cur_level.set(root, true);
    for dist in 0.. {
        let mut v = 0;
        let mut should_stop = true;
        let mut next_level = BitSet::new(n);
        loop {
            if let Some(next) = cur_level.first_set(v) {
                v = next;
                should_stop = false;
            } else {
                break;
            }
            assert!(cur_level.get(v));
            assert_eq!(res[v], std::u32::MAX);
            res[v] = dist;
            assert!(not_seen.get(v));
            not_seen.set(v, false);
            next_level |= &graph[v];
            v += 1;
        }
        next_level &= &not_seen;
        cur_level = next_level;
        if should_stop {
            break;
        }
    }
    res
}
