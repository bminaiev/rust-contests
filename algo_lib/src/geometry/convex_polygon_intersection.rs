use crate::{
    geometry::{half_plane_intersection::half_plane_intersection, polygon::PolygonT},
    misc::num_traits::Double,
};

pub fn convex_polygon_intersection<T: Double>(
    p1: &PolygonT<T>,
    p2: &PolygonT<T>,
) -> Option<PolygonT<T>> {
    let mut planes = vec![];
    for polygon in [p1, p2].iter() {
        for segment in polygon.edges() {
            planes.push(segment);
        }
    }
    half_plane_intersection(planes, None)
}
