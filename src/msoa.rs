use std::{fs, str::FromStr};

use geojson::{Feature, FeatureCollection, GeoJson};

pub fn read_msoa_geojson(msoa: &str) -> Feature {
    let filename = format!("./data/msoa/{}.geojson", msoa);
    let geojson_str = fs::read_to_string(filename).unwrap();
    let g = GeoJson::from_str(&geojson_str).unwrap();
    return match g {
        GeoJson::Geometry(_) => todo!(),
        GeoJson::Feature(f) => f,
        GeoJson::FeatureCollection(_) => todo!(),
    };
}

pub fn read_msoa_usable_geojson(msoa: &str) -> FeatureCollection {
    let filename = format!("./data/msoa-usable/{}.geojson", msoa);
    let geojson_str = fs::read_to_string(filename).unwrap();
    let g = GeoJson::from_str(&geojson_str).unwrap();
    return match g {
        GeoJson::Geometry(_) => todo!(),
        GeoJson::Feature(_) => todo!(),
        GeoJson::FeatureCollection(fc) => fc,
    };
}

pub fn read_msoa_local_buildings_geojson(msoa: &str) -> FeatureCollection {
    let filename = format!("./data/msoa-local-buildings/{}.geojson", msoa);
    let geojson_str = fs::read_to_string(filename).unwrap();
    let g = GeoJson::from_str(&geojson_str).unwrap();
    return match g {
        GeoJson::Geometry(_) => todo!(),
        GeoJson::Feature(_) => todo!(),
        GeoJson::FeatureCollection(fc) => fc,
    };
}
