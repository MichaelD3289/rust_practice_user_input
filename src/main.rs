use employee_departments::{company::Company, updater::Updater};
use std::io;

/// Using a hash map and vectors, create a text interface to allow a user to add
/// employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
/// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn main() {
    let mut company = Company::new("ABC");

    loop {
        let command = get_user_input();

        if command.trim() == String::from("exit") {
            break;
        }

        if command.trim() == "list".to_string() {
            company.list_departments();
            continue;
        }

        let updater = match Updater::from_str(&command.trim()) {
            Some(u) => u,
            _ => {
                println!("Bad Input. Please use 'Action Name to/from Department' Ex 'Add Sally to Sales'");
                continue;
            }
        };

        match updater.update(&mut company) {
            Ok(()) => println!("Updated successfully"),
            Err(()) => println!("Error Updating Employee"),
        };
    }
}

fn get_user_input() -> String {
    println!("\nPlease input a command.");

    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    command
}

// Improvement Ideas
//
// More helpful error messaging for update failures
// Ex Not Found: Department 'Sales' or Not Found: Employee 'Jim'
//
// Handle bad string formats without a panic which currently happens when trying to index the vector creating from the input string
