// use text_io::read;
use std::io::{self, Write};


pub fn read_string(prompt: &str) -> String {
    println!("{prompt}: ");
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Return the string stripped of the trailing newline
    input.trim().to_string()
}

pub fn read_int(prompt: &str) -> i32 {
    loop {
        print!("{prompt}: ");
        io::stdout().flush().unwrap(); 
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");


        match input.trim().parse::<i32>() {
            Ok(num) => return num, 
            Err(_) => println!("Invalid input! Please enter a valid number.\n"),
        }
    }
}

// fn main(){
//     let  a:i32;
//     let  b:i32;
//     println!("Enter two numbers to add using text_io crate");
//     println!("Enter a number");
//     a=read!();
//     println!("Enter another number");
//     b=read!();
//     println!("Sum of {} and {} is {}",a,b,a+b);

//     println!("Enter two numbers to add using std crate");

//     let num1=read_int("Enter a number");
//     let num2=read_int("Enter another number");
//     println!("Sum of {} and {} is {}",num1,num2,num1+num2);



// }