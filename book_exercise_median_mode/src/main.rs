#![allow(unused)]
fn intergers_median(vec:&mut Vec<i32>) -> i32{
    let mut median =0;
    vec.sort_unstable();
    let counter = vec.len();
      if counter % 2 == 0 {
        median = (vec[counter/2-1] + vec[counter/2])/2; 
      }
      else {
        median = vec[(counter-1)/2];  
      }
      median
}
fn intergers_mode(vec:&mut Vec<i32>){
    use std::collections::HashMap;
    let mut intergers: HashMap<&i32, i32> = HashMap::new();
    let mut max_interger = 1;
    for interger in vec.iter() {
        intergers.entry(interger).and_modify(|counter| *counter += 1).or_insert(1);
    }
    for val in intergers.values() {
        if val > &max_interger {
            max_interger = *val;
        }
    }
    intergers.retain(|_, v| *v == max_interger);
    for (k, val) in intergers.iter() {
        println!("the most ofen key is {k}");
     }
}
fn main() {
let mut vec_interger = vec![8, 2, 0, 9, 4, 6, 5, 1, 5, 11];
println!("the median interger is {}",intergers_median(&mut vec_interger));
intergers_mode(&mut vec_interger);
}
