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

    fn area(&self)->f64{
        
        PI*self.r*self.r
    }
}



fn print_area<T: Shape>(shape: &T) {

shape.describe();
println!("Area is: {}",shape.area());
}   

fn main(){
    let rect=Rectangle{
        l:10.0,
        b:20.0,
    };
   let circ=Circle{
    r:10.0,
   };
    print_area(&rect);
    print_area(&circ);
}