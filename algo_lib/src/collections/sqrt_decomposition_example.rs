use crate::collections::sqrt_decomposition::{Part, SqrtDecomposition, SqrtNode};
use crate::io::input::Input;
use crate::io::output::output;
use crate::misc::binary_search::binary_search_first_true;
use crate::misc::min_max::UpdateMinMax;
use crate::{dbg, out, out_line};
use std::cmp::{max, min};

#[derive(Clone)]
struct Node {
    raw_values: Vec<i64>,
    set_min: i64,
    set_max: i64,
    add: i64,
    sorted_values: Vec<i64>,
}

impl Node {
    pub fn new(a: &[i64]) -> Self {
        let mut res = Self {
            raw_values: a.to_vec(),
            set_min: i64::MIN,
            set_max: i64::MAX,
            add: 0,
            sorted_values: vec![],
        };
        res.rebuild();
        res
    }

    fn apply_op(&self, raw: i64) -> i64 {
        min(self.set_max, max(self.set_min, raw)) + self.add
    }

    fn count_le_values(&self, value: i64) -> usize {
        binary_search_first_true(0..self.sorted_values.len(), |pos| {
            self.apply_op(self.sorted_values[pos]) > value
        })
    }
}

impl SqrtNode for Node {
    fn relax(&mut self) {
        for pos in 0..self.raw_values.len() {
            self.raw_values[pos] = self.apply_op(self.raw_values[pos]);
        }
    }

    fn rebuild(&mut self) {
        self.sorted_values.clear();
        for x in self.raw_values.iter() {
            self.sorted_values.push(*x);
        }
        self.sorted_values.sort();

        self.set_min = i64::MIN;
        self.set_max = i64::MAX;
        self.add = 0;
    }
}

pub fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let q = input.usize();
    let a = input.read_vec::<i64>(n);
    let mut sqrt = SqrtDecomposition::new(n, 300, |range| Node::new(&a[range]));
    for _ in 0..q {
        let query_type = input.usize() - 1;
        let full_range = input.usize() - 1..input.usize();
        if query_type == 3 {
            let min_val = input.i64();
            let max_val = input.i64();
            let mut res = 0;
            sqrt.iter_mut(full_range, |part| match part {
                Part::Full(node) => {
                    res += node.count_le_values(max_val) - node.count_le_values(min_val - 1);
                }
                Part::Range(node, range) => {
                    for &val in node.raw_values[range].iter() {
                        if val >= min_val && val <= max_val {
                            res += 1;
                        }
                    }
                }
            });
            out_line!(res);
        } else {
            let x = input.i64();
            sqrt.iter_mut(full_range, |part| match part {
                Part::Full(node) => {
                    match query_type {
                        0 => {
                            node.set_max.update_min(x - node.add);
                            node.set_min.update_min(x - node.add);
                        }
                        1 => {
                            node.set_min.update_max(x - node.add);
                            node.set_max.update_max(x - node.add);
                        }
                        2 => node.add += x,
                        _ => unreachable!(),
                    };
                }
                Part::Range(node, range) => {
                    for val in node.raw_values[range].iter_mut() {
                        match query_type {
                            0 => {
                                val.update_min(x);
                            }
                            1 => {
                                val.update_max(x);
                            }
                            2 => *val += x,
                            _ => unreachable!(),
                        };
                    }
                }
            });
        }
    }
}
