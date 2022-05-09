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
            debug_assert!(cur_level.get(v));
            debug_assert_eq!(res[v], std::u32::MAX);
            res[v] = dist;
            debug_assert!(not_seen.get(v));
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

pub fn bfs_bitsets_vis(root: usize, graph: &[BitSet]) -> BitSet {
    let n = graph.len();
    let mut not_seen = BitSet::new(n);
    for v in 0..n {
        not_seen.set(v, true);
    }
    let mut check_next = BitSet::new(n);
    check_next.set(root, true);
    while let Some(mut v) = check_next.first_set(0) {
        while v < graph.len() {
            not_seen.set(v, false);
            check_next |= &graph[v];
            check_next &= &not_seen;
            v = check_next.first_set(v + 1).unwrap_or(graph.len());
        }
    }
    let mut seen = BitSet::new(n);
    for v in 0..n {
        if !not_seen.get(v) {
            seen.set(v, true);
        }
    }
    seen
}
