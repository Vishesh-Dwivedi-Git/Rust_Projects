
//lifetimes - 2 use cases when u return a reference from a function
//  and when you have a struct that has a reference in it

//borsh - a  framework use to serialize and deserialize data structures in Rust. especially useful for blockchain applications.
// bincode - a binary encoding format for Rust data structures, often used for efficient serialization.
// serde - a framework for serializing and deserializing Rust data structures. converts data structures to and from various formats, including JSON, YAML, and binary formats.
use borsh::{BorshDeserialize, BorshSerialize};
use borsh_derive::{BorshDeserialize, BorshSerialize}; // ⬅️ this brings in the derive macros

#[derive(BorshSerialize, BorshDeserialize)]  

struct User { 
    name:String,
    age:u8,
}


fn main() {
   let name=String::from("Alice");
   let mut vec= vec![];
   //ser and der
   name.serialize(&mut vec ).unwrap();
   println!("Serialized name: {:?}", vec);
   let name2=String::deserialize(&mut &vec[..]).unwrap();
   println!("Name:{}", name2);
   let name3=String::from("vishesh");
   {
    let str=get_string(&name, &name3).to_string() ;

   }
   println!("StringL{}",str);
}



fn get_string<'a>(s1: &'a String, s2:&'a String)-> &'a String {
     if s1.len()>s2.len() {
         return s1;
     }
     else {
        return s2;
     }
}