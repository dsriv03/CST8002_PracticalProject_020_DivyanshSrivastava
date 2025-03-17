use std::error::Error;
use std::str::FromStr;

use crate::persistence::model::crude_runs_dto::CrudeRunsDTO;
use crate::persistence::importer::csv::import_from_csv;
use crate::persistence::formats::writable;

use uuid::Uuid;

use super::writable::Writable;

pub struct InMemory{

    entries: Vec<CrudeRunsDTO>

}

impl Writable for InMemory{

    fn load_all_runs(&mut self) -> &Vec<CrudeRunsDTO>{
    // Call importer to get the DTO vector with all imported entries
    //TODO: import location from props file 
    let list_of_entries: Vec<CrudeRunsDTO> = 
    import_from_csv(String::from("resources/data.csv"))
    .expect("Could not load list of entries. (This would never trigger and should be handled in csv.rs in the future)");

    //TODO: handle csv error here?
    self.entries = list_of_entries;

    &self.entries
    }

    fn persist(&self) -> Result<(), Box<dyn Error>>{
 
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

    fn load_by_id(&self, id: usize) -> Option<&CrudeRunsDTO>{
        
        self.entries.get(id)

    }

    fn create_entry(&mut self, item: CrudeRunsDTO) {

        self.entries.push(item);
    }

    fn update_entry(&mut self, id: usize, item: CrudeRunsDTO){

        //push to last index of vector
        self.create_entry(item);
        //swap input index with last index and drop the last index
        self.entries.swap_remove(id);
    }

    fn delete_entry(&mut self, id: usize) {

        self.entries.remove(id-1);
    }
    
}

impl InMemory {
    
    pub fn new()-> Self{
        InMemory {
            entries: Vec::new()
        }
    }

}