use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use fccore::FCCore;
use std::thread;
use std::sync::{Arc, Mutex};

const TAG : &'static str = "webserve";

fn unknown() -> IronResult<Response> {
    Ok(Response::with((status::NotFound, "unknown command")))
}

fn status_report(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving status request");
    
    //Generate header
    let boiler_start = format!("<html><head><title>Status</title><body>");
    let header = "<b>STATUS PAGE</b><br/>";
    
    //Generate alive data
    let status_portion = format!("ALIVE: {}<br/>", core.alive);
    
    //Generate accelerometer and gyroscope data
    let (acc_x, acc_y, acc_z) = core.sensors.acc;
    let (gyr_x, gyr_y, gyr_z) = core.sensors.gyro;
    let acc_portion = format!("ACC: ({}, {}, {})<br/>GYR: ({}, {}, {})<br/>", acc_x, acc_y, acc_z, gyr_x, gyr_y, gyr_z);
    
    //Generate motor data
    let motor1_power = core.motors.motor_1.current_power();
    let motor2_power = core.motors.motor_2.current_power();
    let motor3_power = core.motors.motor_3.current_power();
    let motor4_power = core.motors.motor_4.current_power();
    let motor_portion = format!("MOTOR 1: {}<br/>MOTOR 2: {}<br/>MOTOR 3: {}<br/>MOTOR 4: {}<br/>",
            motor1_power,
            motor2_power,
            motor3_power,
            motor4_power);
    
    //Generate arm data
    let arm_portion = format!("ARM_SAFETY: {}<br/>ARM_COMMAND: {}<br/>FULLY ARMED: {}<br/>", core.armed_switch(), core.armed_cmd(), core.armed());

    //Generate footer
    let boiler_end = format!("</body></html>");
    
    //Generate HTML mime type to send
    let html_content_type : Mime = "text/html".parse::<Mime>().unwrap();
    
    Ok(Response::with((html_content_type, status::Ok, format!("{}{}{}{}{}{}{}", boiler_start, header, status_portion, acc_portion, motor_portion, arm_portion, boiler_end))))
}

fn motor_test(core_ref: &Arc<Mutex<FCCore>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    let log = core.log_mut();
    
    core.motors.motor_1.set_power(25, log);
    thread::sleep_ms(250);
    core.motors.motor_1.set_power(75, log);
    thread::sleep_ms(250);
    core.motors.motor_1.set_power(100, log);
    thread::sleep_ms(250);
    core.motors.motor_1.set_power(0, log);
    Ok(Response::with((status::Ok, "ok")))
}

fn get_log(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
    let core = core_ref.lock().unwrap();
    Ok(Response::with((status::Ok, core.log().to_string())))
}

fn get_config(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving get config request");
    Ok(Response::with((status::Ok, core.config().to_string())))
}

fn arm_core(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.set_armed_command(true);
    Ok(Response::with((status::Ok, "ok")))
}

fn kill_core(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.alive = false;
    Ok(Response::with((status::Ok, "ok")))
}

fn disarm_core(core_ref : &Arc<Mutex<FCCore>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "disarm core network request");
    core.set_armed_command(false);
    Ok(Response::with((status::Ok, "ok")))
}

fn page_handler(req : &mut Request, core : &Arc<Mutex<FCCore>>) -> IronResult<Response> {    	
    
    let mut full_req_path = String::new();
  
    for item in &req.url.path {
        full_req_path = full_req_path + "/" + item;
    }
  
    core.lock().unwrap().log_mut().add(TAG, &format!("Request: {}", full_req_path));
    
    if req.url.path.len() != 0 {
        let base_cmd : &str = &req.url.path[0].clone();
        match base_cmd {
         "arm" => arm_core(core),
         "disarm" => disarm_core(core),
         "log" => get_log(core),
         "kill" => kill_core(core),
         "config" => get_config(core),
         "motor_test" => motor_test(core),
         "status" | _ => status_report(core)
        }
    } else {
        unknown()
    }
}

pub fn spawn(core : &Arc<Mutex<FCCore>>) {
    let webserve_core = core.clone();
    thread::spawn(move || {
        let webserve_addr_str : &str = &format!("localhost:{}", webserve_core.lock().unwrap().config().fc_webserve_port);
        webserve_core.lock().unwrap().log_mut().add(TAG, &format!("Starting webserve on {}", webserve_addr_str));
        Iron::new(move |req: &mut Request| {
            page_handler(req, &webserve_core)
        }).http(webserve_addr_str).unwrap();
    });
}
