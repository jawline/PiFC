use rustc_serialize::json;
use std::string::{String, ToString};
use fccore::Core;
use std::sync::MutexGuard;
use std::vec::Vec;
use iron::prelude::*;
use std::sync::{Arc, Mutex};
use iron::status;
use iron::mime::Mime;

#[derive(RustcEncodable)]
struct Status {
    pub alive: bool,
    pub armed_switch: bool,
    pub armed_cmd: bool,
    pub armed: bool,
    pub motor_info: Vec<MotorInfo>,
    pub sensor_info: SensorInfo
}

#[derive(RustcEncodable)]
struct MotorInfo {
    pub name: String,
    pub power: usize,
    pub enabled: bool
}

#[derive(RustcEncodable)]
struct SensorInfo {
    pub accelerometer: (f64, f64, f64),
    pub gyroscope: (f64, f64, f64)
}

impl Status {
    fn generate_motor_info(core: &MutexGuard<Core>) -> Vec<MotorInfo> {
        let mut motor_info = Vec::new();

        for motor in core.motors().iter() {
            motor_info.push(MotorInfo{
                name: motor.name.clone(),
                power: motor.current_power(),
                enabled: motor.enabled()
            });
        }

        motor_info
    }

    fn generate_sensor_info(core: &MutexGuard<Core>) -> SensorInfo {
        SensorInfo{
            accelerometer: core.sensors.acc,
            gyroscope: core.sensors.gyro
        }
    }

    pub fn from(core: &MutexGuard<Core>) -> Status {
        Status{
            alive: core.alive,
            armed_switch: core.armed_switch(),
            armed_cmd: core.armed_cmd(),
            armed: core.armed(),
            motor_info: Status::generate_motor_info(core),
            sensor_info: Status::generate_sensor_info(core)
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        json::encode(self).unwrap()
    }
}

pub fn status_report(core_ref :&Arc<Mutex<Core>>) -> Response {
    let json_content_type : Mime = "application/json".parse::<Mime>().unwrap();
    Response::with((json_content_type, status::Ok, Status::from(&core_ref.lock().unwrap()).to_string()))
}