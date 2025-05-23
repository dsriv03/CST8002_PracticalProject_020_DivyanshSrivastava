use std::{array, io};

use chrono::NaiveDate;
use tabled::Table;

use crate::{business::{self, crude_runs_dao::CrudeRunsDao}, persistence::{formats::{in_memory::InMemory, sqlitedb, writable::Writable}, model::crude_runs_dto::CrudeRunsDTO}, presentation::sort::sort};

    /// Core display loop of the CLI menu
    ///
    /// # Arguments
    ///
    /// * None
    /// 
    /// # Returns
    /// 
    /// * None
pub fn display_loop(){
    let mut display = true;
    let mut dao_type = true;

    println!("Welcome to my Program!");
    println!("This program was created by Divyansh Srivastava, 041109063");

    let mut memory_dao: CrudeRunsDao<_> = business::crude_runs_dao::CrudeRunsDao { Dao: InMemory::new() };
    let mut sqlite_dao: CrudeRunsDao<_> = business::crude_runs_dao::CrudeRunsDao { Dao: sqlitedb::SqliteDB::new() };

    //call load_all once as an init function and delegate actual loading to a different method, using field vector
    for entry in memory_dao.load_all() {
        entry.to_string();
    }

    let mut entry_table = Table::new(memory_dao.load_all());
    println!("{}", entry_table);

    display_options();

    while display == true {

        println!("Please choose an option.");
            match take_input().as_str() {
                "1" => {
                    let list = match dao_type {
                        true => {memory_dao.get_runs()},
                        false => {sqlite_dao.get_runs()}
                    };
                    println!("{}", Table::new(list))
                }
                ,
                "2" => {
                    match dao_type {
                        true => {memory_dao.persist();},
                        false => {sqlite_dao.persist();}
                    }
                },
                "3" => {
                    let id: u64 = take_input().parse().expect("Couldn't parse as a number");
                    match dao_type {
                        //true => {memory_dao.load_by_id(id.try_into().unwrap()).unwrap().to_string();},
                        true => {memory_dao.load_by_id(id.try_into().unwrap());},
                        false => {sqlite_dao.load_by_id(id.try_into().unwrap()).unwrap().to_string();}
                    }
                },
                "4" => {
                    
                    let new_entry = create_new_item_helper();
                    match dao_type {
                        true => {memory_dao.create_entry(new_entry);},
                        false => {sqlite_dao.create_entry(new_entry);}
                    }
                },
                "5" => {
                    let id: u64 = take_input().parse().expect("Couldn't parse as a number");
                    match dao_type {
                        true => {
                        let new_entry = create_new_item_helper();
                        memory_dao.update_entry(id.try_into().unwrap(), new_entry);
                    },
                        false => {
                        let new_entry = create_new_item_helper();
                        sqlite_dao.update_entry(id.try_into().unwrap(), new_entry);
                    }
                    }
                },
                "6" => {
                    let id: u64 = take_input().parse().expect("Couldn't parse as a number");
                    match dao_type {
                        true => {
                        memory_dao.delete_entry(id.try_into().unwrap());
                    },
                        false => {
                        sqlite_dao.delete_entry(id.try_into().unwrap());
                    }
                    }
                },
                "7" => {
                    dao_type = !dao_type;
                }
                ,
                "8" => {
                    sort(memory_dao.get_runs());
                }
                ,
                "0" => {
                    println!("The program will now exit.");
                    display = false;
                }
                ,
                _ => {println!("Please enter a valid menu item");
                display_options();
            }
        }
        

}

    /// Takes input and creates a new item
    ///
    /// # Arguments
    ///
    /// * None
    /// 
    /// # Returns
    /// 
    /// * CrudeRunsDTO
fn create_new_item_helper() -> CrudeRunsDTO{

    let mut new_item: Vec<String> = Vec::new();
        let mut counter = 0;
        let item_list: [String; 11] = 
        ["id".to_string(),
        "week_end %Y-%m-%d".to_string(),
        "week_end_last_year %Y-%m-%d".to_owned(),
        "region".to_owned(),
        "volume".to_owned(),
        "capacity".to_owned(),
        "four_week_avg".to_owned(),
        "four_week_avg_lastyear".to_owned(),
        "ytd_avg".to_owned(),
        "ytd_avg_lastyear".to_owned(),
        "unit".to_owned()];
        while counter < 11 {
            println!("Enter {}:", item_list[counter]);
            new_item.push(take_input());
            counter += 1;
    }

    let mut entry = CrudeRunsDTO::default();
    entry.set_id(new_item[0].parse::<i64>().unwrap());
    entry.set_weekly_end(NaiveDate::parse_from_str(&new_item[1], "%Y-%m-%d").unwrap());
    entry.set_week_end_last_year(NaiveDate::parse_from_str(&new_item[2], "%Y-%m-%d").unwrap());
    entry.set_region(String::from(&new_item[3]));
    entry.set_volume(new_item[4].parse().unwrap());
    entry.set_capacity(new_item[5].parse().unwrap());
    entry.set_four_week_avg(new_item[6].parse().unwrap());
    entry.set_four_week_avg_lastyear(new_item[6].parse().unwrap());
    entry.set_ytd_avg(new_item[7].parse().unwrap());
    entry.set_ytd_avg_last_year(new_item[8].parse().unwrap());
    entry.set_unit(String::from(&new_item[9]));
    entry
}

    /// Takes input and returns the string
    ///
    /// # Arguments
    ///
    /// * None
    /// 
    /// # Returns
    /// 
    /// * String
pub fn take_input() -> String{
    let mut input_string = String::new();
    input_string.clear();
    io::stdin().read_line(&mut input_string).unwrap().to_string();
    println!("{}", input_string);
    input_string.to_string().trim().to_string()
}

    /// Prints available options
    ///
    /// # Arguments
    ///
    /// * None
    /// 
    /// # Returns
    /// 
    /// * None
pub fn display_options(){
    println!("Available options");
    println!("1 - Display all values");
    println!("2 - Write the extracted data in a new file");
    println!("3 - Load entry by ID");
    println!("4 - Create a new entry");
    println!("5 - Update an existing entry");
    println!("6 - Delete an entry");
    println!("7 - Switch DAO");
    println!("8 - Sort");
    println!("0 - Exit Program");
}
}