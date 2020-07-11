mod ST;

// fn main() {
//    let server_01 = ST::server_temp{
//        cpu_temp: 40,
//        gpu_temp: 32,
//        disk_temp: 35,
//        chassis_temp: 33,
//    };
//    let server_02 = ST::server_temp{
//     cpu_temp: 55,
//     gpu_temp: 50,
//     disk_temp: 50,
//     chassis_temp: 40,
//    };
//    let server_03 = ST::server_temp_create(36,34,40,35);
//    let server_04 = ST::server_temp_create_short(38,33,46,33);
//    println!("{:?}",server_01);
//    println!("{:?}",server_02);
//    println!("{:?}",server_03);
//    println!("{:?}",server_04);

//    println!("The Server Average Temp is: {}",ST::average_server_temp(server_01));
//    println!("The Server Average Temp is: {}",ST::average_server_temp(server_02));
//    println!("The Server Average Temp is: {}",ST::average_server_temp(server_03));
//    println!("The Server Average Temp is: {}",ST::average_server_temp(server_04));
// }



// Dynamic User Input with struct

fn main(){
    let cpu_temp = ST::input_fn_integer();
    let gpu_temp = ST::input_fn_integer();
    let disk_temp = ST::input_fn_integer();
    let chassis_temp = ST::input_fn_integer();
    let server_01 = ST::server_temp_create_short(cpu_temp,
                            gpu_temp,disk_temp,chassis_temp);
    println!("The server average temp is this :{}",
                ST::average_server_temp(server_01));
}

