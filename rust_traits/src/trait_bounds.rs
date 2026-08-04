struct Pair<T>{
first:T,
second:T
}
impl<T:std::fmt::Display> Pair<T> {
    fn show(&self){
        println!("{}  {}",
    self.first,
    self.second,
    )
    }
}

fn main(){
    let pn=Pair{
        first:45,
        second:88
    };

        let pf=Pair{
        first:45.8,
        second:88.3
    };
    
        let ps=Pair{
        first:"Kumar".to_string(),
        second:"Devashish".to_string()
    };
pn.show();
pf.show();
ps.show();
    

}