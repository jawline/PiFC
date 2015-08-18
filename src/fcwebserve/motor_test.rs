use iron::prelude::*;
use std::sync::{Arc, Mutex, MutexGuard};
use fccore::Core;
use fccore::motors::MotorID;
use iron::status;
use std::thread;

pub fn motor_test(core_ref: &Arc<Mutex<Core>>) -> Response {
    
    //Keep each step in a minimized scope so that it doesn't block the rest of the system in the thread sleep
    {
        let mut core = core_ref.lock().unwrap();
        core.set_motor_power(MotorID::FrontLeft, 25);
        core.set_motor_power(MotorID::FrontRight, 25);
        core.set_motor_power(MotorID::BackLeft, 25);
        core.set_motor_power(MotorID::BackRight, 25);
    }
    thread::sleep_ms(1000);
    
    {
        let mut core = core_ref.lock().unwrap();
        core.set_motor_power(MotorID::FrontLeft, 50);
        core.set_motor_power(MotorID::FrontRight, 50);
        core.set_motor_power(MotorID::BackLeft, 50);
        core.set_motor_power(MotorID::BackRight, 50);
    }
    thread::sleep_ms(1000);
    
    {
        let mut core = core_ref.lock().unwrap();
        core.set_motor_power(MotorID::FrontLeft, 75);
        core.set_motor_power(MotorID::FrontRight, 75);
        core.set_motor_power(MotorID::BackLeft, 75);
        core.set_motor_power(MotorID::BackRight, 75);
    }
    thread::sleep_ms(1000);
    
    {
        let mut core = core_ref.lock().unwrap();
        core.set_motor_power(MotorID::FrontLeft, 100);
        core.set_motor_power(MotorID::FrontRight, 100);
        core.set_motor_power(MotorID::BackLeft, 100);
        core.set_motor_power(MotorID::BackRight, 100);
    }
    thread::sleep_ms(1000);
    
    {
        let mut core = core_ref.lock().unwrap();
        core.set_motor_power(MotorID::FrontLeft, 0);
        core.set_motor_power(MotorID::FrontRight, 0);
        core.set_motor_power(MotorID::BackLeft, 0);
        core.set_motor_power(MotorID::BackRight, 0);
    }
    thread::sleep_ms(0);

    Response::with((status::Ok, "ok"))
}