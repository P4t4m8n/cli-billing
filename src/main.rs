use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}
impl Bill {
    fn print(&self) {
        println!("name:{:?}, amount:{:?}", self.name, self.amount)
    }
}

pub struct Bills {
    bills: HashMap<String, Bill>,
}
impl Bills {
    fn new() -> Self {
        Self {
            bills: HashMap::new(),
        }
    }

    fn add(&mut self, bill: Bill) {
        self.bills.insert(bill.name.to_string(), bill);
    }

    fn remove(&mut self, name: &str) -> bool {
        self.bills.remove(name).is_some()
    }

    fn get_all(&self) -> Vec<&Bill> {
        self.bills.values().collect()
    }

    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.bills.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                return true;
            }
            None => return false,
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
}
impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::UpdateBill),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("== Bill Manager ==");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Update Bill");
        println!("");
        println!("Enter selection: ");
    }
}
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Error")
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let amount = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &amount == "" {
            return None;
        }

        let parsed_amount: Result<f64, _> = amount.parse::<f64>();
        match parsed_amount {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Error"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
    }

    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            bill.print();
        }
    }

    pub fn remove_bill(bills: &mut Bills) {
        view_bills(bills);
        println!("Enter name");

        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        if bills.remove(&name) {
            print!("Success")
        } else {
            println!("Fail")
        }
    }

    pub fn update_bill(bills: &mut Bills) {
        view_bills(bills);
        println!("Enter name");

        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };

        if bills.update(&name, amount) {
            print!("Success")
        } else {
            println!("Fail")
        }
    }
}

fn run_program() -> Option<()> {
    let mut bills = Bills::new();
    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            Some(MainMenu::UpdateBill) => menu::update_bill(&mut bills),
            None => break,
        }
    }
    None
}
fn main() {
    run_program();
}
