use std::pin::Pin;

use futures::Stream;
use geo::Rect;
use geozero::wkb;
use sqlx::{Error, Pool, Sqlite};

#[derive(sqlx::FromRow)]
pub struct Geom {
    pub geom: wkb::Decode<geo_types::Geometry<f64>>,
}

pub fn make_query(layer: &str) -> String {
    format!(
        "SELECT geom from {} WHERE id IN 
         (SELECT id FROM rtree_{}_geom WHERE 
              (minx >= ? AND miny >= ? AND minx <= ? AND miny <= ?)
           OR (maxx >= ? AND maxy >= ? AND maxx <= ? AND maxy <= ?)
         );",
        layer, layer
    )
}

pub fn get_geoms<'a>(
    query: &'a str,
    bbox: &'a Rect<f64>,
    pool: &'a Pool<Sqlite>,
    // ) -> Pin<Box<dyn Stream<Item = Result<<Self::Database as Database>::Row, Error>> + Send>> {
) -> Pin<Box<dyn Stream<Item = Result<Geom, Error>> + Send + 'a>> {
    let rows = sqlx::query_as::<_, Geom>(&query)
        // mins
        .bind(bbox.min().x)
        .bind(bbox.min().y)
        .bind(bbox.max().x)
        .bind(bbox.max().y)
        // maxs
        .bind(bbox.min().x)
        .bind(bbox.min().y)
        .bind(bbox.max().x)
        .bind(bbox.max().y)
        // fetch
        .fetch(pool);
    return rows;
}
