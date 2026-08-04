use std::f64::consts::PI;
trait Shape{
    fn area(&self)->f64;
    fn describe(&self){
        println!("I am a shape");
    }

}

struct Rectangle{
l:f64,
b:f64,
  
}

struct Circle{
    r:f64,
}

impl Shape for Rectangle{
    fn describe(&self){
        println!("I am a rectangle");
    }
fn area(&self)->f64{
    
    self.l*self.b  
}
}
impl Shape for Circle{
fn describe(&self){
        println!("I am a Circle");
    }
    fn area(&self)->f64{
        
        PI*self.r*self.r
    }
}



// fn print_area<T: Shape>(shape: &T) {

// shape.describe();
// println!("Area is: {:}",shape.area());
// }   

fn main(){
    let shapes:Vec<Box<dyn Shape>>=vec![
    Box::new(Rectangle{
        l:10.0,
        b:20.0,
    }),
   Box::new(Circle{
    r:10.0,
   }),
       Box::new(Rectangle{
        l:4.0,
        b:5.0,
    }),

    ];
for sh in &shapes{
   sh.describe();
    
    println!("Area: {:.2}",sh.area());
};
}