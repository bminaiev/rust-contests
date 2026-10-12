use crate::collections::last_exn::LastExn;
use crate::geometry::point::PointT;
use crate::geometry::polygon::PolygonT;
use crate::geometry::segment::SegmentT;
use crate::misc::num_traits::Double;
use std::collections::VecDeque;

fn filter_same_dir<T: Double>(sorted_planes: &[SegmentT<T>]) -> Vec<SegmentT<T>> {
    let mut res: Vec<SegmentT<T>> = vec![];
    for &plane in sorted_planes.iter() {
        if !res.is_empty() && res.last_exn().dir() == plane.dir() {
            let last = res.last_exn();
            if last.to_the_left(&plane.from) >= 0 {
                res.pop();
            } else {
                continue;
            }
        }
        res.push(plane);
    }
    res
}

///
/// Planes are to the left of each segment (not sure?)
///
pub fn half_plane_intersection<T: Double>(
    mut planes: Vec<SegmentT<T>>,
    add_inf_bound: Option<T>,
) -> Option<PolygonT<T>> {
    if let Some(inf) = add_inf_bound {
        let pts: Vec<_> = [(-inf, -inf), (inf, -inf), (inf, inf), (-inf, inf)]
            .into_iter()
            .map(|(x, y)| PointT::new(x, y))
            .collect();
        for i in 0..pts.len() {
            planes.push(SegmentT::new(pts[i], pts[(i + 1) % pts.len()]));
        }
    }
    planes.sort_by_key(|segment| segment.dir());
    planes = filter_same_dir(&planes);

    let mut deque: VecDeque<SegmentT<T>> = VecDeque::new();

    let pop_back = |deque: &mut VecDeque<SegmentT<T>>, check: &SegmentT<T>| -> bool {
        let len = deque.len();
        let inter = deque[len - 1]
            .to_line()
            .intersect(&deque[len - 2].to_line())
            .unwrap();
        if check.to_the_left(&inter) <= 0 {
            deque.pop_back();
            true
        } else {
            false
        }
    };

    let pop_front = |deque: &mut VecDeque<SegmentT<T>>, check: &SegmentT<T>| -> bool {
        let inter = deque[0].to_line().intersect(&deque[1].to_line()).unwrap();
        if check.to_the_left(&inter) <= 0 {
            deque.pop_front();
            true
        } else {
            false
        }
    };

    for plane in planes.iter() {
        if !deque.is_empty() {
            let last = deque.back().unwrap();
            if plane.dir() == last.dir() && plane.to_the_left(&last.from) >= 0 {
                continue;
            }
        }
        while deque.len() >= 2 {
            if !pop_back(&mut deque, plane) {
                break;
            }
        }

        while deque.len() >= 2 {
            if !pop_front(&mut deque, plane) {
                break;
            }
        }

        if !deque.is_empty() && deque[deque.len() - 1].dir().inverse() == plane.dir() {
            return None;
        }
        deque.push_back(*plane);
    }

    while deque.len() >= 3 {
        let first = deque[0];
        if !pop_back(&mut deque, &first) {
            break;
        }
    }

    while deque.len() >= 3 {
        let last = deque[deque.len() - 1];

        if !pop_front(&mut deque, &last) {
            break;
        }
    }

    if deque.len() < 3 {
        None
    } else {
        let len = deque.len();
        let mut res = Vec::with_capacity(len);
        for i in 0..len {
            res.push(
                deque[i]
                    .to_line()
                    .intersect(&deque[(i + 1) % len].to_line())
                    .unwrap(),
            );
        }
        Some(PolygonT::new(res))
    }
}
