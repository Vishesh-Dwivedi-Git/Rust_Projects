fn main() {

    let name = String :: from( "Vishesh");
    println!("Hello, world!");
    let (name,len)= get_length(name);
    println!("length of the String id : {}", len);
    println!("The name is : {}", name);
    
}

fn get_length(name1:String)-> (String,usize) {
    let len= name1.len();
     return (name1,len);
}
