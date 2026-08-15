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
    #[inline]
    pub fn get_pos_in_array(&self, pos_in_string: usize) -> usize {
        self.pos_in_sorted[pos_in_string]
    }

    pub fn len(&self) -> usize {
        self.sorted_suffixes.len()
    }

    fn lcp_sparse_table(&self) -> Ref<'_, SparseTableMin<u32>> {
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
        let mut k = 0usize;
        for i in 0..n {
            k = k.saturating_sub(1);
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
        T: Number + Ord,
    {
        str.push(T::ZERO);
        let n = str.len();
        // Rank-compress to a dense integer alphabet; the appended sentinel gets
        // rank 0, strictly smaller than everything else.
        let mut order = gen_vec(n - 1, |x| x);
        order.sort_unstable_by_key(|&id| str[id]);
        let mut ranks = vec![0u32; n];
        let mut num_ranks = 1u32;
        for (pos, win) in order.windows(2).enumerate() {
            if pos == 0 {
                ranks[win[0]] = 1;
            }
            if str[win[1]] != str[win[0]] {
                num_ranks += 1;
            }
            ranks[win[1]] = num_ranks;
        }
        if n == 2 {
            ranks[order[0]] = 1;
        }
        let sorted_suffixes = sais(&ranks, num_ranks as usize + 1);
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

// SA-IS, O(n). `s` must end with a unique smallest character (sentinel) and
// contain values in 0..alphabet.
fn sais(s: &[u32], alphabet: usize) -> Vec<usize> {
    const EMPTY: usize = usize::MAX;
    let n = s.len();
    if n == 1 {
        return vec![0];
    }
    let mut is_s = vec![false; n];
    is_s[n - 1] = true;
    for i in (0..n - 1).rev() {
        is_s[i] = s[i] < s[i + 1] || (s[i] == s[i + 1] && is_s[i + 1]);
    }
    let mut cnt = vec![0usize; alphabet];
    for &c in s {
        cnt[c as usize] += 1;
    }
    let bucket_starts = |bkt: &mut Vec<usize>| {
        let mut sum = 0;
        for (b, &c) in bkt.iter_mut().zip(cnt.iter()) {
            *b = sum;
            sum += c;
        }
    };
    let bucket_ends = |bkt: &mut Vec<usize>| {
        let mut sum = 0;
        for (b, &c) in bkt.iter_mut().zip(cnt.iter()) {
            sum += c;
            *b = sum;
        }
    };
    let mut sa = vec![EMPTY; n];
    let mut bkt = vec![0usize; alphabet];
    let induce = |sa: &mut Vec<usize>, bkt: &mut Vec<usize>, lms_in_order: &[usize]| {
        sa.fill(EMPTY);
        bucket_ends(bkt);
        for &p in lms_in_order.iter().rev() {
            let c = s[p] as usize;
            bkt[c] -= 1;
            sa[bkt[c]] = p;
        }
        bucket_starts(bkt);
        for i in 0..n {
            let j = sa[i];
            if j != EMPTY && j > 0 && !is_s[j - 1] {
                let c = s[j - 1] as usize;
                sa[bkt[c]] = j - 1;
                bkt[c] += 1;
            }
        }
        bucket_ends(bkt);
        for i in (0..n).rev() {
            let j = sa[i];
            if j != EMPTY && j > 0 && is_s[j - 1] {
                let c = s[j - 1] as usize;
                bkt[c] -= 1;
                sa[bkt[c]] = j - 1;
            }
        }
    };
    let lms: Vec<usize> = (1..n).filter(|&i| is_s[i] && !is_s[i - 1]).collect();
    induce(&mut sa, &mut bkt, &lms);
    let mut sorted_lms: Vec<usize> = sa
        .iter()
        .copied()
        .filter(|&p| p > 0 && is_s[p] && !is_s[p - 1])
        .collect();
    // Name LMS substrings by their rank; equal substrings get equal names.
    let mut next_lms = vec![0usize; n];
    for w in lms.windows(2) {
        next_lms[w[0]] = w[1];
    }
    next_lms[*lms.last().unwrap()] = n - 1;
    let mut name_of = vec![0u32; n];
    let mut num_names = 1u32;
    for w in sorted_lms.windows(2) {
        let (p1, p2) = (w[0], w[1]);
        let (e1, e2) = (next_lms[p1], next_lms[p2]);
        if e1 - p1 != e2 - p2 || s[p1..=e1] != s[p2..=e2] {
            num_names += 1;
        }
        name_of[p2] = num_names - 1;
    }
    if (num_names as usize) < sorted_lms.len() {
        let reduced: Vec<u32> = lms.iter().map(|&p| name_of[p]).collect();
        let reduced_sa = sais(&reduced, num_names as usize);
        for (target, &i) in sorted_lms.iter_mut().zip(reduced_sa.iter()) {
            *target = lms[i];
        }
    }
    induce(&mut sa, &mut bkt, &sorted_lms);
    sa
}
