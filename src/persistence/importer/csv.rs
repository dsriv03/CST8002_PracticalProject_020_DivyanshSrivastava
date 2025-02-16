/*
    Author: Divyansh Srivastava
    Student Number: 041109063
    Submission: Practical Project 1 for CST 8002_020
    Submission Date: 26/1/2025
*/

use std::{error::Error, fs::File};
use crate::crude_runs_dto::CrudeRunsDTO;

use chrono::NaiveDate;

    /// Handles importing data from a csv file and packaging them into a vector
    ///
    /// # Arguments
    ///
    /// * `path to file` - Path to the .csv file to import
    ///
    /// # Returns
    /// 
    /// * `All Crude Runs vector OR Error` - Result object containing a CrudeRunsDTO vector if succeeded, or a dynamic error object
pub fn import_from_csv(path: String) -> Result<Vec<CrudeRunsDTO>, Box<dyn Error>>{
//file handles both import and construction, TODO: possibly seperate both responsibilities in future update

    // DTO vector that stores all entries and gets returned to the caller
    let mut vec_dto: Vec<CrudeRunsDTO> = Vec::new();

    // Open data.csv from the path provided, return an error message if the file is missing.
    let file = File::open(path).expect("File missing/inaccessible.");
    let mut rdr = csv::ReaderBuilder::new()
    .from_reader(file);

    // ID Counter
    let mut import_counter: u128 = 1; //TODO: move it to the struct? if possible and increment it there
    
    // For loop that iterates over the reader iterator and returns a Record array
    for result in rdr.records() {
        let record = result?;

        // Instantiate default object
        let mut entry = CrudeRunsDTO::default();

        // Set each field variable after unwrapping and parsing
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

        // Insert a successfully created object into the vector
        vec_dto.push(entry);

        // increment ID count
        import_counter += 1;

        //TODO: create better logic to dynamically change entry count limit (pass it as an argument?)
        if import_counter > 100{
            import_counter -= 1;
            println!("{} successfully imported.", import_counter);
            return Ok(vec_dto);
        }
    }
    // Status message, TODO: move into a debug log in the future
    import_counter -= 1;
    println!("{} successfully imported.", import_counter);

    // Return a successful Result touple with the DTO vector
    Ok(vec_dto)
}
