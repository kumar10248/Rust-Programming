

fn main(){
    let add = |a: i32, b: i32| -> i32 {
    a + b
};

println!("{}", add(10, 20));
    let add=|a,b| a*b;
    println!("{}",add(5,6));

    let square=|a| a*a;
    println!("{}",square(5));

    let greet=||println!("Hello Rust!");
    greet();

   let nums = vec![10,50,40,90];

let max =
    nums.iter()
        .fold(i32::MIN, |acc, x| acc.max(*x));

println!("{}", max);

}