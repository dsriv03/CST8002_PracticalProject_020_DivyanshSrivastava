use std::io;

use chrono::NaiveDate;

use crate::{business::crude_runs_dao::CrudeRunsDao, persistence::model::crude_runs_dto::CrudeRunsDTO};



pub fn display_loop(){
    let mut display = true;
    let mut dao: CrudeRunsDao = CrudeRunsDao::new();

    for entry in dao.load_all_runs() {
        entry.to_string();
    }

    println!("Welcome to my Program!");
    println!("This program was created by Divyansh Srivastava, 041109063");

    println!("Available options");
    println!("1 - Display all values");
    println!("2 - Write the extracted data in a new file");
    println!("3 - Load entry by ID");
    println!("4 - Create a new entry");
    println!("5 - Update an existing entry");
    println!("6 - Delete an entry");
    println!("0 - Exit Program");

    while display == true {

        println!("Please choose an option.");

        let mut input_string = String::new();

            input_string.clear();
            io::stdin().read_line(&mut input_string).unwrap();
            match input_string.trim() {
                "1" => {
                    for entry in dao.load_all_runs() {
                        entry.to_string();
                    }
                }
                ,
                "2" => {
                    dao.write_to_csv();
                },
                "3" => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let id: u64 = input.trim().parse().expect("Couldn't parse as a number");
                    dao.load_by_id(id.try_into().unwrap()).unwrap().to_string();
                },
                "4" => {
                    
                    let new_entry = create_new_item_helper();
                    dao.create_entry(new_entry);
                },
                "5" => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let id: u64 = input.trim().parse().expect("Couldn't parse as a number");
                    if dao.load_by_id(id.try_into().unwrap()).is_none(){
                        println!("Entry does not exist");
                        return;
                    }
                    let new_entry = create_new_item_helper();
                    dao.update_entry(id.try_into().unwrap(), new_entry);
                },
                "6" => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let id: u64 = input.trim().parse().expect("Couldn't parse as a number");
                    if dao.load_by_id(id.try_into().unwrap()).is_none(){
                        println!("Entry does not exist");
                        return;
                    }
                    dao.delete_entry(id.try_into().unwrap());
                },
                "0" => {
                    println!("The program will now exit.");
                    display = false;
            }
                ,
                _ => println!("Please enter a valid menu item"),
            }
        }
        

}


fn create_new_item_helper() -> CrudeRunsDTO{

    let mut new_item: Vec<String> = Vec::new();
        let counter = 0;
        while counter < 11 {
            println!("Enter data:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            new_item.push(input);
    }

    let mut entry = CrudeRunsDTO::default();
    entry.set_id(new_item[0].parse().unwrap());
    entry.set_weekly_end(NaiveDate::parse_from_str(&new_item[1], "%m/%d/%Y").unwrap());
    entry.set_week_end_last_year(NaiveDate::parse_from_str(&new_item[2], "%m/%d/%Y").unwrap());
    entry.set_region(String::from(&new_item[3]));
    entry.set_volume(new_item[4].parse().unwrap());
    entry.set_capacity(new_item[5].parse().unwrap());
    entry.set_four_week_avg(new_item[6].parse().unwrap());
    entry.set_four_week_avg_lastyear(new_item[6].parse().unwrap());
    entry.set_ytd_avg(new_item[7].parse().unwrap());
    entry.set_ytd_avg_last_year(new_item[8].parse().unwrap());
    entry.set_unit(String::from(&new_item[3]));
    entry
}