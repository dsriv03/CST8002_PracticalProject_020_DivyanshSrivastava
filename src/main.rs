/*
    Author: Divyansh Srivastava
    Student Number: 041109063
    Submission: Practical Project 1 for CST 8002_020
    Submission Date: 26/1/2025
*/


mod persistence;
mod business;

use persistence::importer::csv;
use persistence::model::crude_runs_dto::{self, CrudeRunsDTO};
use business::crude_runs_dao;

/// Main - entrypoint of program
fn main() {

    //time benchmark begin
    let start = chrono::Local::now();

    let mut dao = business::crude_runs_dao::CrudeRunsDao::new();
    dao.load_all_runs();
    //time benchmark end
    let end = chrono::Local::now();
    println!("Time taken: {}", start - end); //TODO: replace with a benchmark test

}
