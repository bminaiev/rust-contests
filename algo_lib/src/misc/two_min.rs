#[derive(Clone)]
pub struct TwoMin<IdType: Eq, ValueType: Ord> {
    cnt: usize,
    values: [(IdType, ValueType); 2],
}

impl<IdType: Eq + Copy, ValueType: Ord + Copy> TwoMin<IdType, ValueType> {
    pub fn new(zero_id: IdType, zero_value: ValueType) -> Self {
        Self {
            cnt: 0,
            values: [(zero_id, zero_value), (zero_id, zero_value)],
        }
    }

    fn make_sorted(&mut self) {
        if self.cnt == 2 && self.values[0].1 > self.values[1].1 {
            self.values.swap(0, 1);
        }
    }

    pub fn add(&mut self, id: IdType, value: ValueType) -> bool {
        if self.cnt >= 1 && self.values[0].0 == id {
            if self.values[0].1 <= value {
                return false;
            }
            self.values[0].1 = value;
            return true;
        }
        if self.cnt >= 2 && self.values[1].0 == id {
            if self.values[1].1 <= value {
                return false;
            }
            self.values[1].1 = value;
            self.make_sorted();
            return true;
        }
        if self.cnt == 0 {
            self.cnt = 1;
            self.values[0] = (id, value);
            return true;
        }
        if self.cnt == 1 {
            self.cnt += 1;
            self.values[1] = (id, value);
            self.make_sorted();
            return true;
        }
        if self.cnt == 2 {
            if self.values[1].1 <= value {
                return false;
            }
            self.values[1] = (id, value);
            self.make_sorted();
            return true;
        }
        unreachable!("cnt is greater than 2?");
    }

    pub fn get_value_by_id(&self, id: IdType) -> Option<ValueType> {
        if self.cnt >= 1 && self.values[0].0 == id {
            return Some(self.values[0].1);
        }
        if self.cnt >= 2 && self.values[1].0 == id {
            return Some(self.values[1].1);
        }
        None
    }

    pub fn get_value_by_not_id(&self, not_id: IdType) -> Option<ValueType> {
        if self.cnt >= 1 && self.values[0].0 != not_id {
            return Some(self.values[0].1);
        }
        if self.cnt >= 2 && self.values[1].0 != not_id {
            return Some(self.values[1].1);
        }
        None
    }

    pub fn get_by_not_id(&self, not_id: IdType) -> Option<(IdType, ValueType)> {
        if self.cnt >= 1 && self.values[0].0 != not_id {
            return Some((self.values[0].0, self.values[0].1));
        }
        if self.cnt >= 2 && self.values[1].0 != not_id {
            return Some((self.values[1].0, self.values[1].1));
        }
        None
    }
}
