use std::cmp::{max, min, Ordering};

use crate::{
    geometry::{orientation::make_ccw, point::PointT},
    misc::{binary_search::binary_search_last_true, num_traits::Number},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Segment<T: Number> {
    fr: PointT<T>,
    to: PointT<T>,
    polygon_id: usize,
}

impl<T: Number + Ord> Segment<T> {
    pub fn get_lower_higher(&self) -> (PointT<T>, PointT<T>) {
        if self.fr.y < self.to.y {
            (self.fr, self.to)
        } else {
            (self.to, self.fr)
        }
    }

    pub fn cmp_p(&self, p: PointT<T>) -> Ordering {
        let (lower, higher) = self.get_lower_higher();
        if p.y < lower.y || p.y > higher.y {
            return Ordering::Equal;
        }
        PointT::vect_mul(&lower, &higher, &p).cmp(&T::ZERO)
    }
}

impl<T: Number + Ord> Ord for Segment<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_p(other.fr)
            .then_with(|| self.cmp_p(other.to))
            .then_with(|| other.cmp_p(self.fr).reverse())
            .then_with(|| other.cmp_p(self.to).reverse())
    }
}

impl<T: Number + Ord> PartialOrd for Segment<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PointLocation<T: Number> {
    all_y: Vec<T>,
    pub parents: Vec<Option<usize>>,
    tree_nodes: Vec<Vec<Segment<T>>>,
    same_y: Vec<Vec<Segment<T>>>,
}

impl<T: Number + Ord> PointLocation<T> {
    pub fn new(polygons: &[Vec<PointT<T>>]) -> Self {
        let mut all_y: Vec<T> = polygons
            .iter()
            .flat_map(|poly| poly.iter().map(|p| p.y))
            .collect();
        all_y.sort();
        all_y.dedup();
        let tree_nodes_cnt = all_y.len().next_power_of_two() * 2;
        let mut res = Self {
            same_y: vec![vec![]; all_y.len()],
            all_y,
            parents: vec![None; polygons.len()],
            tree_nodes: vec![vec![]; tree_nodes_cnt],
        };
        for (polygon_id, polygon) in polygons.iter().enumerate() {
            let polygon = make_ccw(polygon.clone());
            for i in 0..polygon.len() {
                let p = polygon[i];
                res.same_y[res.all_y.binary_search(&p.y).unwrap()].push(Segment {
                    fr: p,
                    to: p,
                    polygon_id,
                });
                let segment = Segment {
                    fr: polygon[i],
                    to: polygon[if i + 1 == polygon.len() { 0 } else { i + 1 }],
                    polygon_id,
                };
                if segment.fr.y == segment.to.y {
                    res.same_y[res.all_y.binary_search(&segment.fr.y).unwrap()].push(segment);
                } else {
                    res.add_segment(0, 0, res.all_y.len() - 1, &segment);
                }
            }
        }
        for node in res.tree_nodes.iter_mut() {
            node.sort();
        }
        for same_y in res.same_y.iter_mut() {
            same_y.sort_by_key(|s| s.fr.x + s.to.x);
        }

        let mut polygons_left_points: Vec<_> = polygons
            .iter()
            .enumerate()
            .map(|(id, poly)| (poly.iter().min().unwrap(), id))
            .collect();
        polygons_left_points.sort();
        for (&p, polygon_id) in polygons_left_points.into_iter() {
            res.parents[polygon_id] = res.locate_point(p, false);
        }
        res
    }

    fn add_segment(&mut self, tree_v: usize, l: usize, r: usize, segment: &Segment<T>) {
        let min_y = self.all_y[l];
        let max_y = self.all_y[r];
        let (lower, higher) = segment.get_lower_higher();
        if lower.y <= min_y && higher.y >= max_y {
            self.tree_nodes[tree_v].push(*segment);
        } else if lower.y >= max_y || higher.y <= min_y {
        } else {
            let m = (l + r) >> 1;
            self.add_segment(tree_v * 2 + 1, l, m, segment);
            self.add_segment(tree_v * 2 + 2, m, r, segment);
        }
    }

    pub fn locate_point(&self, p: PointT<T>, incl_bound: bool) -> Option<usize> {
        if incl_bound {
            if let Ok(y) = self.all_y.binary_search(&p.y) {
                let segs = &self.same_y[y];
                if let Ok(p1) = segs.binary_search_by(|seg| {
                    if p.x < min(seg.fr.x, seg.to.x) {
                        Ordering::Greater
                    } else if p.x > max(seg.fr.x, seg.to.x) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }) {
                    return Some(segs[p1].polygon_id);
                }
            }
        }
        let mut segment: Option<Segment<T>> = None;
        let mut tree_v = 0;
        let (mut l, mut r) = (0, self.all_y.len() - 1);
        loop {
            let min_y = self.all_y[l];
            let max_y = self.all_y[r];
            if p.y < min_y || p.y > max_y {
                break;
            }
            if let Some(idx) = binary_search_last_true(0..self.tree_nodes[tree_v].len(), |i| {
                match self.tree_nodes[tree_v][i].cmp_p(p) {
                    Ordering::Less => true,
                    Ordering::Equal => incl_bound,
                    Ordering::Greater => false,
                }
            }) {
                let new_segment = self.tree_nodes[tree_v][idx];
                if segment.is_none() || segment.unwrap().cmp(&new_segment) == Ordering::Less {
                    segment = Some(new_segment);
                }
            }
            if l + 1 < r {
                let m = (l + r) >> 1;
                let mid_y = self.all_y[m];
                if p.y < mid_y {
                    tree_v = tree_v * 2 + 1;
                    r = m;
                } else {
                    tree_v = tree_v * 2 + 2;
                    l = m;
                }
            } else {
                break;
            }
        }
        segment.and_then(|segment| {
            if segment.fr.y < segment.to.y {
                if PointT::vect_mul(&segment.fr, &segment.to, &p) == T::ZERO {
                    Some(segment.polygon_id)
                } else {
                    self.parents[segment.polygon_id]
                }
            } else {
                Some(segment.polygon_id)
            }
        })
    }
}
