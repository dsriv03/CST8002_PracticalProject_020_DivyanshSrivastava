use std::error::Error;

use crate::persistence::{formats::writable::{self, Writable}, model::crude_runs_dto::CrudeRunsDTO};

pub struct CrudeRunsDao<Dao: Writable>{
    pub Dao: Dao
}

impl<Dao: Writable> CrudeRunsDao<Dao> {

    pub fn load_all(&mut self) -> &Vec<CrudeRunsDTO>{
        self.Dao.load_all_runs()
    }

    pub fn persist(&self) -> Result<(), Box<dyn Error>>{
        self.Dao.persist()
    }

    pub fn load_by_id(&self, id: usize) -> Option<&CrudeRunsDTO>{
        self.Dao.load_by_id(id)
    }

    pub fn create_entry(&mut self, item: CrudeRunsDTO){
        self.Dao.create_entry(item)
    }

    pub fn update_entry(&mut self, id: usize, item: CrudeRunsDTO){
        self.Dao.update_entry(id, item)
    }

    pub fn delete_entry(&mut self, id: usize){
        self.Dao.delete_entry(id);
    }

    pub fn get_runs(&mut self) -> Vec<CrudeRunsDTO> {
        self.Dao.get_runs().clone()
    }
}