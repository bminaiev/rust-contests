use std::cmp::Ordering;

use crate::math::gcd::gcd;

#[derive(Clone, Copy, Default, Debug)]
pub struct Frac {
    pub num: i64,
    pub denum: i64,
}

impl Frac {
    pub fn new(mut num: i64, mut denum: i64) -> Self {
        if denum == 0 {
            return match num.cmp(&0) {
                Ordering::Less => Self { num: -1, denum: 0 },
                Ordering::Equal => Self { num: 0, denum: 0 },
                Ordering::Greater => Self { num: 1, denum: 0 },
            };
        }
        if denum < 0 {
            num *= -1;
            denum *= -1;
        }
        let g = gcd(num, denum);
        Self {
            num: num / g,
            denum: denum / g,
        }
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.denum == other.denum
    }
}
impl Eq for Frac {}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.num * other.denum).cmp(&(other.num * self.denum)))
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
