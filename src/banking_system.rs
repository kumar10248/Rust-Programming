use rand::Rng;
mod add;

enum AccountType{
    Savings,
    Current,
}

enum AccountStatus{
    Active,
    Closed,
}

enum TransactionType{
    Deposit,
    TransferIn,
    TransferOut,
    Withdrawal,
}

enum IdentityDocument {
    Aadhaar(String),
    PAN(String),
    Passport(String),
}

enum TransactionStatus{
Success,
Failed,
Pending,
}

enum TransactionChannel {
    ATM,
    UPI,
    NetBanking,
    Branch,
    MobileApp,
}
enum BranchCode{
    RUST0113355,
    RUST0113365,
    RUST0113375,
    RUST0113385,
    RUST0113395,

}

struct Account{
    account_number:u32,
    ifsc_code:BranchCode,
    customer_id:String,
    account_type:AccountType,
    balance_in_paise: u64,
    pin:String,
    account_status:AccountStatus,
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
struct Customer{
    name:String,
    customer_id:String,
    phone:String,
    email:String,
    government_id:IdentityDocument,
    address:String,
    date_of_birth:String,
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
    ifsc_code:BranchCode,
    customer_id:String,
    account_type:AccountType,
    balance_in_paise: u64,
    pin:String,

    )->Self{
        Account{
    account_number,
    ifsc_code,
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

fn read_ifsc_code()->BranchCode{
println!("Select your Branch");

println!("1. Chandigarh University Branch.");
println!("2. Kharar Branch.");
println!("3. Chandigrah Sec-17 Branch.");
println!("4. Chandigrah Sec-43 Branch.");
println!("5. Elante Mall Chandigrah Branch.");

loop{
    let choice=add::read_int("Enter your choice");
    match choice{
        1=>  break BranchCode::RUST0113355,
        
        2=> break BranchCode::RUST0113365,

        3=> break BranchCode::RUST0113375,

        4=> break BranchCode::RUST0113385,

        5=> break BranchCode::RUST0113395,

        
       
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
customers.push(customer);

}

fn open_account(
    customers: &[Customer],
    accounts: &mut Vec<Account>,
){

let customer_id = add::read_string("Enter Customer ID");

let index = get_customer_index(customers, &customer_id);
match index{
    Some(_index)=>{
   let account_number=generate_account_no(accounts);
   let account_type=read_account_type();
   let pin=add::read_int("Enter your 4-digit PIN Number");
   let ifsc_code=read_ifsc_code();
let initial_balance = 0;
   let account=Account::new(
  account_number,
    ifsc_code,
    customer_id,
    account_type,
    initial_balance,
    pin.to_string(),
   );
   accounts.push(account);

    }

    None=> println!("Customer not found"),
}

}



//Main Function
fn main(){
    println!("Welcome To RUST Banking System");

    let mut customers=Vec::<Customer>::new();
    let mut accounts=Vec::<Account>::new();

    loop{
        println!("1. Create Customer");
        println!("1. Open Account");
        println!("9. Exit");

        let choice=add::read_int("Enter your choice");

        match choice{
            1=>open_customer(&mut customers),

            2=>open_account(&customers,&mut accounts ),







            9=> break,
            _ => println!("Please Enter Valid Choice"),
            
        }
    }

}