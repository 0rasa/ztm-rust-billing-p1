// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - [x] I want to add bills, including the name and amount owed.
//   - [x] I want to view existing bills.
// * Stage 2:
//   - [x] I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use custom_lib::stage_one as so;
use custom_lib::stage_two as st;
use custom_lib::stage_three as sh;




fn main() {
    let menu = ["Add bill", "View Bills", "Remove Bill", "Update Bill", "Bill Total", "Exit"];
    let mut bills: Vec<so::Item> = Vec::new();

    loop {
        println!("== Manage Bills ==");
        for i in 0..6 {
            let option = menu[i];
            println!("{:?}. {}",(i+1), option);
        }
        println!();

        println!("Choose function:");
        let input = so::get_input();
        
        if input.is_ok() {
            match input.unwrap().trim() {
                "1" => {
                    let _ = so::add_bill(&mut bills);

                    continue;
                },

                "2" => so::view_bills(&bills),

                "3" => {
                    let _ = st::remove_bill(&mut bills);

                    continue;
                },

                "4" => {
                    let _ = sh::edit_bill(&mut bills);

                    continue;
                },
    
                "6" => {
                    println!("Exiting ...");
                    break;
                },
                _ => println!("NOT IMPL.")
            }
        } else {
            println!("Something went wrong. Exiting...");
            break;
        }

    }
}
