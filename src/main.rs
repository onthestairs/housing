use std::fs::{self, File};

use futures::TryStreamExt;
use geo::{BoundingRect, Contains, GeometryCollection, MapCoords};
use geojson::FeatureCollection;
use geozero::{wkb, ToWkt};
use proj::Proj;
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
    // let msoa = "E02003523";
    let msoa = "E02004015";
    let msoa_geojson = msoa::get_msoa_geojson(msoa);
    let msoa_geom: geo_types::Geometry<f64> = msoa_geojson.try_into().unwrap();
    let msoa_bbox = msoa_geom.bounding_rect().unwrap();

    // projection
    let from = "EPSG:27700";
    let to = "EPSG:4326";
    let projection = Proj::new_known_crs(&from, &to, None).unwrap();

    let mut msoa_count = 0;
    let mut count = 0;
    dbg!(&msoa_bbox);
    let mut stream = get_geoms("local_buildings", &msoa_bbox, &pool);
    let mut geoms = Vec::new();
    while let Some(row) = stream.try_next().await.unwrap() {
        let geom = row.geom.geometry.unwrap();
        if msoa_geom.contains(&geom) {
            msoa_count += 1;
            geoms.push(geom);
        }
        count += 1;
    }

    let geometry_collection = GeometryCollection::from_iter(geoms);
    let geometry_collection_projected =
        geometry_collection.map_coords(|coord| projection.convert(coord).unwrap());
    let feature_collection = FeatureCollection::from(&geometry_collection_projected);
    let feature_collection_str = feature_collection.to_string();
    let filename = format!("./data/msoa-local-buildings/{}.geojson", msoa);
    dbg!(&filename);
    fs::write(filename, feature_collection_str).unwrap();

    println!("MSOA Count is {}", msoa_count);
    println!("Count is {}", count);
}
