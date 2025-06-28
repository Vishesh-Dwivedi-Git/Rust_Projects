use std::ops::Add;

fn sum<T: Add<Output=T>>(a: T , b :T)-> T {
      a+b
}

fn main() {
    println!("Hello, world!");
    let res= sum(5,6);
    println!("the sum is {}", res);
    let sum1=sum(5.0,6.0);
    println!("the new sum is {}", sum1);
}
