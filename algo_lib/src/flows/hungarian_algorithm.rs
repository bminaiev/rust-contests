use crate::collections::array_2d::Array2D;

///
///
/// Searches for a minimum cost
///
/// See http://e-maxx.ru/algo/assignment_hungary
///
pub fn hungarian_algorithm(a: &Array2D<i64>) -> i64 {
    let n = a.len();
    let m = a[0].len();

    let a = {
        let mut a_shifted = Array2D::new(0, n + 1, m + 1);
        for i in 0..n {
            for j in 0..m {
                a_shifted[i + 1][j + 1] = a[i][j];
            }
        }
        a_shifted
    };

    let mut u = vec![0; n + 1];
    let mut v = vec![0; m + 1];
    let mut p = vec![0; m + 1];
    let mut way = vec![0; m + 1];
    for i in 1..=n {
        p[0] = i;
        let mut j0 = 0;
        let mut minv = vec![i64::MAX; m + 1];
        let mut used = vec![false; m + 1];
        loop {
            used[j0] = true;
            let i0 = p[j0];
            let mut delta = i64::MAX;
            let mut j1 = 0;
            for j in 1..=m {
                if !used[j] {
                    let cur = a[i0][j] - u[i0] - v[j];
                    if cur < minv[j] {
                        minv[j] = cur;
                        way[j] = j0;
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        j1 = j;
                    }
                }
            }
            for j in 0..=m {
                if used[j] {
                    u[p[j]] += delta;
                    v[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            j0 = j1;
            if p[j0] == 0 {
                break;
            }
        }
        loop {
            let j1 = way[j0];
            p[j0] = p[j1];
            j0 = j1;
            if j0 == 0 {
                break;
            }
        }
    }

    -v[0]
}
