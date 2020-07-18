pub mod read_inputs{
 use std::io;
 pub fn read() -> String{
     let mut input =String::new();
     io::stdin().read_line(&mut input);
     input
 }
 pub fn read_with_length() -> (String,usize){
    let mut input =String::new();
    io::stdin().read_line(&mut input);
    let input:String = input.trim().parse().unwrap();
    let length = input.len();
    (input,length)
 }   
}