fn main() {
    // =====================================================
    // Variables and Printing
    // =====================================================

    // let name = "Kumar Devashish";
    // let age = 12;
    //
    // println!("My name is {} and age is {}", name, age);
    // println!("Hello, world!");

    // =====================================================
    // Mutability
    // =====================================================

    // let mut score: i32 = 100;
    // score = 138;
    //
    // println!("{score}");

    // =====================================================
    // Arithmetic Operators
    // =====================================================

    // let a: i16 = 38;
    // let b: i16 = 34;
    //
    // println!("{}", a + b);
    // println!("{}", a - b);
    // println!("{}", a * b);
    // println!("{}", a / b);
    // println!("{}", a % b);

    // =====================================================
    // Data Types
    // =====================================================

    // let mut name = String::from("Kumar");
    // name.push_str(" Devashish");
    //
    // let age: u8 = 22;
    // let cgpa: f32 = 7.9;
    // let passed = true;
    // let grade = 'A';
    // let arr = [1, 2, 3, 4];
    //
    // println!("Age: {age}");
    // println!("CGPA: {cgpa}");
    // println!("Passed: {passed}");
    // println!("Grade: {grade}");
    // println!("Name: {name}");
    // println!("{:?}", arr);

    // =====================================================
    // Shadowing
    // =====================================================

    // let x = 5;
    // let x = x + 10;
    //
    // println!("{x}");

    // =====================================================
    // Ownership
    // =====================================================

    // let a = "Hello";
    // let b = a;
    //
    // println!("{b}");
    // println!("{a}");

    // let x = 100;
    // let y = x;
    //
    // println!("{x}");
    // println!("{y}");

    // let s1 = String::from("Rust");
    // let s2 = s1;
    //
    // println!("{s1}");
    // println!("{s2}");

    // =====================================================
    // Ownership in Function
    // =====================================================

    // let s = String::from("Rust");
    //
    // print_name(s);
    // println!("{s}");

    // =====================================================
    // Immutable Borrowing (References)
    // =====================================================

    // let s = String::from("Rust");
    //
    // print_name(&s);
    // println!("{s}");

    // =====================================================
    // Mutable Borrowing
    // =====================================================

    // let mut s = String::from("Rust");

    // let r1 = &mut s;

    // r1.push_str(" Programming");

    // println!("{r1}");
    // println!("{r1}");

    // println!("{s}");
    // println!("{s}");

    // // =====================================================
    // // Multiple Mutable Borrows (Non-overlapping)
    // // =====================================================

    // let mut s = String::from("Rust");

    // let r1 = &mut s;

    // r1.push_str(" Programming");

    // println!("{r1}");

    // let r2 = &mut s;

    // r2.push_str(" is very good");

    // println!("{r2}");

      let name = String::from("Rust");
       let st = String::from("Programming");

    let result = identity(&name,&st);

    println!("{name}");
    println!("{result}");

}

// =====================================================
// Ownership Function
// =====================================================

// fn print_name(name: String) {
//     println!("{name}");
// }

// =====================================================
// Borrowing Function
// =====================================================

// fn print_name(name: &String) {
//     println!("{name}");
// }
fn identity<'a>(s: &'a String,st: &'a String) -> &'a String {
    st
}



// fn identity(s: &String) -> &String {
//     s
// }