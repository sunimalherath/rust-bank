#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
}

impl Account {
    fn new(id: u32, holder: String) -> Account {
        Account {
            id,
            holder,
            balance: 0
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank{
            accounts: vec![]
        }
    }

    // taking the ownership of the 'account' that passed into this method
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("John Doe"));

    bank.add_account(account);

    println!("{:#?}", bank);
}
