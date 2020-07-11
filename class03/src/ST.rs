use std::io;
#[derive(Debug)]
pub struct server_temp{
    pub cpu_temp: i32,
    pub gpu_temp: i32,
    pub disk_temp: i32,
    pub chassis_temp: i32, 
}

pub fn server_temp_create(cpu:i32,gpu:i32,
    disk:i32,chassis:i32)->server_temp{
    server_temp{
        cpu_temp:cpu,
        gpu_temp:gpu,
        disk_temp:disk,
        chassis_temp: chassis,
    }
}

pub fn server_temp_create_short(cpu_temp:i32,gpu_temp:i32,
    disk_temp:i32,chassis_temp:i32)->server_temp{
    server_temp {
        cpu_temp,
        gpu_temp,
        disk_temp,
        chassis_temp,
    }
}

pub fn average_server_temp(server:server_temp) -> i32 {
    let total_temp = server.cpu_temp + server.gpu_temp + server.disk_temp + server.chassis_temp;
    let average = total_temp/4;
    average
}


pub fn input_fn_integer()->i32{
    let mut input = String::new();
    println!("Please enter the input");
    io::stdin().read_line(&mut input);
    let input: i32 = input.trim().parse().unwrap(); // For parsing
    input
}