use std::io;
use tabled::Table;

use crate::persistence::model::crude_runs_dto::CrudeRunsDTO;

    /// Sorts input vector based on user input
    ///
    /// # Arguments
    ///
    /// * CrudeRunsDTO vector
    /// 
pub fn sort(mut list: Vec<CrudeRunsDTO>) {
    println!("Select options to sort by (comma-separated, e.g. '1,2'):");
    println!("Press 1 to sort by region.");
    println!("Press 2 to sort by four week average.");
    println!("Press 3 to sort by capacity.");
    println!("Press 0 or any other option to return to main program.");

    let input = take_input();

    let selected_options: Vec<u8> = input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    list.sort_by(|a, b| {
        let mut cmp = std::cmp::Ordering::Equal;

        for option in &selected_options {
            cmp = match option {
                1 => a.get_region().cmp(&b.get_region()), // Now sorts by region for option 1
                2 => a.get_four_week_avg().partial_cmp(&b.get_four_week_avg()).unwrap_or(cmp),
                3 => a.get_capacity().partial_cmp(&b.get_capacity()).unwrap_or(cmp), // Now sorts by capacity for option 3
                _ => cmp,
            };

            // stop comparison if first sort is unequal
            if cmp != std::cmp::Ordering::Equal {
                break;
            }
        }

        cmp
    });

    println!("{}", Table::new(&list));
}

    /// Takes input, sanitizes it and returns it as a string
    ///
    /// # Returns
    /// 
    /// * String
pub fn take_input() -> String {
    let mut input_string = String::new();
    input_string.clear();
    io::stdin().read_line(&mut input_string).unwrap();
    input_string.trim().to_string() 
}
