trait Animal{
    fn sound(&self)->&str;
    fn name(&self)->&str;

    fn introduce(&self){
        println!(
            "{} says {}",
            self.name(),
            self.sound()
        )
    }
}

struct Cat;
struct Dog;
struct Cow;

impl Animal for Cat{
    fn sound(&self)->&str{
   "Meow"
    }
    fn name(&self)->&str{
  "Cat"

    }

}

impl Animal for Dog{
    fn sound(&self)->&str{
"Bark"
    }
    fn name(&self)->&str{
      "Dog" 
    }

}

impl Animal for Cow{
    fn sound(&self)->&str{
"Moo"

    }
    fn name(&self)->&str{
      "Cow"
    }

}
#[derive(Debug)]
struct Student {
    roll: u32,
    name: String,
}


fn main() {
  let cat=Cat;
  let dog=Dog;
  let cow=Cow;
cat.introduce();
dog.introduce();
cow.introduce();

let mut numbers = vec![10,20,30,40,50];

let sum:i32=numbers.iter().sum();
println!("Sum:{}",sum);

let sum2:i32=numbers.iter().fold(0,|acc,x| acc+x);
println!("Sum2:{}",sum2);

for n in numbers.iter(){
    print!("{},",n);
}
println!();
for n in numbers.iter_mut(){
    *n+=100;
}
for n in numbers.iter(){
    print!("{},",n);
}
println!();

let numbers2= vec![1,2,3,4,5,6];

let even_vec:Vec<&i32> =numbers2
.iter()
.filter(|n| *n%2==0)
.collect();
println!("{:?}",even_vec);

let c =numbers2
.iter()
.filter(|n| *n%2==0)
.count();

println!("count: {}",c);




let lang=vec!["rust","java","go"];
let upper_lang:Vec<String>=lang
.iter()
.map(|s| s.to_uppercase())
.collect();
println!("{:?}",upper_lang);

let mut students=Vec::new();

let student1=Student{
    roll:1,
    name:"Alice".to_string(),
};
let student2=Student{
    roll:2,
    name:"Bob".to_string(),
};
let student3=Student{
    roll:3,
    name:"Charlie".to_string(),
};
let student4=Student{
    roll:4,
    name:String::from("David"),
};

let student5=Student{
    roll:5,
    name:"Eve".to_string(),
};

students.push(student1);
students.push(student2);
students.push(student3);
students.push(student4);
students.push(student5);

let x=students.iter()
.find(|s| s.roll==3);
println!("{:?}",x);  

let index=students.iter()
.position(|s| s.roll==1);
println!("{:?}",index);

let stud=students.iter()
.any(|s| s.name=="Bob");
println!("{:?}",stud);

}
