use density::data;
use geojson::GeoJson;
use std::{
    fs::{self},
    str::FromStr,
};

fn main() {
    let path = data::get_all_msoas_path();
    let geojson_str = fs::read_to_string(path).unwrap();
    let geo_json = GeoJson::from_str(&geojson_str).unwrap();
    if let GeoJson::FeatureCollection(collection) = geo_json {
        for f in collection.features {
            // the name of the MSOA
            let msoa = f.property("MSOA21CD").unwrap();
            let msoa_str = msoa.as_str().unwrap();
            let path = data::get_msoa_path(msoa_str);
            fs::write(path, f.to_string()).unwrap();
        }
    } else {
    }
}
