use geojson::GeoJson;
use std::{
    fs::{self, File},
    io::BufRead,
    str::FromStr,
};

fn main() {
    let geojson_str = fs::read_to_string("./data/msoa/census-2021.geojson").unwrap();
    let geo_json = GeoJson::from_str(&geojson_str).unwrap();
    if let GeoJson::FeatureCollection(collection) = geo_json {
        for f in collection.features {
            let msoa = f.property("MSOA21CD").unwrap();
            let msoa_str = msoa.as_str().unwrap();
            let filename = format!("./data/msoa/{}.geojson", msoa_str);
            fs::write(filename, f.to_string()).unwrap();
        }
    } else {
    }
}
