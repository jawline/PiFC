use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use fccore::Core;
use std::thread;
use std::sync::{Arc, Mutex,MutexGuard};
use fccore::motors::MotorID;
use fcwebserve::config::Config;

const TAG : &'static str = "webserve";

fn unknown() -> IronResult<Response> {
    Ok(Response::with((status::NotFound, "unknown command")))
}

fn generate_armed_info(core: &MutexGuard<Core>) -> String {
    let mut info = format!("<b>ARM INFO</b><br/>");
    info = info + &format!("ARM_SAFETY: {}<br/>", core.armed_switch());
    info = info + &format!("ARM_COMMAND: {}<br/>", core.armed_cmd());
    info = info + &format!("FULLY ARMED: {}<br/>", core.armed());
    info
}

fn generate_motor_info(core: &MutexGuard<Core>) -> String {
    let mut info = format!("<b>MOTOR INFO</b><br/>");

    for motor in core.motors().iter() {
        info = info + &format!("MOTOR {}: {}<br/>", motor.name, motor.current_power());
    }
    
    info
}

fn generate_sensor_info(core: &MutexGuard<Core>) -> String {
    let mut info = format!("<b>SENSOR INFO</b><br/>");
    
    //Generate accelerometer and gyroscope data
    let (acc_x, acc_y, acc_z) = core.sensors.acc;
    let (gyr_x, gyr_y, gyr_z) = core.sensors.gyro;
    info = info + &format!("ACC: ({}, {}, {})<br/>GYR: ({}, {}, {})<br/>", acc_x, acc_y, acc_z, gyr_x, gyr_y, gyr_z);

    info
}

fn generate_status_links() -> String {
    let mut info = format!("<a href=\"./log\">Log</a><br/>");
    info = info + &format!("<a href=\"./arm\">Arm</a><br/>");
    info = info + &format!("<a href=\"./disarm\">Disarm</a><br/>");
    info = info + &format!("<a href=\"./config\">Config</a><br/>");
    info = info + &format!("<a href=\"./motor_test\">Motor Test</a><br/>");
    info = info + &format!("<a href=\"./kill\">Kill</a><br/>");
    info
}

fn status_report(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving status request");
    
    //Generate header
    let boiler_start = format!("<html><head><title>Status</title><body>");
    let header = "<b>STATUS PAGE</b><br/>";
    
    //Generate alive data
    let status_portion = format!("ALIVE: {}<br/><br/>", core.alive);
    
    let arm_portion = format!("{}<br/>", generate_armed_info(&core));
    let acc_portion = format!("{}<br/>", generate_sensor_info(&core));
    let motor_portion = format!("{}<br/>", generate_motor_info(&core));
    
    let status_links = generate_status_links();
    
    //Generate footer
    let boiler_end = format!("</body></html>");
    
    //Generate HTML mime type to send
    let html_content_type : Mime = "text/html".parse::<Mime>().unwrap();
    
    Ok(Response::with((html_content_type, status::Ok, format!("{}{}{}{}{}{}{}{}", boiler_start, header, status_portion, arm_portion, acc_portion, motor_portion, status_links, boiler_end))))
}

fn motor_test(core_ref: &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    
    core.set_motor_power(MotorID::FrontLeft, 25);
    core.set_motor_power(MotorID::FrontRight, 25);
    core.set_motor_power(MotorID::BackLeft, 25);
    core.set_motor_power(MotorID::BackRight, 25);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 50);
    core.set_motor_power(MotorID::FrontRight, 50);
    core.set_motor_power(MotorID::BackLeft, 50);
    core.set_motor_power(MotorID::BackRight, 50);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 75);
    core.set_motor_power(MotorID::FrontRight, 75);
    core.set_motor_power(MotorID::BackLeft, 75);
    core.set_motor_power(MotorID::BackRight, 75);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 100);
    core.set_motor_power(MotorID::FrontRight, 100);
    core.set_motor_power(MotorID::BackLeft, 100);
    core.set_motor_power(MotorID::BackRight, 100);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 0);
    core.set_motor_power(MotorID::FrontRight, 0);
    core.set_motor_power(MotorID::BackLeft, 0);
    core.set_motor_power(MotorID::BackRight, 0);
    thread::sleep_ms(0);

    Ok(Response::with((status::Ok, "ok")))
}

fn get_log(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let core = core_ref.lock().unwrap();
    Ok(Response::with((status::Ok, core.log().to_string())))
}

fn get_config(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving get config request");
    Ok(Response::with((status::Ok, core.config().to_string())))
}

fn arm_core(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.set_armed_command(true);
    Ok(Response::with((status::Ok, "ok")))
}

fn kill_core(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.alive = false;
    Ok(Response::with((status::Ok, "ok")))
}

fn disarm_core(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "disarm core network request");
    core.set_armed_command(false);
    Ok(Response::with((status::Ok, "ok")))
}

fn page_handler(req : &mut Request, core : &Arc<Mutex<Core>>) -> IronResult<Response> {    	
    
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

fn start_webserve_thread(core : Arc<Mutex<Core>>, config: &Config) {
    thread::spawn(move || {
        let webserve_addr = &config.webserve_address as &str;
        core.lock().unwrap().log_mut().add(TAG, &format!("Starting webserve on {}", webserve_addr)));
        Iron::new(move |req: &mut Request| {
            page_handler(req, &core)
        }).http(webserve_addr).unwrap();
    });
}

pub fn spawn(core : &Arc<Mutex<Core>>, config_path: &str) {
    let webserve_config = Config::new(config_path);
    if webserve_config.enabled {
        start_webserve_thread(core.clone(), &webserve_config);
    } else {
        core.lock().unwrap().log_mut().add(TAG, "Log disabled by configuration file");
    }
}
