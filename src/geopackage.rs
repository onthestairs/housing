use std::pin::Pin;

use futures::{Stream, TryStreamExt};
use geo::Rect;
use geozero::{wkb, ToWkt};
use sqlx::{sqlite::SqlitePoolOptions, Error, Pool, Sqlite};

#[derive(sqlx::FromRow)]
pub struct Geom {
    pub geom: wkb::Decode<geo_types::Geometry<f64>>,
}

pub fn get_geoms<'a>(
    layer: &'a str,
    bbox: &'a Rect<f64>,
    pool: &'a Pool<Sqlite>,
    // ) -> Pin<Box<dyn Stream<Item = Result<<Self::Database as Database>::Row, Error>> + Send>> {
) -> Pin<Box<dyn Stream<Item = Result<Geom, Error>> + Send + 'a>> {
    // let query = format!("SELECT geom FROM {}", layer.clone());
    let rows = sqlx::query_as::<_, Geom>(
        "SELECT geom from local_buildings WHERE id IN 
             (SELECT id FROM rtree_local_buildings_geom WHERE 
               minx >= ? AND maxx <= ? AND
               miny >= ? AND maxy <= ?);",
    )
    .bind(bbox.min().x)
    .bind(bbox.max().x)
    .bind(bbox.min().y)
    .bind(bbox.max().y)
    .fetch(pool);
    return rows;
}
