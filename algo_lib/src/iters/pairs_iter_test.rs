#[cfg(test)]
mod tests {
    use crate::iters::pairs_iter::PairsIterTrait;

    #[test]
    fn simple() {
        let a = [1, 3, 5];
        let pairs: Vec<(i32, i32)> = a.iter().pairs().map(|(a, b)| (*a, *b)).collect();
        assert_eq!(pairs, vec![(1, 3), (1, 5), (3, 5)]);
    }

    #[test]
    fn simple2() {
        let pairs: Vec<(i32, i32)> = (0..4).pairs().collect();
        assert_eq!(pairs, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
    }

    #[test]
    fn empty() {
        let pairs: Vec<(i32, i32)> = (0..1).pairs().collect();
        assert_eq!(pairs, vec![]);
    }

    #[test]
    fn empty2() {
        let pairs: Vec<(i32, i32)> = (0..0).pairs().collect();
        assert_eq!(pairs, vec![]);
    }
}
