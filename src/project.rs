struct Student{
    name:String,
    uid:String,
    age:u8,
    cgpa:f32,

}



impl Student {
    // methods go here
        fn change_cgpa(&mut self){
       self.cgpa+=0.1;
        }

    fn display(&self){
       
    println!("Student name is: {} ,uid is {}, age is {} and CGPA is {}",
    self.name,
    self.uid,
    self.age, 
    self.cgpa
);


    }
    fn search_student<'a>(
        students:&'a Vec<Student>,
        name:&str,
    )-> Option<&'a  Student>{

        for student in students{
            if student.name==name{
                return Some(student);
            }
        }
        None
    }

    fn search_student_byuid<'a>(
        students:&'a Vec<Student>,
        uid:&str,
    )->Result<&'a Student,String>{
 for student in students{
            if student.uid==uid{
                return Ok(student);
            }
        }
        Err(String::from("Student Details not found"))
    }
}

fn main(){
    let mut students = Vec::new();

let stud1=Student{
    name:String::from("Devashish"),
    uid:String::from("22BCS10248"),
    age:22,
    cgpa:7.90,
};

let stud2=Student{
    name:String::from("Harsha"),
    uid:String::from("22BCS10131"),
    age:21,
    cgpa:8.70,
};

let stud3=Student{
    name:String::from("Saniya"),
    uid:String::from("22BCS10177"),
    age:22,
    cgpa:8.0,
};

let stud4=Student{
    name:String::from("Diya"),
    uid:String::from("22BCS10472"),
    age:22,
    cgpa:7.6,
};

let stud5=Student{
    name:String::from("Pradeep"),
    uid:String::from("22BCS10284"),
    age:22,
    cgpa:8.7,
};

let stud6=Student{
    name:String::from("Khushi"),
    uid:String::from("22BCS10468"),
    age:22,
    cgpa:8.0,
};

students.push(stud1);
students.push(stud2);
students.push(stud3);
students.push(stud4);
students.push(stud5);
students.push(stud6);




for student in &mut students {
    student.change_cgpa();
}

// for student in &students {
//    student.display();
// }

let name=String::from("Saniya");
let uid=String::from("22BCS10472");
let result1=Student::search_student(&students,&name);
let result2=Student::search_student_byuid(&students,&uid);

match result1{
    Some(student)=>{
        println!("Found!!!🥳🥳🥳 ");
        student.display();
    }
    None =>{
         println!("Student not found with name {}",name);
    }
}


match result2{
    Ok(student)=>{
        println!("Found!!!🥳🥳🥳 ");
        student.display();
    }
    Err(err) =>{
         println!("{}",err);
    }
}




}