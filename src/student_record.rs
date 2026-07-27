#![allow(dead_code)]
use crate::add;
use std::fs::File;
use std::io::{BufRead, BufReader,Write};
pub struct Student{
   pub roll_no:String,
   pub   name:String,
   pub age:u8,
}
impl Student {
  
pub fn display(&self) {
        println!("{:<20} {:<30} {:<3}", 
            self.roll_no, 
            self.name, 
            self.age);
    }

}
pub fn get_index(
     students:&Vec<Student>,
    roll:&str
)->Option<usize>{


for (index, student) in students.iter().enumerate() {
    if student.roll_no == roll {
        return Some(index);
    }
}
None
}

pub fn load_students() -> std::io::Result<Vec<Student>>{
  let file=File::open("students.txt")?;
    let reader=BufReader::new(file);
    let mut students=Vec::new();

    for (_index,line ) in reader.lines().enumerate(){
        let line=line?;
        let parts:Vec<&str>=line.split(',').collect();
        if parts.len()==3{
            let student=Student{
                roll_no:parts[0].to_string(),
                name:parts[1].to_string(),
                age:parts[2].parse::<u8>().unwrap_or(0),
            };
            students.push(student); 
        }   


}
    Ok(students)



}

pub fn save_students(students: &[Student]) -> std::io::Result<()> {
   

    let mut file =File::create("students.txt")?;

    for student in students {
        
        writeln!(file, "{},{},{}", student.roll_no, student.name, student.age)?;
    }

    Ok(())
}

pub fn search_student_rollno<'a>(
        students:&'a Vec<Student>,
        roll:&str,
    )->Result<&'a Student,String>{
 for student in students{
            if student.roll_no==roll{
                return Ok(student);
            }
        }
        Err(String::from("Student Details not found"))
    }


pub fn delete_student(
    students:&mut Vec<Student>,
    roll:&str
)->Result<(), String>{

if let Some(index) = get_index(students,roll) {
    students.remove(index);
    return Ok(());
}
 Err(String::from("Student Details not found"))
}

pub fn update_student(
    students:&mut Vec<Student>,
    roll:&str
)->Result<(), String>{


if let Some(index) = get_index(students,roll) {
  let stud = &mut students[index];
    stud.name = add::read_string("Enter your new Name");
    stud.age = add::read_int("Enter your Age") as u8;
    return Ok(());

}
 Err(String::from("Student Details not found"))

}

pub fn add_student(student:&mut Vec<Student> ){

  
    let rn=add::read_string("Enter your Roll No.");
        for stud in student.iter(){
        if stud.roll_no==rn{
            println!("Student Details already exists with roll no. {rn}");
            return;
        }
    }
    let nm=add::read_string("Enter your Name");
    let age=add::read_int("Enter your Age");


    let stud=Student{
        roll_no:rn,
        name:nm,
        age: age as u8
    };

    student.push(stud);
      

    println!("Student Data Entered successfully");

}

// fn main(){
//     let mut students = Vec::<Student>::new();
//     println!("Loading Student Details from file...");
//     match load_students(){
//             Ok(loaded_students)=>{
//               let  studs = loaded_students;
//                 students = studs; // Update the main students vector with the loaded data
//                 println!("Student Details Loaded successfully from file");
//             }
//             Err(err)=> println!("Error loading student details: {}", err),
//         }

//     loop{
//     println!("Welcome to Student Management System!");

//     println!("1. Add Student Details");
//     println!("2. Display all Student Details");
//     println!("3. search Student Details");
//     println!("4. delete Student Details");
//     println!("5. update Student Details");

//     println!("6. Exit");

//     let choice=add::read_int("Enter Your Choice");

//     match choice{
//         1 => add_student(&mut students),
//         2 => {
//             println!("----------------------------------------------------------");
//             println!("Roll No.             Name                           Age ");
      
//             for student in &students{

//                 student.display();
//             }
//             println!("----------------------------------------------------------");

//         }
//         3 => {
//            let rn=add::read_string("Enter your Roll No.");

//             let stud=search_student_rollno(&students,&rn);
//             match stud{
//                 Ok(student)=> student.display(),
//                 Err(err)=>println!("{err}"),
//             }
//         }
//        4 => {
//            let rn=add::read_string("Enter your Roll No.");

//        let dl= delete_student(&mut students,&rn);
//          match dl{
//            Ok(())=>  println!("Student Details Deleted successfully of roll no. {rn}"),
             
//          Err(err)=>println!("{err}"),
//             }
          
//        }
//        5 => {
//            let rn=add::read_string("Enter your Roll No.");
//            let up= update_student(&mut students, &rn);
          
//          match up{
//            Ok(())=>  println!("Student Details Updated successfully of roll no. {rn}"),
             
//          Err(err)=>println!("{err}"),
//             }

//        }

  
//        6 => {
//         println!("Saving Student Details to file...");
//               match save_students(&students){
//             Ok(())=> println!("Student Details Saved successfully to file"),
//             Err(err)=> println!("Error saving student details: {}", err),
//         }
//         println!("Exiting the program...");
//         break;
//     }
//        _ => println!("please enter valid choice"),

//     }


//     }
    
// }