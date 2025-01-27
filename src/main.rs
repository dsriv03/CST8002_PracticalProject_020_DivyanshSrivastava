mod importer;
mod model;

use model::crude_runs_dto::CrudeRunsDTO;

use crate::importer::csv::import_from_csv;

fn main() {

    //time benchmark begin
    let start = chrono::Local::now();
    //TODO: import location from props file 
    let list_of_entries: Vec<CrudeRunsDTO> = 
    import_from_csv(String::from("resources/data.csv")).expect("Could not load list of entries. (This would never trigger and should be handled in csv.rs in the future)");
    for mut entry in list_of_entries{
        entry.to_string();
    }

    //time benchmark end
    let end = chrono::Local::now();
    println!("Time taken: {}", start - end); //replace with a benchmark test

}
