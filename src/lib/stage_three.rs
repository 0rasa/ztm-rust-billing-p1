use std::io;

use crate::{stage_one::{get_bill_amount, get_bill_name, get_input, pause, Item}, stage_two::{get_bill_index, is_there_bills}};



pub fn edit_bill(bills: &mut Vec<Item>) -> io::Result<()> {
    let length = bills.len();

    if is_there_bills(length, "edit") == false {
        return Ok(())
    }

    Item::print_all(&bills);


    loop {
        let index = get_bill_index(length - 1, "edit").unwrap();
    
        let bill = bills.get_mut(index);

        if bill.is_some() {
            let bill = bill.unwrap();

            let name = get_bill_name()?;
            let amount = get_bill_amount()?;

            println!("Are you sure you want to edit name and amount (Y/n)?");
            let answer = get_input()?;

            if answer.to_lowercase().as_str() == "n" {
                println!("Going back to main menu ...");
                pause();

                break Ok(())
            }

            let old_name = bill.name();
            let old_amount = bill.amount();
            
            println!("Updating The bill from name {:?} and amount {:?} to {:?} {:?}", old_name, old_amount, name, amount);
            
            bill.set_name(name);
            bill.set_amount(amount);
            
            println!("Updated.");

            pause();
            
            break Ok(())
        } else {
            continue;
        }
    }


}
