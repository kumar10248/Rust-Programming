struct Student{
    name:String,
    age:u8,
    cgpa:f32,

}



impl Student {
    // methods go here
        fn change_cgpa(&mut self){
       self.cgpa+=0.1;
        }

    fn display(&self){
       
    println!("Student name is: {} , age is {} and CGPA is {}",
    self.name,
    self.age, 
    self.cgpa
);


    }
}

fn main(){
    let mut students = Vec::new();

let stud1=Student{
    name:String::from("Devashish"),
    age:22,
    cgpa:7.90,
};

let stud2=Student{
    name:String::from("Harsha"),
    age:21,
    cgpa:8.70,
};
students.push(stud1);
students.push(stud2);

for student in &mut students {
    student.change_cgpa();
}

for student in &students {
   student.display();
}





}