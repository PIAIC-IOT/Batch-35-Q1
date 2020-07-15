//// Example 01
// #[derive(Debug)]
// struct Car{
//     Name: String,
//     Color: String,
//     EngineCapacity: i32,
// }



// impl Car{
//     fn new(Name:String,Color:String,EngineCapacity:i32)-> Car{
//         Car{
//             Name,
//             Color,
//             EngineCapacity,
//         }
//     } 
    
//     fn view(&self,wheels:i32){
//         println!("Car Name: {}",self.Name);
//         println!("Car Color: {}",self.Color);
//         println!("Car Engine Capacity: {}",self.EngineCapacity);
//         println!("Car Wheels: {}",wheels);
//     }
// }



// fn main() {

//    let car_01 = Car::new("Corolla".to_string(),String::from("Red"),1300);
// //    car_01.view(6);
// //    Car::view(&car_01,6);
// //    println!("{:?}",);
// //    let car_02 = Car::new
// //    let car_03 = Car::new
    

// }



// fn main(){

//     let number = 4;
//     match number {
//       4 => println!("The number is Four"),  
//       6 => println!("The number is Six"),
//       _ => println!("The number is not Four and Six"),  
//     }

//     if (number == 4){
//         println!("The number is Four");
//     }
//     else if (number == 6){
//         println!("The number is Six");
//     }
//     else{
//         println!("The number is not Four and Six");
//     }
// }


// //// Example 02
// #[derive(Debug)]
// enum EngineCapacity{
//     EightHundred,
//     Thoudsand,
//     ThirdteenHundred,
// }


// #[derive(Debug)]
// struct Car{
//     Name: String,
//     Color: String,
//     EngineCapacity: EngineCapacity,
// }



// impl Car{
//     fn new(Name:String,Color:String,EngineCapacity:EngineCapacity)-> Car{
//         Car{
//             Name,
//             Color,
//             EngineCapacity,
//         }
//     } 
    
//     fn view(&self){
//         println!("Car Name: {}",self.Name);
//         println!("Car Color: {}",self.Color);
//         println!("Car Engine Capacity: {:?}",
//             match self.EngineCapacity{
//                 EngineCapacity::EightHundred => "Eight Hundred CC",
//                 // EngineCapacity::Thoudsand => "Thoudsand CC",  
//                 // EngineCapacity::ThirdteenHundred => "Thirdteen Hundred CC",  
//                 _ => "Invalid Engine Capacity",
//            }
//         );
//     }
// }

// fn main() {

//    let car_01 = Car::new("Corolla".to_string(),String::from("Red"),EngineCapacity::ThirdteenHundred);
//     // println!("{:?}",car_01);
//     car_01.view();
// //    car_01.view(6);
// //    Car::view(&car_01,6);

// //    let car_02 = Car::new
// //    let car_03 = Car::new
    
// }