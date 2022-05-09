use crate::collections::bit_set_fixed_size::BitSetFixedSize;

pub fn bfs_bitsets_fixed_size(root: usize, graph: &[BitSetFixedSize]) -> BitSetFixedSize {
    let n = graph.len();
    let mut not_seen = BitSetFixedSize::new(n);
    for v in 0..n {
        not_seen.set(v, true);
    }
    let mut check_next = BitSetFixedSize::new(n);
    check_next.set(root, true);
    while let Some(mut v) = check_next.first_set(0) {
        while v < graph.len() {
            not_seen.set(v, false);
            check_next |= &graph[v];
            check_next &= &not_seen;
            v = check_next.first_set(v + 1).unwrap_or(graph.len());
        }
    }
    let mut seen = BitSetFixedSize::new(n);
    for v in 0..n {
        if !not_seen.get(v) {
            seen.set(v, true);
        }
    }
    seen
}
