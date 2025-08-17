use crate::collections::array_2d::Array2D;

pub struct MatchingResult {
    pub min_cost: i64,
    pub column_per_row: Vec<usize>,
}

///
///
/// Searches for a minimum cost
///
/// Returns None if there is no perfect matching
///
/// Total cost should not exceed std::i64::MAX / 2
///
/// See http://e-maxx.ru/algo/assignment_hungary
///
pub fn hungarian_algorithm(a: &Array2D<i64>) -> Option<MatchingResult> {
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

    let mut row_potential = vec![0; n + 1];
    let mut col_potential = vec![0; m + 1];
    let mut row_per_column = vec![0; m + 1]; // 1-indexed, 0 means None
    let mut prev_column = vec![0; m + 1];
    for i in 1..=n {
        row_per_column[0] = i;
        let mut last_path_col = 0;
        const MAX: i64 = i64::MAX;
        let mut minv = vec![MAX; m + 1];
        let mut used_column = vec![false; m + 1];

        loop {
            used_column[last_path_col] = true;
            let cur_row = row_per_column[last_path_col];
            let mut delta = MAX;
            let mut next_column = 0;
            for j in 1..=m {
                if !used_column[j] {
                    if a[cur_row][j] != i64::MAX {
                        let cur = a[cur_row][j] - row_potential[cur_row] - col_potential[j];
                        if cur < minv[j] {
                            minv[j] = cur;
                            prev_column[j] = last_path_col;
                        }
                    }
                    if minv[j] < delta {
                        delta = minv[j];
                        next_column = j;
                    }
                }
            }
            if next_column == 0 || delta >= MAX / 2 {
                return None;
            }
            for j in 0..=m {
                if used_column[j] {
                    row_potential[row_per_column[j]] += delta;
                    col_potential[j] -= delta;
                } else {
                    minv[j] -= delta;
                }
            }

            last_path_col = next_column;
            if row_per_column[last_path_col] == 0 {
                break;
            }
        }
        if last_path_col == 0 {
            return None;
        }
        assert_eq!(row_per_column[last_path_col], 0);
        loop {
            let j1 = prev_column[last_path_col];
            row_per_column[last_path_col] = row_per_column[j1];
            last_path_col = j1;
            if last_path_col == 0 {
                break;
            }
        }
    }

    let mut column_per_row = vec![0; n];
    for j in 1..=m {
        if row_per_column[j] != 0 {
            column_per_row[row_per_column[j] - 1] = j - 1;
            assert_ne!(a[row_per_column[j]][j], i64::MAX);
        }
    }

    Some(MatchingResult {
        min_cost: -col_potential[0],
        column_per_row,
    })
}
