use fccore::FCCore;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, sleep_ms, JoinHandle};

pub fn start_logic_thread(core : &Arc<Mutex<FCCore>>) -> JoinHandle<()> {
	let thread_core = core.clone();
	return spawn(move || {
      fccore_thread_loop(thread_core);
    });
}

fn fccore_thread_loop(core_ref : Arc<Mutex<FCCore>>) {
    while core_ref.lock().unwrap().alive() {
      sleep_ms(50);
      core_ref.lock().unwrap().update_sensors();
    }
}
