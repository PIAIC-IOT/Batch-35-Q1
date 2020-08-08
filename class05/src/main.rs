// pub mod ST {
//     use std::io;
//     #[derive(Debug)]
//     pub struct server_temp{
//         pub cpu_temp: i32,
//         pub gpu_temp: i32,
//         pub disk_temp: i32,
//         pub chassis_temp: i32, 
//     }

//     pub fn server_temp_create(cpu:i32,gpu:i32,
//         disk:i32,chassis:i32)->server_temp{
//         server_temp{
//             cpu_temp:cpu,
//             gpu_temp:gpu,
//             disk_temp:disk,
//             chassis_temp: chassis,
//         }
//     }

//     pub fn server_temp_create_short(cpu_temp:i32,gpu_temp:i32,
//         disk_temp:i32,chassis_temp:i32)->server_temp{
//         server_temp {
//             cpu_temp,
//             gpu_temp,
//             disk_temp,
//             chassis_temp,
//         }
//     }

//     pub fn average_server_temp(server:server_temp) -> i32 {
//         let total_temp = server.cpu_temp + server.gpu_temp + server.disk_temp + server.chassis_temp;
//         let average = total_temp/4;
//         average
//     }


//     pub fn input_fn_integer()->i32{
//         let mut input = String::new();
//         println!("Please enter the input");
//         io::stdin().read_line(&mut input);
//         let input: i32 = input.trim().parse().unwrap(); // For parsing
//         input
//     }
//     pub mod Input {
//         use std::io;
//         pub fn read_line()->i32{
//             let mut input = String::new();
//             println!("Please enter the input");
//             io::stdin().read_line(&mut input);
//             let input: i32 = input.trim().parse().unwrap(); // For parsing
//             input
//         }
//     }
// }


// fn main() {
//     let cpu_temp = ST::Input::read_line();
//     let gpu_temp = ST::input_fn_integer();
//     let disk_temp = ST::input_fn_integer();
//     let chassis_temp = ST::input_fn_integer();
//     let server_01 = ST::server_temp_create_short(cpu_temp,
//                             gpu_temp,disk_temp,chassis_temp);
//     println!("The server average temp is this :{}",
//                 ST::average_server_temp(server_01));
// }

// #[warn(unused_must_use)]
// use iotio::*;
// fn main(){
//     let input = read_inputs::read();
//     println!("The result is this {}",input);
//     let input02 = read_inputs::read_with_length();
//     println!("The result is this {},{}",input02.0,input02.1);
// }


// // Third Party Package Implmentation
// use text_io::read;
// fn main(){
//     let a :i32 = read!();
//     let b :i32 = read!();
//     println!("The sum of {} and {} is {}", a,b,a+b);
// }


// pub mod ST {
//     use std::io;
//     #[derive(Debug)]
//     pub struct server_temp{
//         pub cpu_temp: i32,
//         pub gpu_temp: i32,
//         pub disk_temp: i32,
//         pub chassis_temp: i32, 
//     }

//     pub fn server_temp_create(cpu:i32,gpu:i32,
//         disk:i32,chassis:i32)->server_temp{
//         server_temp{
//             cpu_temp:cpu,
//             gpu_temp:gpu,
//             disk_temp:disk,
//             chassis_temp: chassis,
//         }
//     }

//     pub fn server_temp_create_short(cpu_temp:i32,gpu_temp:i32,
//         disk_temp:i32,chassis_temp:i32)->server_temp{
//         server_temp {
//             cpu_temp,
//             gpu_temp,
//             disk_temp,
//             chassis_temp,
//         }
//     }

//     pub fn average_server_temp(server:server_temp) -> i32 {
//         let total_temp = server.cpu_temp + server.gpu_temp + server.disk_temp + server.chassis_temp;
//         let average = total_temp/4;
//         average
//     }

//     pub fn input_fn_integer()->i32{
//         let mut input = String::new();
//         println!("Please enter the input");
//         io::stdin().read_line(&mut input);
//         let input: i32 = input.trim().parse().unwrap(); // For parsing
//         input
//     }

//     pub mod Input {
//         use std::io;
//         pub fn read_line()->i32{
//             let mut input = String::new();
//             println!("Please enter the input");
//             io::stdin().read_line(&mut input);
//             let input: i32 = input.trim().parse().unwrap(); // For parsing
//             input
//         }
//     }

//     pub fn add_intergers(){
//         let mut input_01 = String::new();
//         println!("Please enter the input");
//         io::stdin().read_line(&mut input_01);
//         let input_01: i32 = input_01.trim().parse().unwrap(); // For parsing

//         let mut input_02 = String::new();
//         println!("Please enter the input");
//         io::stdin().read_line(&mut input_02);
//         let input_02: i32 = input_02.trim().parse().unwrap(); // For parsing

//         super::add(input_01, input_02);        
//     }
// }


// fn main() {
//         // Relative Path in Crate
//     let cpu_temp = ST::Input::read_line();
//         // Absulote Path in Crate
//     let gpu_temp = crate::ST::input_fn_integer();
//     let disk_temp = ST::input_fn_integer();
//     let chassis_temp = ST::input_fn_integer();
//     let server_01 = ST::server_temp_create_short(cpu_temp,
//                             gpu_temp,disk_temp,chassis_temp);
//     println!("The server average temp is this :{}",
//                 ST::average_server_temp(server_01));
//     ST::add_intergers();                
// }

// fn add (x:i32,y:i32){
//     println!("The sum is: {}",x+y);
// }



pub mod level_01{
    fn add_level_01 (x:i32,y:i32){
      println!("The sum is: {}",x+y);
    }

    pub mod level_02{
        pub mod level_03{
            pub mod level_04{
                pub fn add_level_04(){
                    super::super::super::super::add(4, 5);
                }
            }
        }        
    }
}
// use std::{clone,char,io};
// use std::{io,clone::*};
// use std::io;
// use std::clone;

fn main(){
    level_01::level_02::level_03::level_04::add_level_04();
    add(3,4);
}
fn add(x:i32,y:i32){
    println!("The sum is: {}",x+y);
}