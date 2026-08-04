struct Student{
    name:String,
    age:u32
}
fn main(){
    let student=Box::new(Student{
        name:"Devashish".to_string(),
        age:23
    });

    println!("Name: {}", student.name);
    println!("age: {}", student.age);


}