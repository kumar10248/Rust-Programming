fn main(){
let nums = vec![1,2,3,4,5];

let ans = nums
    .iter() // Item = &i32
    .filter(|x| **x % 2 == 0) // x: &i32 -> returns bool
    .map(|x| x * 10) // x: &i32 -> returns i32
    .collect::<Vec<_>>();// Vec<i32>

println!("{:?}", ans);
}