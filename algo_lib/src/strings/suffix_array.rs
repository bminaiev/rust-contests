use crate::collections::rev_permutation::rev_permutation;
use crate::collections::sparse_table_min::SparseTableMin;
use crate::misc::gen_vector::gen_vec;
use crate::misc::num_traits::Number;
use crate::strings::utils::vec2str;
use std::cell::{Ref, RefCell};
use std::cmp::{max, min};
use std::ops::Index;

#[derive(Debug)]
pub struct SuffixArray {
    sorted_suffixes: Vec<usize>,
    pos_in_sorted: Vec<usize>,
    lcp: Vec<u32>,
    lcp_sparse_table: RefCell<Option<SparseTableMin<u32>>>,
}

impl SuffixArray {
    pub fn get_pos_in_array(&self, pos_in_string: usize) -> usize {
        self.pos_in_sorted[pos_in_string]
    }

    pub fn len(&self) -> usize {
        self.sorted_suffixes.len()
    }

    fn lcp_sparse_table(&self) -> Ref<SparseTableMin<u32>> {
        self.lcp_sparse_table
            .borrow_mut()
            .get_or_insert_with(|| SparseTableMin::new(&self.lcp));
        Ref::map(self.lcp_sparse_table.borrow(), |m| m.as_ref().unwrap())
    }

    pub fn lcp(&self, p1: usize, p2: usize) -> usize {
        let min_pos = min(p1, p2);
        let max_pos = max(p1, p2);
        if min_pos == max_pos {
            return self.len();
        }
        let lcp_table = self.lcp_sparse_table();
        let pos = lcp_table.find_min_pos(min_pos..max_pos);
        self.lcp[pos] as usize
    }

    fn build_lcp<T>(str: &[T], sorted_suffixes: &[usize], pos_in_sorted: &[usize]) -> Vec<u32>
    where
        T: Number,
    {
        let n = str.len();
        let mut lcp = vec![0; n - 1];
        let mut k = 0;
        for i in 0..n {
            if k > 0 {
                k -= 1;
            }
            if pos_in_sorted[i] == n - 1 {
                k = 0;
                continue;
            }
            let j = sorted_suffixes[pos_in_sorted[i] + 1];
            while max(i + k, j + k) < n && str[i + k] == str[j + k] {
                k += 1;
            }
            lcp[pos_in_sorted[i]] = k as u32
        }
        lcp
    }

    pub fn debug_print_suf_array(mut str: Vec<u8>) {
        str.push(0);
        let n = str.len();
        let mut ids = gen_vec(n, |x| x);
        ids.sort_by_key(|&pos| &str[pos..]);
        for (pos, &id) in ids.iter().enumerate() {
            eprintln!("{} -> {}", pos, vec2str(&str[id..]));
        }
    }

    pub fn new<T>(mut str: Vec<T>) -> Self
    where
        T: Number,
    {
        str.push(T::ZERO);
        let n = str.len();
        let mut sorted_suffixes = gen_vec(str.len(), |x| x);
        // TODO: replace with counting sort?
        sorted_suffixes.sort_by_key(|&id| str[id as usize]);
        let mut class_eq = vec![0; n];
        for win in sorted_suffixes.windows(2) {
            if str[win[1] as usize] != str[win[0] as usize] {
                class_eq[win[1] as usize] = class_eq[win[0] as usize] + 1;
            } else {
                class_eq[win[1] as usize] = class_eq[win[0] as usize];
            }
        }
        let mut num_classes = class_eq.iter().max().unwrap() + 1;
        let mut suffixes_new = vec![0; n];
        let mut class_eq_new = vec![0; n];
        let mut cnt = vec![0; n];
        for lvl in 0.. {
            let half = 1 << lvl;
            if half >= n {
                break;
            }
            for (val_new, val) in suffixes_new.iter_mut().zip(sorted_suffixes.iter()) {
                let next = (*val as i32) - (half as i32);
                let next = if next < 0 { next + n as i32 } else { next };
                *val_new = next as usize;
            }
            for i in 0..num_classes {
                cnt[i] = 0;
            }
            for &class_id in class_eq.iter() {
                cnt[class_id] += 1;
            }
            for i in 1..num_classes {
                cnt[i] += cnt[i - 1];
            }
            for i in (0..n).rev() {
                cnt[class_eq[suffixes_new[i]]] -= 1;
                sorted_suffixes[cnt[class_eq[suffixes_new[i]]]] = suffixes_new[i];
            }
            class_eq_new[sorted_suffixes[0]] = 0;
            num_classes = 1;
            for i in 1..n {
                let mid1 = (sorted_suffixes[i] + half) % n;
                let mid2 = (sorted_suffixes[i - 1] + half) % n;
                if class_eq[sorted_suffixes[i]] != class_eq[sorted_suffixes[i - 1]]
                    || class_eq[mid1] != class_eq[mid2]
                {
                    num_classes += 1;
                }
                class_eq_new[sorted_suffixes[i]] = num_classes - 1;
            }
            for i in 0..n {
                class_eq[i] = class_eq_new[i];
            }
        }

        let pos_in_sorted = rev_permutation(&sorted_suffixes);
        let lcp = Self::build_lcp(&str, &sorted_suffixes, &pos_in_sorted);
        if cfg!(debug_assertions) {
            // too slow for debug mode?
            for (w, &lcp) in sorted_suffixes.windows(2).zip(lcp.iter()) {
                let first = &str[w[0]..];
                let second = &str[w[1]..];
                assert!(
                    first < second,
                    "[{} -> {:?}] not less than [{} -> {:?}]",
                    w[0],
                    &str[w[0]..],
                    w[1],
                    &str[w[1]..]
                );
                let lcp = lcp as usize;
                assert!(first[0..lcp] == second[0..lcp]);
                assert_ne!(first.get(lcp), second.get(lcp));
            }
        }
        Self {
            sorted_suffixes,
            pos_in_sorted,
            lcp,
            lcp_sparse_table: RefCell::new(None),
        }
    }
}

impl Index<usize> for SuffixArray {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.sorted_suffixes[index]
    }
}
