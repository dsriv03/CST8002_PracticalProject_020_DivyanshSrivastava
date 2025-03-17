use std::error::Error;
use crate::persistence::model::crude_runs_dto::CrudeRunsDTO;

pub trait Writable {
    fn load_all_runs(&mut self) -> &Vec<CrudeRunsDTO>;
    fn persist(&self) -> Result<(), Box<dyn Error>>;
    fn load_by_id(&self, id: usize) -> Option<&CrudeRunsDTO>;
    fn create_entry(&mut self, item: CrudeRunsDTO);
    fn update_entry(&mut self, id: usize, item: CrudeRunsDTO);
    fn delete_entry(&mut self, id: usize);

}