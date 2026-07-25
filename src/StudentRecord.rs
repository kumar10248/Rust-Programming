mod add;
struct Student{
    roll_no:String,
    name:String,
    age:u8,
}
impl Student {
  
    fn display(&self){
       
    println!("Student name is: {} ,roll no is {}, age is {}",
    self.name,
    self.roll_no,
    self.age, 
);
    }

}
fn get_index(
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

fn search_student_rollno<'a>(
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


fn delete_student(
    students:&mut Vec<Student>,
    roll:&str
)->Result<(), String>{

if let Some(index) = get_index(students,roll) {
    students.remove(index);
    return Ok(());
}
 Err(String::from("Student Details not found"))
}

fn update_student(
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

fn add_student(student:&mut Vec<Student> ){
    let nm=add::read_string("Enter your Name");
    let rn=add::read_string("Enter your Roll No.");
    let age=add::read_int("Enter your Age");
    let stud=Student{
        roll_no:rn,
        name:nm,
        age: age as u8
    };

    student.push(stud);
    println!("Student Data Entered successfully");

}

fn main(){
    let mut students = Vec::<Student>::new();
    loop{
    println!("Welcome to Student Management System!");
    println!("1. Add Student Details");
    println!("2. Display all Student Details");
    println!("3. search Student Details");
    println!("4. delete Student Details");
    println!("5. update Student Details");
    println!("6. Exit");

    let choice=add::read_int("Enter Your Choice");

    match choice{
        1 => add_student(&mut students),
        2 => {
            for student in &students{

                student.display();
            }
        }
        3 => {
           let rn=add::read_string("Enter your Roll No.");

            let stud=search_student_rollno(&students,&rn);
            match stud{
                Ok(student)=> student.display(),
                Err(err)=>println!("{err}"),
            }
        }
       4 => {
           let rn=add::read_string("Enter your Roll No.");

       let dl= delete_student(&mut students,&rn);
         match dl{
           Ok(())=>  println!("Student Details Deleted successfully of roll no. {rn}"),
             
         Err(err)=>println!("{err}"),
            }
          
       }
       5 => {
           let rn=add::read_string("Enter your Roll No.");
           let up= update_student(&mut students, &rn);
          
         match up{
           Ok(())=>  println!("Student Details Updated successfully of roll no. {rn}"),
             
         Err(err)=>println!("{err}"),
            }

       }
       6 => break,
       _ => println!("please enter valid choice"),

    }


    }
    
}