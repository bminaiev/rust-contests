use crate::{
    geometry::{half_plane_intersection::half_plane_intersection, polygon::PolygonT},
    misc::ord_f64::OrdF64,
};

type Polygon = PolygonT<OrdF64>;

pub fn convex_polygon_intersection(p1: &Polygon, p2: &Polygon) -> Option<Polygon> {
    let mut planes = vec![];
    for polygon in [p1, p2].iter() {
        for segment in polygon.edges() {
            planes.push(segment);
        }
    }
    half_plane_intersection(planes, None)
}
