#[cfg(test)]
mod tests {
    use crate::seg_trees::lazy_seg_tree_max::{MaxValNode, SegTreeMax};

    #[test]
    fn simple() {
        let n = 5;
        let mut seg_tree = SegTreeMax::new(n, &|pos| MaxValNode { max_val: 0, pos });
        seg_tree.update(2..3, 123);
        let res = seg_tree.get(0..5);
        assert_eq!(res.max_val, 123);
        assert_eq!(res.pos, 2);
    }
}
