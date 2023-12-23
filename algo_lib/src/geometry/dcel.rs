use crate::{
    f,
    geometry::{line::Line, point::PointT, polygon::PolygonT},
    misc::ord_f64::OrdF64,
};

// assumes no equal lines
pub fn dcel(lines: &[Line]) -> Vec<PolygonT<OrdF64>> {
    type Point = PointT<OrdF64>;

    let mut all_points = vec![];
    let mut on_line = vec![vec![]; lines.len()];
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if let Some(p) = lines[i].intersect(&lines[j]) {
                all_points.push(p);
            }
        }
    }
    let cmp = |p1: &Point, p2: &Point| {
        let dx = (p1.x - p2.x).abs();
        if dx < OrdF64::EPS {
            p1.y.partial_cmp(&p2.y).unwrap()
        } else {
            p1.x.partial_cmp(&p2.x).unwrap()
        }
    };
    all_points.sort_by(cmp);
    all_points.dedup_by(|p1, p2| {
        let dx = (p1.x - p2.x).abs();
        let dy = (p1.y - p2.y).abs();
        dx < OrdF64::EPS && dy < OrdF64::EPS
    });
    for i in 0..lines.len() {
        for j in 0..all_points.len() {
            if lines[i].on_line(&all_points[j]) {
                on_line[i].push(j);
            }
        }
        on_line[i].sort();
    }
    let mut all_edges = vec![];
    let mut g = vec![vec![]; all_points.len()];
    for i in 0..lines.len() {
        for w in on_line[i].windows(2) {
            let from = w[0];
            let to = w[1];
            all_edges.push((from, to));
            g[from].push(all_edges.len() - 1);
            all_edges.push((to, from));
            g[to].push(all_edges.len() - 1);
        }
    }
    for v in 0..g.len() {
        g[v].sort_by(|&to1, &to2| {
            let p1 = all_points[all_edges[to1].1] - all_points[v];
            let p2 = all_points[all_edges[to2].1] - all_points[v];
            let s1 = p1.side();
            let s2 = p2.side();
            if s1 != s2 {
                s1.cmp(&s2)
            } else {
                Point::vect_mul2(&p1, &p2).cmp(&f!(0.0))
            }
        });
    }
    let mut edge_pos = vec![usize::MAX; all_edges.len()];
    for v in 0..g.len() {
        for i in 0..g[v].len() {
            let to = g[v][i];
            edge_pos[to] = i;
        }
    }
    let mut seen = vec![false; all_edges.len()];
    let mut polygons = vec![];
    for start_edge in 0..all_edges.len() {
        if seen[start_edge] {
            continue;
        }
        let mut cur_edge = start_edge;
        let mut points = vec![];
        while !seen[cur_edge] {
            seen[cur_edge] = true;
            let (fr, to) = all_edges[cur_edge];
            points.push(all_points[fr]);
            let pos = edge_pos[cur_edge ^ 1];
            let need_pos = (pos + 1) % g[to].len();
            cur_edge = g[to][need_pos];
        }
        let polygon = PolygonT::new(points);
        let area = polygon.area_signed().0;
        if area < 0.0 {
            // skip external side
            continue;
        }
        polygons.push(polygon);
    }
    polygons
}
