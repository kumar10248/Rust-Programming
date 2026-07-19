fn main() {
    // let name="Kumar Devashish";
    // let age=12;
    // println!("My name is {} and age is {}",name,age);

    // println!("Hello, world!");
    // let mut score:i32=100;
    
    //arithmetic operator

    // score=138;
    // println!("{score}");
    // let a:i16=38;
    // let b:i16=34;
    // println!("{}",a+b);
    // println!("{}",a-b);
    // println!("{}",a*b);
    // println!("{}",a/b);
    // println!("{}",a%b);

    //datatypes

    // let mut name = String::from("Kumar");
    // name.push_str(" Devashish");
    // let age: u8 = 22;
    // let cgpa: f32 = 7.9;
    // let passed = true;
    // let grade = 'A';
    // let arr=[1,2,3,4];

    // println!("ages: {age}");
    // println!("cgpas: {cgpa}");
    // println!("passeds: {passed}");
    // println!("grades: {grade}");
    // println!("Name: {name}");
    // println!("{:?}",arr);

//shadowing
// let x=5;
// let x=x+10;
// println!("{x}");

//Ownership

//     let a = "Hello";
//     let b = a;

//     println!("{b}");
//     println!("{a}");

//     let x = 100;
// let y = x;

// println!("{x}");
// println!("{y}");

// let s1 = String::from("Rust");
// let s2 = s1;

// println!("{s1}");
// println!("{s2}");

//  ownership in function

// let s = String::from("Rust");

//     print_name(s);
//     println!("{s}");

// borrowing(reference) in function
let s = String::from("Rust");

    print_name(&s);
    println!("{s}");



}


// fn print_name(name: String) {
//     println!("{name}");
// }


fn print_name(name: &String) {
    println!("{name}");
}




