use clap::Parser;
use density::msoa;
use geo::{algorithm::area::Area, GeodesicArea};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the MSOA
    #[arg(short, long)]
    msoa: String,
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

    println!(
        "MSOA: {}\nUsable area (m2): {}\nLocal buildings area (m2): {}\nBuilt-up area percent: {}%",
        msoa, usable_area, local_buildings_area, built_up_area_percent
    );
}
