# User Input Practice

This is a project suggestion found while learning Rust [here](https://doc.rust-lang.org/book/ch08-03-hash-maps.html). The purpose of this project is to help learn Rust by building small projects.

## Goal

The goal of the code found here is to create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales”

Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.


## Extra Features

- Remove Keyword: On top of 'Add' a user may use 'Remove' to remove an employee from a department
  - Example: "Remove Sally from Engineering"


## Steps to Run

1. **Installation:**
    - Make sure you have Rust and Cargo installed. If not, you can download them from [rustup.rs](https://rustup.rs/).

2. **Cloning the Repository:**
    ```bash
    git clone https://github.com/MichaelD3289/rust_practice_user_input.git
    ```
3. **Building the Project:**
    ```bash
    cd your-project
    cargo build
    ```

4. **Running the Application:**
    ```bash
    cargo run
    ```
5. **Testing (Optional):**
    There are currently no tests to run.

    To run tests, use:
    ```bash
    cargo test
    ```


## Improvement Ideas
  - Better instructions to user when the program loads
    - `help` command that will give more detailed information on how to use
  - Better error messages detailing where the program went wrong and why
  - allowing a user to list the company they are updating when they start
  - saving the company data to a file that can be retrieved next time the user upates that company
  - list departments and employee seperately as well as grouped
