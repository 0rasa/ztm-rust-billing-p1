use std::io;



pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}


pub fn pause() {
    println!("Press [Enter] to continue the program.");
    let _ = get_input();
}


#[derive(Clone)]
pub struct Item {
    name: String,
    amount: f64
}

impl Item {
    pub fn new(name: String, amount: f64) -> Self {
        Self {
            name, amount
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn print_all(bills: &Vec<Self>) {
        let mut c = 0;

        for i in bills {
            println!("[{:?}] {:?} - {:?}", (c+1), i.name(), i.amount());
            c += 1;
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    fn print(&self) {
        println!("Name: {:?}\nAmount: {:?}", self.name, self.amount);
    }
}


pub fn get_bill_amount() -> io::Result<f64> {
    loop {
        println!("Enter bill amount:");
        let amount_result = get_input()?;

        match amount_result.parse::<f64>() {
            Ok(amount) => break Ok(amount),
            Err(_) => {
                println!("Please correct the amount. Make sure it's a number.");
                continue;
            }
        };

    }
}

pub fn get_bill_name() -> io::Result<String> {
    loop {
        println!("Enter bill name:");
        let name = get_input()?;

        if name.trim().is_empty() {
            println!("The name can not be empty!");
            continue;
        }
        
        break Ok(name)
    }
}

pub fn add_bill(bills: &mut Vec<Item>) -> io::Result<()> {
    let name = get_bill_name()?;
    let amount = get_bill_amount()?;

    let item = Item::new(name, amount);

    println!("----------------------");

    println!("Adding Bill:");
    item.print();
    bills.push(item);
    println!("----------------------");


    pause();

    Ok(())
}

pub fn view_bills(bills: &Vec<Item>) {
    println!("----------------------");

    for i in bills {
        i.print();
        println!("----------------------");
    }

    pause();

}

