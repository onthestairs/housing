use geo::BooleanOps;

pub fn geometries_difference(
    g1: &geo_types::Geometry<f64>,
    g2: &geo_types::Geometry<f64>,
) -> geo_types::Geometry<f64> {
    return match (g1, g2) {
        (geo_types::Geometry::MultiPolygon(p1), geo_types::Geometry::MultiPolygon(p2)) => {
            geo_types::Geometry::MultiPolygon(p1.difference(p2))
        }
        (geo_types::Geometry::MultiPolygon(p1), geo_types::Geometry::Polygon(p2)) => {
            let p2_multi = geo_types::MultiPolygon::new(vec![p2.clone()]);
            geo_types::Geometry::MultiPolygon(p1.difference(&p2_multi))
        }
        (geo_types::Geometry::Polygon(p1), geo_types::Geometry::Polygon(p2)) => {
            geo_types::Geometry::MultiPolygon(p1.difference(p2))
        }
        _ => g1.clone(),
    };
}
