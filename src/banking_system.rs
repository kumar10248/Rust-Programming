use rand::Rng;
mod add;
use std::fs::File;
use std::io::{BufRead, Write};

#[derive(Debug)]
enum AccountType{
    Savings,
    Current,
}
#[derive(Debug)]
enum AccountStatus{
    Active,
    Closed,
}
#[derive(Debug)]
enum TransactionType{
    Deposit,
    TransferIn,
    TransferOut,
    Withdrawal,
}
#[derive(Debug)]
enum IdentityDocument {
    Aadhaar(String),
    PAN(String),
    Passport(String),
}
#[derive(Debug)]
enum TransactionStatus{
Success,
Failed,
Pending,
}

#[derive(Debug)]
enum TransactionChannel {
    ATM,
    UPI,
    NetBanking,
    Branch,
    MobileApp,
}

#[derive(Debug)]
enum Branch {
    ChandigarhUniversity,
    Kharar,
    ChandigarhSec17,
    ChandigarhSec43,
    ElanteMall,
}

struct Account{
    account_number:u32,
    branch:Branch,
    customer_id:String,
    account_type:AccountType,
    balance_in_paise: u64,
    pin:String,
    account_status:AccountStatus,
}

struct Customer{
    name:String,
    customer_id:String,
    phone:String,
    email:String,
    government_id:IdentityDocument,
    address:String,
    date_of_birth:String,
}


struct Transaction{
    transaction_id:u64,
    from_account:u32,
    to_account:Option<u32>,
    transaction_type:TransactionType,
    amount:u64,
    channel:TransactionChannel,
    date_time:String,
    status:TransactionStatus,

}

impl Transaction {
    fn display(&self) {
        let to_account = self
            .to_account
            .map(|acc| acc.to_string())
            .unwrap_or("None".to_string());

        let amount_rupees = self.amount / 100;

        println!(
            "{:<15} {:<12} {:<12} {:?} ₹{:<8} {:?} {:<25} {:?}",
            self.transaction_id,
            self.from_account,
            to_account,
            self.transaction_type,
            amount_rupees,
            self.channel,
            self.date_time,
            self.status,
        );
    }
}

impl Customer {
    fn display(&self) {
                println!(
            "{:<10} {:<30} {:<30} {:<12}   {:<50}   {:?} {:<10}", 
            self.customer_id, 
            self.name,
            self.email, 
            self.phone,
           self.address,
           self.government_id,
          self.date_of_birth,
        );

    }
}

impl Account {
    fn display(&self) {
        let balance_in_rupee = self.balance_in_paise / 100;

        println!(
            "{:<10} {:<30} {:<15} {:?} ₹{} {:?}",
            self.account_number,
            self.branch.name(),
            self.customer_id,
            self.account_type,
            balance_in_rupee,
            self.account_status,
        );
    }
}


fn save_customers(customers: &[Customer]) -> std::io::Result<()> {
    let mut file = File::create("data/customers.txt")?;

    for customer in customers {
        writeln!(
            file, 
            "{},{},{},{},{},{:?},{}", 
            customer.customer_id, 
            customer.name,
            customer.email, 
            customer.phone,
           customer.address,
           customer.government_id,
          customer.date_of_birth,
        )?;
    }

    Ok(())
}

fn save_accounts(accounts: &[Account]) -> std::io::Result<()> {
    let mut file = File::create("data/accounts.txt")?;

    for account in accounts {
        writeln!(
            file, 
            "{},{},{},{:?},{},{:?}",
            account.account_number,
            account.branch.name(),
            account.customer_id,
            account.account_type,
            account.balance_in_paise,
            account.account_status,
        )?;
    }

    Ok(())
}

fn save_transactions(transactions: &[Transaction]) -> std::io::Result<()> {
    let mut file = File::create("data/transactions.txt")?;

    for transaction in transactions {
        let to_account = transaction
            .to_account
            .map(|acc| acc.to_string())
            .unwrap_or("None".to_string());

        writeln!(
            file, 
           "{},{},{},{:?},{},{:?},{},{:?}",
            transaction.transaction_id,
            transaction.from_account,
            to_account,
            transaction.transaction_type,
            transaction.amount,
            transaction.channel,
            transaction.date_time,
            transaction.status,
        )?;
    }

    Ok(())
}


//it can be use for both cutomer ID and Account number generation
fn generate_random_8digit() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10_000_000..=99_999_999)
}

fn generate_customer_id(customers:&[Customer])->String{
 loop {
        let rn = generate_random_8digit();
        let candidate_id = format!("CUST{}", rn);

        // Check if candidate_id already belongs to an existing customer
        let is_duplicate = customers.iter().any(|c| c.customer_id == candidate_id);

        if !is_duplicate {
            return candidate_id ;// Unique ID found!
        }
    }
}

pub fn generate_account_no(accounts: &[Account]) -> u32 {
    loop {
        let candidate_no = generate_random_8digit();
        if !accounts.iter().any(|a| a.account_number == candidate_no) {
            return candidate_no;
        }
    }
}

fn generate_transaction_id(transactions: &[Transaction]) -> u64 {
    let mut rng = rand::thread_rng();
    loop {
        let candidate_id: u64 = rng.gen_range(100_000_000_000..=999_999_999_999);
        if !transactions.iter().any(|t| t.transaction_id == candidate_id) {
            return candidate_id;
        }
    }
}

impl Branch {
    fn ifsc_code(&self) -> &'static str {
        match self {
            Branch::ChandigarhUniversity => "RUST0113355",
            Branch::Kharar => "RUST0113365",
            Branch::ChandigarhSec17 => "RUST0113375",
            Branch::ChandigarhSec43 => "RUST0113385",
            Branch::ElanteMall => "RUST0113395",
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Branch::ChandigarhUniversity => "Chandigarh University",
            Branch::Kharar => "Kharar",
            Branch::ChandigarhSec17 => "Chandigarh Sector 17",
            Branch::ChandigarhSec43 => "Chandigarh Sector 43",
            Branch::ElanteMall => "Elante Mall",
        }
    }
}

impl Customer{
    
    pub fn new( 
        name:String,
        customer_id:String,
         phone:String, 
         email:String, 
          government_id:IdentityDocument,
          address:String,
          date_of_birth:String,
        )->Self{
   Customer{
     name,
     customer_id,
     phone,
     email,
     government_id,
     address,
     date_of_birth,
   }
    }
}

impl Account{
    pub fn new(
    account_number:u32,
    branch:Branch,
    customer_id:String,
    account_type:AccountType,
    balance_in_paise: u64,
    pin:String,

    )->Self{
        Account{
    account_number,
    branch,
    customer_id,
    account_type,
    balance_in_paise,
    pin,
    account_status:AccountStatus::Active,
        }
    }
}

fn get_customer_index(customers:&[Customer], customer_id: &str)->Option<usize>{
    for (index, customer) in customers.iter().enumerate() {
        if customer.customer_id == customer_id {
            return Some(index);
        }
    }
    None
}



fn read_identity_document()->IdentityDocument{
println!("select government ID proof");

println!("1. Adhhar Card No.");
println!("2. PAN Card No.");
println!("3. Passport  No.");
loop{
    let choice=add::read_int("Enter your choice");
    match choice{
        1=>{
           let number= add::read_string("Enter Aadhaar Number");
            break IdentityDocument::Aadhaar(number);
        }
        2=>{
           let number= add::read_string("Enter PAN Number");
            break IdentityDocument::PAN(number);
        }
        3=>{
           let number= add::read_string("Enter Passport Number");
            break IdentityDocument::Passport(number);
        }
        _=> println!("Please Enter Valid Choice"),
    }

}


}




fn read_account_type()->AccountType{
println!("Select Account type");

println!("1. Savings Account.");
println!("2. Current Account.");
loop{
    let choice=add::read_int("Enter your choice");
    match choice{
        1=>{
           
            break AccountType::Savings;
        }
        2=>{
            break AccountType::Current;
        }
       
        _=> println!("Please Enter Valid Choice"),
    }

}

}

fn read_branch()->Branch{
println!("Select your Branch");

println!("1. Chandigarh University Branch.");
println!("2. Kharar Branch.");
println!("3. Chandigrah Sec-17 Branch.");
println!("4. Chandigrah Sec-43 Branch.");
println!("5. Elante Mall Chandigrah Branch.");

loop{
    let choice=add::read_int("Enter your choice");
    match choice{
        1=>  break Branch::ChandigarhUniversity,
        
        2=> break Branch::Kharar,

        3=> break Branch::ChandigarhSec17,

        4=> break Branch::ChandigarhSec43,

        5=> break Branch::ElanteMall,

        
       
        _=> println!("Please Enter Valid Choice"),
    }

}

}

fn read_transaction_type()->TransactionType{
println!("Select Transaction Type");

println!("1. Deposit");
println!("2. Withdrawal");
println!("3. TransferOut");
println!("4. TransferIn");

loop{
    let choice=add::read_int("Enter your choice");
    match choice{
        1=>  break TransactionType::Deposit,
        
        2=> break TransactionType::Withdrawal,

        3=> break TransactionType::TransferOut,

        4=> break TransactionType::TransferIn,

        
       
        _=> println!("Please Enter Valid Choice"),
    }

}

}

fn read_transaction_channel()->TransactionChannel{
println!("Select the Transaction Option");

println!("1. UPI");
println!("2. ATM.");
println!("3. NetBanking");
println!("4. Branch");
println!("5. Mobile App");

loop{
    let choice=add::read_int("Enter your choice");
    match choice{
        1=>  break TransactionChannel::UPI,
        
        2=> break TransactionChannel::ATM,

        3=> break TransactionChannel::NetBanking,

        4=> break TransactionChannel::Branch,

        5=> break TransactionChannel::MobileApp,

        
       
        _=> println!("Please Enter Valid Choice"),
    }

}

}

fn open_customer(customers:&mut Vec<Customer> ){
let customer_id=generate_customer_id(customers);
let name=add::read_string("Enter Your Name");
let phone=add::read_string("Enter Your Phone Number");
let email=add::read_string("Enter Your Email ID");
let government_id=read_identity_document();
let address=add::read_string("Enter Your Address");
let date_of_birth=add::read_string("Enter Your Date Of Birth");

let customer=Customer::new(
    name,
    customer_id,
    phone,
    email,
    government_id,
    address,
    date_of_birth,
);
println!("================================");
println!("Customer Created Successfully!");
println!("Customer ID: {}", customer.customer_id);
println!("================================");
customers.push(customer);

}

fn open_account(
    customers: &[Customer],
    accounts: &mut Vec<Account>,
    transactions: &mut Vec<Transaction>,
) {
    let customer_id = add::read_string("Enter Customer ID");

    match get_customer_index(customers, &customer_id) {
        Some(_index) => {
            let account_type = read_account_type();
            let branch = read_branch();
            let pin = add::read_int("Enter your 4-digit PIN Number");
            let initial_rupees: u64 = add::read_int("Please Enter Amount (Minimum Required: Rs. 100)") as u64;

            if initial_rupees < 100 {
                println!("Error: Minimum deposit is Rs. 100.");
                return;
            }

            let account_number = generate_account_no(accounts);

            // 1. Create account with 0 initial balance
            let account = Account::new(
                account_number,
                branch,
                customer_id,
                account_type,
                0, // Starts at 0
                pin.to_string(),
            );
            accounts.push(account);

            // 2. Deposit the initial funds via the transaction engine!
            let amount_in_paise = initial_rupees * 100;
            match execute_transaction(
                accounts,
                transactions,
                account_number,
                None,
                TransactionType::Deposit,
                amount_in_paise,
                TransactionChannel::Branch,
            ) {
                Ok(_) => {
                    println!("Account created successfully! Account No: {}", account_number);
                    println!("Initial deposit of Rs. {} processed via transaction engine.", initial_rupees);
                }
                Err(err) => println!("Failed to process initial deposit: {}", err),
            }
        }
        None => println!("Customer not found"),
    }
}


 fn execute_transaction(
    accounts: &mut Vec<Account>,
    transactions: &mut Vec<Transaction>,
    from_acc_no: u32,
    to_acc_no: Option<u32>,
    tx_type: TransactionType,
    amount_in_paise: u64,
    channel: TransactionChannel,
) -> Result<(), String> {
    // 1. Locate source account
    let from_index = accounts
        .iter()
        .position(|a| a.account_number == from_acc_no)
        .ok_or_else(|| String::from("Source account not found!"))?;

    // 2. Perform financial operations based on TransactionType
    match tx_type {
        TransactionType::Deposit => {
            accounts[from_index].balance_in_paise += amount_in_paise;
        }
        TransactionType::Withdrawal => {
            if accounts[from_index].balance_in_paise < amount_in_paise {
                return Err(String::from("Insufficient balance!"));
            }
            accounts[from_index].balance_in_paise -= amount_in_paise;
        }
        TransactionType::TransferOut => {
            if accounts[from_index].balance_in_paise < amount_in_paise {
                return Err(String::from("Insufficient balance!"));
            }
            let target_no = to_acc_no.ok_or_else(|| String::from("Target account required for transfer"))?;
            
            let to_index = accounts
                .iter()
                .position(|a| a.account_number == target_no)
                .ok_or_else(|| String::from("Target account not found!"))?;

            accounts[from_index].balance_in_paise -= amount_in_paise;
            accounts[to_index].balance_in_paise += amount_in_paise;
        }
        TransactionType::TransferIn => {
            return Err(String::from("TransferIn cannot be initiated directly."));
        }
    }

    // 3. Record transaction in ledger
    let tx = Transaction {
        transaction_id: generate_transaction_id(transactions),
        from_account: from_acc_no,
        to_account: to_acc_no,
        transaction_type: tx_type,
        amount: amount_in_paise,
        channel,
        date_time: "2026-07-29 10:00:00".to_string(),
        status: TransactionStatus::Success,
    };

    transactions.push(tx);
    Ok(())
}

 fn perform_transaction(
    accounts: &mut Vec<Account>,
    transactions: &mut Vec<Transaction>,
) -> Result<(), String> {
    let from_acc_no: u32 = add::read_int("Enter Account Number") as u32;

    // Check PIN
    let from_index = accounts
        .iter()
        .position(|a| a.account_number == from_acc_no)
        .ok_or_else(|| String::from("Account not found!"))?;

    let entered_pin = add::read_string("Enter 4-digit PIN");
    if accounts[from_index].pin != entered_pin {
        return Err(String::from("Invalid PIN!"));
    }

    let tx_type = read_transaction_type();
    let channel = read_transaction_channel();
    let amount_rupees: u64 = add::read_int("Enter Amount in Rupees") as u64;
    let amount_in_paise = amount_rupees * 100;

    let mut to_acc_no: Option<u32> = None;

    if let TransactionType::TransferOut = tx_type {
        let target_no: u32 = add::read_int("Enter Target Account Number")as u32;
        to_acc_no = Some(target_no);
    }

    // Delegate execution
    execute_transaction(
        accounts,
        transactions,
        from_acc_no,
        to_acc_no,
        tx_type,
        amount_in_paise,
        channel,
    )?;

    println!("Transaction completed successfully!");
    Ok(())
}

// Helper function to handle saving everything
fn save_all_data(
    customers: &[Customer],
    accounts: &[Account],
    transactions: &[Transaction],
) -> Result<(), std::io::Error> {
    println!("Saving Customers Details to file...");
    save_customers(customers)?;
    println!("Customers Details saved successfully.");

    println!("Saving Accounts Details to file...");
    save_accounts(accounts)?;
    println!("Accounts Details saved successfully.");

    println!("Saving Transactions Details to file...");
    save_transactions(transactions)?;
    println!("Transactions Details saved successfully.");

    Ok(())
}

//Main Function
fn main(){
    println!("Welcome To RUST Banking System");

    let mut customers=Vec::<Customer>::new();
    let mut accounts=Vec::<Account>::new();
    let mut transactions=Vec::<Transaction>::new();


    loop{
        println!("1. Create Customer");
        println!("2. Open Account");
        println!("3. Make Transaction");
        println!("4. View Customer");
        println!("5. View Customer Account Details");
        println!("6. View Transactions Details");
        println!("9. Exit");

        let choice=add::read_int("Enter your choice");

        match choice{
            1=>open_customer(&mut customers),

            2=>open_account(&customers,&mut accounts,&mut transactions ),

            3=>{
                let transaction=perform_transaction(&mut accounts, &mut transactions);
                match transaction{
                    Ok(())=>println!("Transaction successfully"),
                   Err(err)=>println!("{err}"),
                }
            }
           4=>{
               println!("{}", "-".repeat(180));
                println!("Customer ID        Name                         Email ID               Phone Number            Address         Government ID        DOB");
                for customer in &customers{
                    customer.display();
                }
                println!("{}", "-".repeat(180));

            }

             5=>{
                println!("{}", "-".repeat(180));
                println!("Account Number      Branch Name                         Customer ID               Account Type            Balance         Account Status");
                for account in &accounts{
                    account.display();
                }
                println!("{}", "-".repeat(180));

            }

             6=>{
               println!("{}", "-".repeat(200));
                println!("Transaction ID            From Account               To Account              Transaction Type           Amount            Channel         Date & Time        status");
                for transaction in &transactions{
                    transaction.display();
                }
                println!("{}", "-".repeat(200));

            }
           

                        9 => {
                if let Err(err) = save_all_data(&customers, &accounts, &transactions) {
                    println!("Error while saving system data: {}", err);
                }
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Please Enter Valid Choice"),
            
        }
    }

}