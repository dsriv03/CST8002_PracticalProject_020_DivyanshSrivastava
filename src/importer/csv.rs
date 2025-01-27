
use std::{error::Error, fs::File};

use chrono::NaiveDate;

use crate::model::crude_runs_dto::CrudeRunsDTO;

//Vec<CrudeRunsDTO>

//file handles both import and construction, possibly seperate both responsibilities in future update
pub fn import_from_csv(path: String) -> Result<Vec<CrudeRunsDTO>, Box<dyn Error>>{


    let mut vec_dto: Vec<CrudeRunsDTO> = Vec::new();

    let file = File::open(path).expect("File missing/inaccessible.");
    let mut rdr = csv::ReaderBuilder::new()
    .from_reader(file);

    let mut import_counter: u128 = 1; //used as primary ID, TODO: move it to the struct? if possible
    
    for result in rdr.records() {
        let record = result?;

        let mut entry = CrudeRunsDTO::default();

        //unhandled unwraps, TODO: implement serde
        entry.set_id(import_counter);
        entry.set_weekly_end(NaiveDate::parse_from_str(&record.get(0).unwrap(), "%m/%d/%Y").unwrap());
        entry.set_week_end_last_year(NaiveDate::parse_from_str(&record.get(1).unwrap(), "%m/%d/%Y").unwrap());
        entry.set_region(String::from(record.get(2).unwrap()));
        entry.set_volume(record.get(3).expect("Not a num").parse()?);
        entry.set_capacity(record.get(4).expect("Not a num").parse()?);
        entry.set_four_week_avg(record.get(5).expect("Not a num").parse()?);
        entry.set_four_week_avg_lastyear(record.get(6).expect("Not a num").parse()?);
        entry.set_ytd_avg(record.get(7).expect("Not a num").parse()?);
        entry.set_ytd_avg_last_year(record.get(8).expect("Not a num").parse()?);
        entry.set_unit(String::from(record.get(9).unwrap()));

        vec_dto.push(entry);
        import_counter += 1;
    }
    println!("{} successfully imported.", import_counter);
    Ok(vec_dto)
}
