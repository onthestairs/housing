use std::fs::{self};

use clap::Parser;
use futures::TryStreamExt;
use geo::{BoundingRect, GeometryCollection, MapCoords};
use geojson::FeatureCollection;
use proj::Proj;
use sqlx::sqlite::SqlitePoolOptions;

use density::{data, geo_helpers, geopackage, msoa};

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

    let mut usable_msoa = msoa_geom.clone();

    let unusable_layers = vec!["national_parks", "greenspace", "woodland"];
    for unusable_layer in unusable_layers {
        let query = geopackage::make_query(&unusable_layer);
        let mut stream = geopackage::get_geoms(&query, &msoa_bbox, &pool);
        while let Some(row) = stream.try_next().await.unwrap() {
            let geom = row.geom.geometry.unwrap();
            usable_msoa = geo_helpers::geometries_difference(&usable_msoa, &geom);
        }
    }

    let usable_msoa_projected = usable_msoa.map_coords(|coord| projection.convert(coord).unwrap());
    let usable_msoa_feature_collection = GeometryCollection::from(usable_msoa_projected);
    let feature_collection = FeatureCollection::from(&usable_msoa_feature_collection);
    let feature_collection_str = feature_collection.to_string();
    let path = data::get_msoa_usable_path(&msoa);
    fs::write(path, feature_collection_str).unwrap();
}
