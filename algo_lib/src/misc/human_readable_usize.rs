use std::fmt::Debug;

pub struct HumanReadableUsize(pub usize);

impl Debug for HumanReadableUsize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.0;
        if v < 1000 {
            write!(f, "{}", self.0)?
        } else if v < 10_000 {
            write!(f, "{}.{}K", v / 1000, (v % 1000) / 100)?
        } else if v < 1_000_000 {
            write!(f, "{}K", v / 1000)?
        } else if v < 10_000_000 {
            write!(f, "{}.{}M", v / 1_000_000, (v % 1_000_000) / (100_000))?
        } else {
            write!(f, "{}M", v / 1_000_000)?
        }
        Ok(())
    }
}
