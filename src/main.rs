/*
    Author: Divyansh Srivastava
    Student Number: 041109063
    Submission: Practical Project 1 for CST 8002_020
    Submission Date: 26/1/2025
*/


mod persistence;

use persistence::importer::csv;
use persistence::model::crude_runs_dto;


/// Main - entrypoint of program
fn main() {

    //time benchmark begin
    let start = chrono::Local::now();

    // Call importer to get the DTO vector with all imported entries
    //TODO: import location from props file 
    let list_of_entries: Vec<crude_runs_dto::CrudeRunsDTO> = 
    csv::import_from_csv(String::from("resources/data.csv")).expect("Could not load list of entries. (This would never trigger and should be handled in csv.rs in the future)");

    // Iteratore over the DTO vector and call to string on each
    for mut entry in list_of_entries{
        entry.to_string();
        println!("Practical Project 1 by Divyansh Srivastava, 041109063.")
    }

    //time benchmark end
    let end = chrono::Local::now();
    println!("Time taken: {}", start - end); //TODO: replace with a benchmark test

}
