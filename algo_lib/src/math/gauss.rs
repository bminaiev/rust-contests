use crate::collections::array_2d::Array2D;
use crate::misc::num_traits::Number;

/**

Modifies array [a], and returns number of linear independent rows

*/
pub fn gauss<M>(a: &mut Array2D<M>) -> usize
where
    M: Number,
{
    let n = a.rows();
    let m = a[0].len();
    let mut row_iter = 0;
    for column in 0..m {
        if let Some(use_row) = (row_iter..n).find(|&row| a[row][column] != M::ZERO) {
            a.swap(row_iter, use_row);
            let inv = M::ONE / a[row_iter][column];
            for val in a[row_iter].iter_mut() {
                *val *= inv;
            }
            // TODO: not great for floats..
            assert_eq!(a[row_iter][column], M::ONE);
            for row in row_iter + 1..n {
                if a[row][column] != M::ZERO {
                    let mult = a[row][column];
                    for col in 0..m {
                        let sub = a[row_iter][col] * mult;
                        a[row][col] -= sub;
                    }
                    // TODO: floats?
                    assert_eq!(a[row][column], M::ZERO);
                }
            }
            row_iter += 1;
        }
    }
    row_iter
}
