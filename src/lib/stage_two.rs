
use crate::stage_one::{Item, get_input, pause};


pub fn get_bill_index(len: usize, msg: &str) -> Option<usize> {
    loop {
        println!("Which Bill you want to {:?} ?", msg);

        let input = match get_input() {
            Ok(v) => v,
            Err(_) => {
                println!("Something went wrong.");
                continue;
            },
        };

        // try to turn user input into i32.
        let index = match input.parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                println!("Make sure you entered a number.");
                continue;
            }
        };


        if index == 0 {
            println!("You can't use negative numbers.");
            continue;
        }

        let is_not_from_list = index - 1;
        if  is_not_from_list > len {
            println!("The chosen number must be from the list.");
            continue;
        }



        break Some(index - 1)
    }
}


pub fn is_there_bills(len: usize, msg: &str) -> bool {
    if len == 0 {
        println!("There's no bills to {:?}.", msg);
        pause();

        return false;
    }

    true
}



// Note: i'm returning empty result to quickly exit from the function,
// in case there's no bills.
pub fn remove_bill(bills: &mut Vec<Item>) -> Result<(), ()> {
    // 1. print all bills names like:
    //      [1] Laptop Bill #1
    //      [2] Car Bill #1
    // 2. ask user "Which bill you want to remove"
    // 3. when user enter number i.e 1
    // 4. subtract 1 from the user input to get the index.
    //    need to make sure that the user entered a number and the number exists in the vec.
    
    let length = bills.len();

    if is_there_bills(length, "remove") == false {
        return Err(())
    }

    Item::print_all(&bills);

    loop {
        let index = get_bill_index(length - 1, "remove").unwrap();
    
        let bill = bills.get(index);
    
        
        if bill.is_some() {
            let bill = bill.unwrap();

            println!("Removing: {:?} - {:?}", bill.name(), bill.amount());

            bills.remove(index);

            pause();


            break;
        } else {
            continue;
        }

    }
    
    Ok(())
}
