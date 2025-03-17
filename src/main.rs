/*
    Author: Divyansh Srivastava
    Student Number: 041109063
    Submission: Practical Project 1 for CST 8002_020
    Submission Date: 26/1/2025
*/

mod persistence;
mod business;
mod presentation;

use persistence::formats;
use persistence::importer::csv;
use presentation::menu;

/// Main - entrypoint of program
fn main() {

    //time benchmark begin
    let start = chrono::Local::now();

    menu::display_loop();

    //time benchmark end)
    let end = chrono::Local::now();
    println!("Time used: {}", start - end); //TODO: replace with a benchmark test

}
