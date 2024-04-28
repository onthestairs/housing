use std::fs::{self};

use clap::Parser;
use futures::TryStreamExt;
use geo::{BoundingRect, Contains, GeometryCollection, MapCoords};
use geojson::FeatureCollection;
use proj::Proj;
use sqlx::sqlite::SqlitePoolOptions;

use density::{data, geopackage, msoa};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the MSOA
    #[arg(short, long)]
    msoa: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&data::get_zoomstack_path())
        .await
        .unwrap();

    let msoa = args.msoa;
    let msoa_geojson = msoa::read_msoa_geojson(&msoa);
    let msoa_geom: geo_types::Geometry<f64> = msoa_geojson.try_into().unwrap();
    let msoa_bbox = msoa_geom.bounding_rect().unwrap();

    // projection
    let from = "EPSG:27700";
    let to = "EPSG:4326";
    let projection = Proj::new_known_crs(&from, &to, None).unwrap();

    let query = geopackage::make_query("local_buildings");
    let mut stream = geopackage::get_geoms(&query, &msoa_bbox, &pool);
    let mut geoms = Vec::new();
    while let Some(row) = stream.try_next().await.unwrap() {
        let geom = row.geom.geometry.unwrap();
        if msoa_geom.contains(&geom) {
            geoms.push(geom);
        }
    }

    let geometry_collection = GeometryCollection::from_iter(geoms);
    let geometry_collection_projected =
        geometry_collection.map_coords(|coord| projection.convert(coord).unwrap());
    let feature_collection = FeatureCollection::from(&geometry_collection_projected);
    let feature_collection_str = feature_collection.to_string();

    let path = data::get_msoa_local_buildings_path(&msoa);
    fs::write(path, feature_collection_str).unwrap();
}
