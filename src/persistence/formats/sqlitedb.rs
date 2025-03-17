use std::error::Error;
use std::str::FromStr;

use crate::persistence::model::crude_runs_dto::CrudeRunsDTO;
use crate::persistence::importer::csv::import_from_csv;
use crate::persistence::formats::writable;

use uuid::Uuid;

use super::writable::Writable;

pub struct SqliteDB{
    dbcon: rusqlite::Connection,
    entries: Vec<CrudeRunsDTO>

}

impl Writable for SqliteDB{

    /// Handles importing data from a csv file, creating an SQL table and packaging them into a vector
    ///
    /// # Arguments
    ///
    /// * None
    /// 
    /// # Returns
    /// 
    /// * `All Crude Runs vector`
    fn load_all_runs(&mut self) -> &Vec<CrudeRunsDTO>{
    
    self.entries = self.import_to_sql();
    &self.entries
    }

    /// Saves data into a csv using Uuid
    ///
    /// # Arguments
    ///
    /// * None
    /// 
    /// # Returns
    /// 
    /// * `dynamic error object`
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

    /// Loads entry by id
    ///
    /// # Arguments
    ///
    /// * ID
    /// 
    /// # Returns
    /// 
    /// * `Specified crude runs object`
    fn load_by_id(&self, id: usize) -> Option<&CrudeRunsDTO>{
        
        self.entries.get(id)

    }

    /// Creates new crude runs entry
    ///
    /// # Arguments
    ///
    /// * CrudeRunsDTO
    /// 
    /// # Returns
    /// 
    /// * None
    fn create_entry(&mut self, item: CrudeRunsDTO) {

        self.entries.push(item);
    }

    /// Updates a preexisting entry
    ///
    /// # Arguments
    ///
    /// * ID, CrudeRunsDTO
    /// 
    /// # Returns
    /// 
    /// * None
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

impl SqliteDB {
    
    pub fn new()-> Self{

        let conn = rusqlite::Connection::open("resources/output.db").unwrap();
        
        Self::init(&conn);
        SqliteDB {
            dbcon: conn,
            entries: Vec::new()
        }
    }

    pub fn init(dbcon: &rusqlite::Connection){
        dbcon.execute_batch(
            "CREATE TABLE crude_runs (
                id UUID PRIMARY KEY,
                week_end DATE NOT NULL,
                week_end_last_year DATE NOT NULL,
                region VARCHAR(255) NOT NULL,
                volume DOUBLE PRECISION NOT NULL,
                capacity DOUBLE PRECISION NOT NULL,
                four_week_avg DOUBLE PRECISION NOT NULL,
                four_week_avg_lastyear DOUBLE PRECISION NOT NULL,
                ytd_avg DOUBLE PRECISION NOT NULL,
                ytd_avg_lastyear DOUBLE PRECISION NOT NULL,
                unit VARCHAR(50) NOT NULL
                );");
                
    }

    pub fn import_to_sql(&self) -> Vec<CrudeRunsDTO>{
    // Call importer to get the DTO vector with all imported entries
    //TODO: import location from props file 
    let list_of_entries: Vec<CrudeRunsDTO> = 
    import_from_csv(String::from("resources/data.csv"))
    .expect("Could not load list of entries. (This would never trigger and should be handled in csv.rs in the future)");

    //TODO: handle csv error here?
    for entry in &list_of_entries{
        self.dbcon.execute("INSERT INTO crude_runs VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        (entry.get_id(),
        entry.get_week_end(),
        entry.get_week_end_last_year(),
        entry.get_region(),
        entry.get_volume(),
        entry.get_capacity(),
        entry.get_four_week_avg(),
        entry.get_four_week_avg_lastyear(),
        entry.get_ytd_avg(),
        entry.get_ytd_avg_lastyear(),
        entry.get_unit()
        )
        );
    };
    list_of_entries
    }


}