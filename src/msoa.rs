use std::{fs, str::FromStr};

use geojson::{Feature, GeoJson};

pub fn get_msoa_geojson(msoa: &str) -> Feature {
    let filename = format!("./data/msoa/{}.geojson", msoa);
    let geojson_str = fs::read_to_string(filename).unwrap();
    let g = GeoJson::from_str(&geojson_str).unwrap();
    return match g {
        GeoJson::Geometry(_) => todo!(),
        GeoJson::Feature(f) => f,
        GeoJson::FeatureCollection(_) => todo!(),
    };
}
