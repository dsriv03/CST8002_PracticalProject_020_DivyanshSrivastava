use crate::persistence::model::crude_runs_dto::CrudeRunsDTO;
use crate::csv::import_from_csv;

pub struct CrudeRunsDao{

    entries: Vec<CrudeRunsDTO>

}

impl CrudeRunsDao{

    pub fn new()-> Self{
        CrudeRunsDao {
            entries: Vec::new()
        }
    }

    pub fn load_all_runs(&mut self){
    // Call importer to get the DTO vector with all imported entries
    //TODO: import location from props file 
    let list_of_entries: Vec<CrudeRunsDTO> = 
    import_from_csv(String::from("resources/data.csv"))
    .expect("Could not load list of entries. (This would never trigger and should be handled in csv.rs in the future)");

    //TODO: handle csv error here?
    self.entries = list_of_entries;

    // Iteratore over the DTO vector and call to string on each
    for entry in &mut self.entries{
        entry.to_string();
        println!("Practical Project 1 by Divyansh Srivastava, 041109063.")
    }
    }

}