use std::error::Error;
use std::str::FromStr;

use crate::persistence::model::crude_runs_dto::CrudeRunsDTO;
use crate::csv::import_from_csv;

use uuid::Uuid;

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
    for entry in &self.entries{
        entry.to_string();
        //TODO: Shift this to presentation
        println!("Practical Project 1 by Divyansh Srivastava, 041109063.")
    }
    }

    pub fn write_to_csv(&self) -> Result<(), Box<dyn Error>>{
 
        let id = Uuid::new_v4().to_string();
        let mut path: String = String::from_str("resources/").expect("Bad input");
        path.push_str(id.as_str());
        path.push_str(".csv");
        
        let mut writer = csv::Writer::from_path(path)?;
        for entry in &self.entries{
            writer.serialize(&entry);
        }
        Ok(())
    }

    pub fn load_by_id(&self, id: usize) -> Option<&CrudeRunsDTO>{
        
        self.entries.get(id-1)

    }

    pub fn create_entry(&mut self, item: CrudeRunsDTO) {

        self.entries.push(item);
    }

    pub fn update_entry(&mut self, id: usize, item: CrudeRunsDTO){

        //Input is validated in presentation layer
        self.entries.remove(id);
    }


    
}