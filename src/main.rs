use futures::TryStreamExt;
use geo::{BoundingRect, Contains};
use geozero::{wkb, ToWkt};
use sqlx::sqlite::SqlitePoolOptions;

use crate::geopackage::get_geoms;

mod geopackage;
mod msoa;

static PATH: &str = "./data/uk-zoomstack-geopacakge/OS_Open_Zoomstack.gpkg";

#[tokio::main]
async fn main() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(PATH)
        .await
        .unwrap();

    // brighton
    let msoa = "E02003523";
    let msoa_geojson = msoa::get_msoa_geojson(msoa);
    let msoa_geom: geo_types::Geometry<f64> = msoa_geojson.try_into().unwrap();
    let msoa_bbox = msoa_geom.bounding_rect().unwrap();

    let mut msoa_count = 0;
    let mut count = 0;
    // let rows: (wkb::Decode<geo_types::Geometry<f64>>,) =
    //     sqlx::query_as("SELECT geom FROM local_buildings").fetch(&pool);
    // let mut rows =
    //     sqlx::query_as::<_, Geom>("SELECT geom FROM local_buildings LIMIT 2").fetch(&pool);
    // if let Some(geom) = row.0.geometry {
    //     // println!("{}", geom.to_wkt().unwrap());
    //     count += 1;
    // }
    dbg!(&msoa_bbox);
    let mut stream = get_geoms("local_buildings", &msoa_bbox, &pool);
    while let Some(row) = stream.try_next().await.unwrap() {
        let geom = row.geom.geometry.unwrap();
        if msoa_geom.contains(&geom) {
            msoa_count += 1;
        }
        count += 1;
    }
    println!("MSOA Count is {}", msoa_count);
    println!("Count is {}", count);
}
