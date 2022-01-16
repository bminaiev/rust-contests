#[cfg(test)]
mod tests {
    use crate::collections::sparse_table_max::SparseTableMax;
    use crate::misc::rand::Random;

    #[test]
    fn simple() {
        let mut rnd = Random::new(787788);
        for n in 1..50 {
            let a = rnd.gen_vec(n, 0..20i32);
            let table = SparseTableMax::new(&a);
            for from in 0..n {
                for to in from + 1..=n {
                    let res_from_table = a[table.find_max_pos(from..to)];
                    let slow_res = *a[from..to].iter().max().unwrap();
                    assert_eq!(res_from_table, slow_res);
                }
            }
        }
    }
}
