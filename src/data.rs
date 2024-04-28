pub fn get_zoomstack_path() -> String {
    return "./data/uk-zoomstack-geopackage/OS_Open_Zoomstack.gpkg".to_string();
}

pub fn get_all_msoas_path() -> String {
    return format!("./data/msoa-all/2021.geojson");
}

pub fn get_msoa_path(msoa: &str) -> String {
    return format!("./data/msoa/{}.geojson", msoa);
}

pub fn get_msoa_local_buildings_path(msoa: &str) -> String {
    return format!("./data/msoa-local-buildings/{}.geojson", msoa);
}

pub fn get_msoa_usable_path(msoa: &str) -> String {
    return format!("./data/msoa-usable/{}.geojson", msoa);
}

pub fn get_census_accomodation_path() -> String {
    return format!("./data/census/accomodation.csv");
}

pub fn get_census_population_path() -> String {
    return format!("./data/census/population.csv");
}

pub fn get_stats_path(msoa: &str) -> String {
    return format!("./data/stats/{}.json", msoa);
}
