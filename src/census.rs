use crate::data;

pub fn get_population_for_msoa(msoa: &str) -> u64 {
    let path = data::get_census_population_path();
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut count = 0;
    for result in rdr.records() {
        let record = result.unwrap();
        if record[0] == *msoa {
            let pop: u64 = record[4].parse().unwrap();
            count += pop;
        }
    }
    return count;
}

#[derive(serde::Serialize)]
pub struct Dwellings {
    pub total: u64,
    pub detached_or_semi: u64,
}

pub fn get_dwellings_for_msoa(msoa: &str) -> Dwellings {
    let path = data::get_census_accomodation_path();
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut total_count = 0;
    let mut detatched_or_semi_count = 0;
    for result in rdr.records() {
        let record = result.unwrap();
        if record[0] == *msoa {
            let observation: u64 = record[4].parse().unwrap();
            total_count += observation;

            let accomodation_type: u64 = record[2].parse().unwrap();
            if accomodation_type == 1 || accomodation_type == 2 {
                detatched_or_semi_count += observation;
            }
        }
    }
    return Dwellings {
        total: total_count,
        detached_or_semi: detatched_or_semi_count,
    };
}
