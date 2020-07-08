pub fn find_string_length(x: String)->(String,usize){
    let length = x.len();
    (x,length)
}

pub fn find_string_length_ref(x: &String)->(String,usize){
    let length = x.len();
    (x.to_string(),length)
}
pub fn print_function(){ // Scope of a print function
    let mut x = String::from("Hello");
    // let x = 30;
    println!("{}",x);
    x.push_str(" World");

    let mut y = &x;
    println!("value of X: {}",x);
    println!("value of Y: {}",y);    
    println!("Hello, world!");
}

