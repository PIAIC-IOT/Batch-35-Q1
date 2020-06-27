fn main() {
  
    let integer = 10;
    let gravity = 9.8;
    let decision = true;
    let character = 'c';


    println!("{},{},{}",gravity,integer,character);

    let different_data = (35,5.8,'c');
    println!("{:?}",different_data);
    println!("{}",different_data.0);

    let same_data = [45,67,87,4,5];
    println!("{:?}",same_data);
    println!("{}",same_data[2]);
    

// Funcation Calling 
    let square_value = my_test_fn(8);
    println!("The square value from my test function is this: {}",square_value);


    let number = 17;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // let mut count = 1;
    // loop {
    //     println!("Print Value!{}",count);
    //     count = count + 1;
    // }

    for x in 0..10{// Outter Loop
        for y in 0..10{ // Inner Loop
            print!("{}{}----",x,y);
        }
    }
    
}




// Funcations are having the four major things. 
// Two are most important with it function would not be run.
// 1=>Funcation Declaration
// 2=>Funcation Calling 
// 3=>Funcations Input Parameters
// 4=>Funcations Return Parameters/ Return Type


// Funcation Declaration
fn my_test_fn(value:i32)->i32{ // value: i32 Input Parameters AND -> i32 called Return Type
    println!("This is my test function value: {}",value);
    let square = value*value;
    square
}