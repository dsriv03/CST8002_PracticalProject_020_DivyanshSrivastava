mod importer;
mod model;
use model::crude_runs_dto::CrudeRunsDTO;

use crate::importer::csv::import_from_csv;

fn main() {
    println!("Hello, world!");

    import_from_csv(String::from("test"));

}
