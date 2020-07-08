mod printfn;
use std::env;
fn main() { // Scope of a main function
    // printfn::print_function();
    // let x = "Hello".to_string();
    // let data = String::from("Hello World");
    // // let output = find_string_length(data.clone());
    // let output = printfn::find_string_length_ref(&data);
    // println!("{},{}",output.0,output.1);
    // println!("{}",data);

    let value_01 = env::args().nth(1).expect("The Error");
    let value_02 = env::args().nth(2).unwrap();

    println!("The Complete name is this: {} {}",value_01,value_02);
}

