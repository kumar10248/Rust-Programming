use rand::Rng;
mod add;
fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

fn guess_number(){
    let num=generate_random_number();
    println!("Random Number: {}", num);
    let mut count=0;
    loop{
      let gn=add::read_int("Enter Guess Number");
      count+=1;
       if gn==num{
         println!("");
         println!("Congratulations! You have guessed it Right in {count} Attempts");
         println!("");
         println!("*******************************************************************************");
         println!("");
     
break;
       }
 
       
           else if gn>num {
                println!("Too High, Please try lower Number");
            }
            else{
                 println!("Too Low, Please try Higher Number");
            }
        
      
    }
}

fn main(){
println!("Welcome to the NUMBER GUESSING GAME!\n");

    loop {
        guess_number();

        println!("Play Again?");
        println!("Press 1 for Yes");
        println!("Press 0 for No");

        let choice = add::read_int("Please enter your choice");
        if choice != 1 {
            println!("Thanks for playing the Number Guessing Game!");
            break;
        }
    }
}