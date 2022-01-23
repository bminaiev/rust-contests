struct SubmaskIter {
    full_mask: usize,
    cur: Option<usize>,
}

impl Iterator for SubmaskIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cur) = self.cur {
            let res = cur;
            self.cur = if cur == 0 {
                None
            } else {
                Some((cur - 1) & self.full_mask)
            };
            Some(res)
        } else {
            None
        }
    }
}

pub fn all_submasks_of(mask: usize) -> impl Iterator<Item = usize> {
    SubmaskIter {
        cur: Some(mask),
        full_mask: mask,
    }
}