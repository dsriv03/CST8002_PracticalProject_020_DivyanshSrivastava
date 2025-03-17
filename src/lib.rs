mod persistence;
mod business;

#[cfg(test)]
mod tests{

    use crate::{business::{self, crude_runs_dao::{self, CrudeRunsDao}}, persistence::formats::{in_memory::InMemory, sqlitedb::SqliteDB, writable::Writable}};


    #[test]
    fn test_load_all(){
        let mut memory_dao: CrudeRunsDao<InMemory> = business::crude_runs_dao::CrudeRunsDao { Dao: InMemory::new() };
        let mut sqlite_dao: CrudeRunsDao<SqliteDB> = business::crude_runs_dao::CrudeRunsDao { Dao: SqliteDB::new() };
        memory_dao.load_all();
        sqlite_dao.load_all();
        let memory_item = memory_dao.load_by_id(1).unwrap();
        let sqlite_item = sqlite_dao.load_by_id(1).unwrap();
        assert_eq!(memory_item.get_id(), sqlite_item.get_id());
        assert_eq!(memory_item.get_region(), sqlite_item.get_region());
    }
}