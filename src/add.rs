use std::io::{self, Write};


pub fn read_string(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap(); 

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
