use std::fs;

use clap::Parser;
use density::{
    census::{self, Dwellings},
    data, msoa,
};
use geo::GeodesicArea;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the MSOA
    #[arg(short, long)]
    msoa: String,
}

#[derive(serde::Serialize)]
struct Stats {
    usable_area_m2: f64,
    local_buildings_area_m2: f64,
    built_up_area_percent: f64,
    population: u64,
    dwellings: Dwellings,
}

fn main() {
    let args = Args::parse();

    let msoa = args.msoa;
    let msoa_usable_geojson = msoa::read_msoa_usable_geojson(&msoa);
    let msoa_usable_geom: geo_types::Geometry<f64> = msoa_usable_geojson.try_into().unwrap();
    let usable_area = msoa_usable_geom.geodesic_area_signed();

    let msoa_local_buildings_geojson = msoa::read_msoa_local_buildings_geojson(&msoa);
    let msoa_local_buildings_geom: geo_types::Geometry<f64> =
        msoa_local_buildings_geojson.try_into().unwrap();
    let local_buildings_area = msoa_local_buildings_geom.geodesic_area_signed();

    let built_up_area_percent = local_buildings_area / usable_area * 100.0;

    let population = census::get_population_for_msoa(&msoa);
    let dwellings = census::get_dwellings_for_msoa(&msoa);

    let stats = Stats {
        usable_area_m2: usable_area,
        local_buildings_area_m2: local_buildings_area,
        built_up_area_percent,
        population,
        dwellings,
    };

    let path = data::get_stats_path(&msoa);
    fs::write(path, serde_json::to_string(&stats).unwrap()).unwrap();
}
